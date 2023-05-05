use std::borrow::Cow;
use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{Ident, LitStr};
use crate::node_types::{Children, NodeName, NodeType, NodeTypeKind, Type};

impl NodeType {
    pub fn print(&self) -> TokenStream {
        let NodeName { rust_type_name, rust_method_name: _, sexp_name, is_implicit } = &self.name;
        let ident = Ident::new(rust_type_name, Span::call_site());
        let sexp_name_literal = LitStr::new(sexp_name, Span::call_site());
        // let named = self.named;
        match &self.kind {
            NodeTypeKind::Supertype { subtypes } => {
                if !is_implicit {
                    panic!("Node types with subtypes must be implicit (start with \"_\")")
                }

                let variants = subtypes.iter().map(Type::print_variant);
                let from_cases = subtypes.iter().map(Type::print_from_case);
                let node_cases = subtypes.iter().map(Type::print_node_case);
                quote! {
                    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
                    #[doc = concat!("Typed node `", #sexp_name, "`")]
                    pub enum #ident<'tree> {
                        #(#variants)*
                    }

                    impl<'tree> TryFrom<TSNode<'tree>> for #ident<'tree> {
                        type Error = tree_sitter_lib::IncorrectKind<'tree>;

                        fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
                            match node.kind() {
                                #(#from_cases)*
                                _ => Err(tree_sitter_lib::IncorrectKind {
                                    node,
                                    kind: #sexp_name_literal,
                                })
                            }
                        }
                    }

                    impl<'tree> tree_sitter_lib::TypedNode<'tree> for #ident<'tree> {
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
                    panic!("Node types without subtypes must not be implicit (not start with \"_\")")
                }
                let field_accessors = fields.iter().map(|(name, field)| {
                    let name_sexp_literal = LitStr::new(name, Span::call_site());
                    field.print(
                        (
                            Cow::Owned(format!("{}s", name)),
                            quote!(concat!("Get the field `", #name_sexp_literal, "`")),
                            quote! { self.0.children_by_field_name(#name_sexp_literal, &mut c) }
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
                        quote! { self.0.children(&mut c) }
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
                    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
                    #[doc = concat!("Typed node `", #sexp_name, "`")]
                    pub struct #ident<'tree>(tree_sitter::Node<'tree>);

                    impl<'tree> TryFrom<TSNode<'tree>> for #ident<'tree> {
                        type Error = tree_sitter_lib::IncorrectKind<'tree>;

                        fn try_from(node: TSNode<'tree>) -> Result<Self, Self::Error> {
                            if node.kind() == #sexp_name_literal {
                                Ok(Self(node))
                            } else {
                                Err(tree_sitter_lib::IncorrectKind {
                                    node,
                                    kind: #sexp_name_literal,
                                })
                            }
                        }
                    }

                    impl<'tree> tree_sitter_lib::TypedNode<'tree> for #ident<'tree> {
                        fn node(&self) -> &tree_sitter::Node<'tree> {
                            &self.0
                        }
                    }

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
            let ident = Ident::new(&children_name, Span::call_site());
            let nonempty_doc = if self.required {
                quote! { #[doc = "This is guaranteed to return at least one child"] }
            } else {
                quote! {}
            };
            let child_type = Type::print_sum_type(&self.types);
            let children_fn = quote! {
                #[doc = #children_doc]
                #nonempty_doc
                pub fn #ident(&self, c: &mut tree_sitter::TreeCursor<'tree>) -> impl Iterator<Item = tree_sitter_lib::NodeResult<'tree, #child_type>> {
                    #children_body.map(#child_type::try_from)
                }
            };
            let child_i_fn = child_i.map(|(child_i_name, child_i_doc, child_i_body)| {
                let child_i_ident = Ident::new(&child_i_name, Span::call_site());
                quote! {
                    #[doc = #child_i_doc]
                    pub fn #child_i_ident(&self, i: usize) -> Option<tree_sitter_lib::NodeResult<'tree, #child_type>> {
                        #child_i_body.map(#child_type::try_from)
                    }
                }
            });
            quote! {
                #children_fn
                #child_i_fn
            }
        } else {
            let ident = Ident::new(&child_name, Span::call_site());
            let mut child_type = Type::print_sum_type(&self.types);
            child_type = quote! { tree_sitter_lib::NodeResult<'tree, #child_type> };
            if self.required {
                child_body = quote! { #child_body.expect("tree-sitter node missing its required child, there should at least be a MISSING node in its place") };
            } else {
                child_type = quote! { Option<#child_type> };
            }
            quote! {
                #[doc = #child_doc]
                pub fn #ident(&self) -> #child_type {
                    #child_body
                }
            }
        }
    }
}

impl Type {
    fn print_sum_type(types: &[Type]) -> TokenStream {
        let types = types.iter().map(Type::print_type);
        quote! { tree_sitter_lib::Either![#(#types),*] }
    }

    fn print_type(&self) -> TokenStream {
        let ident = Ident::new(&self.name.rust_type_name, Span::call_site());
        quote! { #ident<'tree> }
    }

    fn print_variant(&self) -> TokenStream {
        let ident = Ident::new(&self.name.rust_type_name, Span::call_site());
        quote! {
            #ident(#ident<'tree>),
        }
    }

    fn print_from_case(&self) -> TokenStream {
        let ident = Ident::new(&self.name.rust_type_name, Span::call_site());
        let sexp_name_literal = LitStr::new(&self.name.sexp_name, Span::call_site());
        quote! {
            #sexp_name_literal => Ok(Self::#ident(#ident(node))),
        }
    }

    fn print_node_case(&self) -> TokenStream {
        let ident = Ident::new(&self.name.rust_type_name, Span::call_site());
        quote! {
            Self::#ident(x) => x.node(),
        }
    }
}