use std::borrow::Cow;
use proc_macro2::TokenStream;
use quote::quote;
use crate::node_types::{Children, NodeName, NodeType, NodeTypeKind};
use crate::mk_syntax::{ident, lit_str};

impl NodeType {
    pub fn print(&self) -> TokenStream {
        let NodeName { rust_type_name, sexp_name, is_implicit } = &self.name;
        let ident = ident!(rust_type_name, "node kind converted into a rust type name (this is a library error, please report)");
        let sexp_name_literal = lit_str(sexp_name);
        // let named = self.named;
        match &self.kind {
            NodeTypeKind::Supertype { subtypes } => {
                if !is_implicit {
                    panic!("Node types with subtypes must be implicit (start with \"_\"): _{}", sexp_name)
                }

                let variants = subtypes.iter().map(NodeName::print_variant);
                let from_cases = subtypes.iter().map(NodeName::print_from_case);
                let node_cases = subtypes.iter().map(NodeName::print_node_case);
                quote! {
                    #[doc = concat!("Typed node `", #sexp_name, "`")]
                    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
                    #[allow(non_camel_case_types)]
                    pub enum #ident<'tree> {
                        #(#variants)*
                    }

                    #[automatically_derived]
                    impl<'tree> TryFrom<tree_sitter::Node<'tree>> for #ident<'tree> {
                        type Error = type_sitter_lib::IncorrectKind<'tree>;

                        #[inline]
                        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
                            match node.kind() {
                                #(#from_cases)*
                                _ => Err(type_sitter_lib::IncorrectKind {
                                    node,
                                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                                })
                            }
                        }
                    }

                    #[automatically_derived]
                    impl<'tree> type_sitter_lib::TypedNode<'tree> for #ident<'tree> {
                        const KIND: &'static str = #sexp_name_literal;

                        #[inline]
                        fn node(&self) -> &tree_sitter::Node<'tree> {
                            match self {
                                #(#node_cases)*
                            }
                        }
                    }
                }
            }
            NodeTypeKind::Regular { fields, children } => {
                if *is_implicit {
                    panic!("Node types without subtypes must not be implicit (not start with \"_\"): _{}", sexp_name)
                }
                let field_accessors = fields.iter().map(|(name, field)| {
                    let name_sexp_literal = lit_str(name);
                    field.print(
                        (
                            Cow::Owned(format!("{}s", name)),
                            quote!(concat!("Get the field `", #name_sexp_literal, "`")),
                            quote! { self.0.children_by_field_name(#name_sexp_literal, c) }
                        ),
                        (
                            Cow::Borrowed(name),
                            quote!(concat!("Get the field `", #name_sexp_literal, "`")),
                            quote! { self.0.child_by_field_name(#name_sexp_literal) }
                        ),
                        None
                    )
                });
                let child_accessor = children.as_ref().map(|children| children.print(
                    (
                        Cow::Borrowed("children"),
                        quote!("Get the node's children"),
                        quote! { self.0.children(c) }
                    ),
                    (
                        Cow::Borrowed("child"),
                        quote!("Get the node's child"),
                        quote! { self.0.child(0) }
                    ),
                    Some((
                        Cow::Borrowed("child"),
                        quote!("Get the node's child #i"),
                        quote! { self.0.child(i) }
                    ))
                ));
                quote! {
                    #[doc = concat!("Typed node `", #sexp_name, "`")]
                    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
                    #[allow(non_camel_case_types)]
                    pub struct #ident<'tree>(tree_sitter::Node<'tree>);

                    #[automatically_derived]
                    impl<'tree> TryFrom<tree_sitter::Node<'tree>> for #ident<'tree> {
                        type Error = type_sitter_lib::IncorrectKind<'tree>;

                        #[inline]
                        fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
                            if node.kind() == #sexp_name_literal {
                                Ok(Self(node))
                            } else {
                                Err(type_sitter_lib::IncorrectKind {
                                    node,
                                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                                })
                            }
                        }
                    }

                    #[automatically_derived]
                    impl<'tree> type_sitter_lib::TypedNode<'tree> for #ident<'tree> {
                        const KIND: &'static str = #sexp_name_literal;

                        #[inline]
                        fn node(&self) -> &tree_sitter::Node<'tree> {
                            &self.0
                        }

                        #[inline]
                        unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
                            Self(node)
                        }
                    }

                    #[automatically_derived]
                    impl<'tree> #ident<'tree> {
                        #(#field_accessors)*
                        #child_accessor
                    }
                }
            }
        }
    }
}

impl Children {
    fn print(
        &self,
        (children_name, children_doc, children_body): (Cow<'_, str>, TokenStream, TokenStream),
        (child_name, child_doc, mut child_body): (Cow<'_, str>, TokenStream, TokenStream),
        child_i: Option<(Cow<'_, str>, TokenStream, TokenStream)>
    ) -> TokenStream {
        if self.multiple {
            let ident = ident!(&children_name, "node field name converted into a rust method name (this is a library error, please report)");
            let nonempty_doc = if self.required {
                quote! { #[doc = "This is guaranteed to return at least one child"] }
            } else {
                quote! {}
            };
            let child_type = NodeName::print_sum_type(&self.types);
            let children_fn = quote! {
                #[doc = #children_doc]
                #nonempty_doc
                #[allow(dead_code)]
                #[inline]
                pub fn #ident<'a>(&'a self, c: &'a mut tree_sitter::TreeCursor<'tree>) -> impl Iterator<Item = type_sitter_lib::NodeResult<'tree, #child_type>> + 'a {
                    #children_body.map(<#child_type as TryFrom<_>>::try_from)
                }
            };
            let child_i_fn = child_i.map(|(child_i_name, child_i_doc, child_i_body)| {
                let child_i_ident = ident!(&child_i_name, "node field name converted into a rust method name (this is a library error, please report)");
                quote! {
                    #[doc = #child_i_doc]
                    #[allow(dead_code)]
                    #[inline]
                    pub fn #child_i_ident(&self, i: usize) -> Option<type_sitter_lib::NodeResult<'tree, #child_type>> {
                        #child_i_body.map(<#child_type as TryFrom<_>>::try_from)
                    }
                }
            });
            quote! {
                #children_fn
                #child_i_fn
            }
        } else {
            let ident = ident!(&child_name, "node field name converted into a rust method name (this is a library error, please report)");
            let mut child_type = NodeName::print_sum_type(&self.types);
            child_body = quote! { #child_body.map(<#child_type as TryFrom<_>>::try_from) };
            child_type = quote! { type_sitter_lib::NodeResult<'tree, #child_type> };
            if self.required {
                child_body = quote! { #child_body.expect("tree-sitter node missing its required child, there should at least be a MISSING node in its place") };
            } else {
                child_type = quote! { Option<#child_type> };
            }
            quote! {
                #[doc = #child_doc]
                #[allow(dead_code)]
                #[inline]
                pub fn #ident(&self) -> #child_type {
                    #child_body
                }
            }
        }
    }
}

impl NodeName {
    fn print_sum_type(types: &[NodeName]) -> TokenStream {
        match types.len() {
            0 => quote! { type_sitter_lib::either_n::Void },
            1 => {
                let type_ = NodeName::print_type(&types[0]);
                quote! { #type_ }
            },
            // 26 is the largest EitherN type, otherwise it gets split into an either of eithers
            n if n <= 26 => {
                let types = types.iter().map(NodeName::print_type);
                #[allow(non_snake_case)]
                let EitherN = ident!(&format!("Either{}", n), "<won't fail>");
                quote! { type_sitter_lib::either_n::#EitherN<#(#types),*> }
            }
            _ => {
                let types = types.iter().map(NodeName::print_type);
                quote! { type_sitter_lib::Either![#(#types),*] }
            }
        }
    }

    fn print_type(&self) -> TokenStream {
        let ident = ident!(&self.rust_type_name, "node kind converted into a rust type name (this is a library error, please report)");
        quote! { #ident<'tree> }
    }

    fn print_variant(&self) -> TokenStream {
        let ident = ident!(&self.rust_type_name, "node kind converted into a rust type name (this is a library error, please report)");
        quote! {
            #ident(#ident<'tree>),
        }
    }

    fn print_from_case(&self) -> TokenStream {
        let ident = ident!(&self.rust_type_name, "node kind converted into a rust type name (this is a library error, please report)");
        let sexp_name_literal = lit_str(&self.sexp_name);
        quote! {
            #sexp_name_literal => Ok(unsafe { Self::#ident(<#ident as type_sitter_lib::TypedNode<'tree>>::from_node_unchecked(node)) }),
        }
    }

    fn print_node_case(&self) -> TokenStream {
        let ident = ident!(&self.rust_type_name, "node kind converted into a rust type name (this is a library error, please report)");
        quote! {
            Self::#ident(x) => x.node(),
        }
    }
}