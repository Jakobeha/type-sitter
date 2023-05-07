use std::borrow::Cow;
use std::cell::RefCell;
use indexmap::IndexMap;
use join_lazy_fmt::Join;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{Ident, LitStr, Path};
use crate::generated_tokens::{AnonUnions, GeneratedNodeTokens};
use crate::types::{AnonUnionId, Children, NodeName, NodeType, NodeTypeKind};
use crate::mk_syntax::{ident, lit_str};

impl NodeType {
    pub fn print(&self, tree_sitter: &Path) -> GeneratedNodeTokens {
        let mut tokens = GeneratedNodeTokens::new();
        let NodeName { sexp_name, rust_type_name, rust_method_name: _, is_implicit, is_named } = &self.name;
        let ident = ident!(rust_type_name, "node kind converted into a rust type name (this is a library error, please report)");
        let kind = lit_str(sexp_name);
        let doc = quote! { concat!("Typed node `", #sexp_name, "`") };

        let definition = match &self.kind {
            NodeTypeKind::Supertype { subtypes } => {
                if !is_implicit {
                    panic!("Node types with subtypes must be implicit (start with \"_\"): _{}", sexp_name)
                }

                NodeName::print_sum_definition(
                    doc,
                    &ident,
                    &kind,
                    subtypes,
                    tree_sitter
                )
            }
            NodeTypeKind::Regular { fields, children } => {
                if *is_implicit {
                    panic!("Node types without subtypes must not be implicit (not start with \"_\"): _{}", sexp_name)
                }

                NodeName::print_product_definition(
                    doc,
                    &ident,
                    &kind,
                    fields,
                    children.as_ref(),
                    tree_sitter,
                    &mut tokens.anon_unions
                )
            }
        };

        if *is_named {
            tokens.toplevel.extend(definition);
        } else {
            tokens.unnamed.extend(definition);
        }
        tokens
    }
}

impl Children {
    fn print(
        &self,
        (children_name, children_doc, children_body): (Cow<'_, str>, TokenStream, TokenStream),
        (child_name, child_doc, mut child_body): (Cow<'_, str>, TokenStream, TokenStream),
        child_i: Option<(Cow<'_, str>, TokenStream, TokenStream)>,
        tree_sitter: &Path,
        anon_unions: &mut AnonUnions,
    ) -> TokenStream {
        if self.multiple {
            let ident = ident!(&children_name, "node field name converted into a rust method name (this is a library error, please report)");
            let nonempty_doc = if self.required {
                quote! { #[doc = "This is guaranteed to return at least one child"] }
            } else {
                quote! {}
            };
            let mut child_type = NodeName::print_sum_name(&self.types, tree_sitter, anon_unions);
            child_type = quote! { type_sitter_lib::ExtraOr<'tree, #child_type> };
            let children_fn = quote! {
                #[doc = #children_doc]
                #nonempty_doc
                #[allow(dead_code)]
                #[inline]
                pub fn #ident<'a>(&'a self, c: &'a mut #tree_sitter::TreeCursor<'tree>) -> impl Iterator<Item = type_sitter_lib::NodeResult<'tree, #child_type>> + 'a {
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
            let mut child_type = NodeName::print_sum_name(&self.types, tree_sitter, anon_unions);
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
    fn print_product_definition(
        doc: TokenStream,
        ident: &Ident,
        kind: &LitStr,
        fields: &IndexMap<String, Children>,
        children: Option<&Children>,
        tree_sitter: &Path,
        anon_unions: &mut AnonUnions,
    ) -> TokenStream {
        let anon_unions_ref = RefCell::new(anon_unions);
        let field_accessors = fields.iter().map(|(name, field)| {
            let mut anon_unions = anon_unions_ref.borrow_mut();
            let name_sexp = lit_str(name);
            field.print(
                (
                    Cow::Owned(format!("{}s", name)),
                    quote!(concat!("Get the field `", #name_sexp, "`")),
                    quote! { self.0.children_by_field_name(#name_sexp, c) }
                ),
                (
                    Cow::Borrowed(name),
                    quote!(concat!("Get the field `", #name_sexp, "`")),
                    quote! { self.0.child_by_field_name(#name_sexp) }
                ),
                None,
                tree_sitter,
                &mut *anon_unions
            )
        });
        let children_accessors = children.map(|children| {
            let mut anon_unions = anon_unions_ref.borrow_mut();
            children.print(
                (
                    Cow::Borrowed("children"),
                    quote!("Get the node's named children"),
                    quote! { self.0.named_children(c) }
                ),
                (
                    Cow::Borrowed("child"),
                    quote!("Get the node's only named child"),
                    quote! { self.0.named_child(0) }
                ),
                Some((
                    Cow::Borrowed("child"),
                    quote!("Get the node's named child #i"),
                    quote! { self.0.named_child(i) }
                )),
                tree_sitter,
                &mut *anon_unions
            )
        });
        quote! {
            #[doc = #doc]
            #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
            #[allow(non_camel_case_types)]
            pub struct #ident<'tree>(#tree_sitter::Node<'tree>);

            #[automatically_derived]
            impl<'tree> #ident<'tree> {
                #(#field_accessors)*
                #children_accessors
            }

            #[automatically_derived]
            impl<'tree> TryFrom<#tree_sitter::Node<'tree>> for #ident<'tree> {
                type Error = type_sitter_lib::IncorrectKind<'tree>;

                #[inline]
                fn try_from(node: #tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
                    if node.kind() == #kind {
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
                const KIND: &'static str = #kind;

                #[inline]
                fn node(&self) -> &#tree_sitter::Node<'tree> {
                    &self.0
                }

                #[inline]
                fn node_mut(&mut self) -> &mut #tree_sitter::Node<'tree> {
                    &mut self.0
                }

                #[inline]
                unsafe fn from_node_unchecked(node: #tree_sitter::Node<'tree>) -> Self {
                    Self(node)
                }
            }
        }
    }

    fn print_sum_definition(
        doc: TokenStream,
        ident: &Ident,
        kind: &LitStr,
        subtypes: &[NodeName],
        tree_sitter: &Path,
    ) -> TokenStream {
        let variants = subtypes.iter().map(NodeName::print_variant_definition);
        let variant_accessors = subtypes.iter().map(NodeName::print_variant_accessor);
        let from_cases = subtypes.iter().map(NodeName::print_from_case);
        let node_cases = subtypes.iter().map(NodeName::print_node_case);
        let node_cases2 = subtypes.iter().map(NodeName::print_node_case);
        quote! {
            #[doc = #doc]
            #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
            #[allow(non_camel_case_types)]
            pub enum #ident<'tree> {
                #(#variants)*
            }

            #[automatically_derived]
            impl<'tree> #ident<'tree> {
                #(#variant_accessors)*
            }

            #[automatically_derived]
            impl<'tree> TryFrom<#tree_sitter::Node<'tree>> for #ident<'tree> {
                type Error = type_sitter_lib::IncorrectKind<'tree>;

                #[inline]
                fn try_from(node: #tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
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
                const KIND: &'static str = #kind;

                #[inline]
                fn node(&self) -> &#tree_sitter::Node<'tree> {
                    match self {
                        #(#node_cases)*
                    }
                }

                #[inline]
                fn node_mut(&mut self) -> &mut #tree_sitter::Node<'tree> {
                    match self {
                        #(#node_cases2)*
                    }
                }
            }
        }
    }

    fn print_sum_name(
        names: &[NodeName],
        tree_sitter: &Path,
        anon_unions: &mut AnonUnions
    ) -> TokenStream {
        match names.len() {
            // Never type
            0 => quote! { type_sitter_lib::Never },
            // Regular type
            1 => {
                let type_ = NodeName::print_type(&names[0]);
                quote! { #type_ }
            },
            // Anonnymous union
            _ => {
                let anon_union_id = AnonUnionId::new(names);
                let anon_union_name = ident!(&anon_union_id.name, "anon union name generated by type-sitter (this is a library error, please report)");
                if !anon_unions.contains_key(&anon_union_id) {
                    let kind = lit_str(&format!("{{{}}}", " | ".join(names.iter().map(|n| &n.sexp_name))));
                    let definition = NodeName::print_sum_definition(
                        quote! { concat!("one of `", #kind, "`") },
                        &anon_union_name,
                        &kind,
                        names,
                        tree_sitter,
                    );
                    anon_unions.insert(anon_union_id, definition);
                }
                quote! { #anon_union_name }
            }
        }
    }

    fn print_type(&self) -> TokenStream {
        let ident = ident!(&self.rust_type_name, "node kind converted into a rust type name (this is a library error, please report)");
        quote! { #ident<'tree> }
    }

    fn print_variant_definition(&self) -> TokenStream {
        let ident = ident!(&self.rust_type_name, "node kind converted into a rust type name (this is a library error, please report)");
        quote! {
            #ident(#ident<'tree>),
        }
    }
    fn print_variant_accessor(&self) -> TokenStream {
        let ident = ident!(&self.rust_type_name, "node kind converted into a rust type name (this is a library error, please report)");
        let method = ident!(&self.rust_method_name, "node kind converted into a rust type name (this is a library error, please report)");
        let sexp_name = lit_str(&self.sexp_name);
        quote! {
            #[doc = concat!("Returns the node if it is of kind `", #sexp_name, "`, otherwise returns None")]
            #[inline]
            #[allow(unused, non_snake_case)]
            pub fn #method(self) -> Option<#ident<'tree>> {
                match self {
                    Self::#ident(x) => Some(x),
                    _ => None,
                }
            }
        }
    }

    fn print_from_case(&self) -> TokenStream {
        let ident = ident!(&self.rust_type_name, "node kind converted into a rust type name (this is a library error, please report)");
        let kind = lit_str(&self.sexp_name);
        quote! {
            #kind => Ok(unsafe { Self::#ident(<#ident as type_sitter_lib::TypedNode<'tree>>::from_node_unchecked(node)) }),
        }
    }

    fn print_node_case(&self) -> TokenStream {
        let ident = ident!(&self.rust_type_name, "node kind converted into a rust type name (this is a library error, please report)");
        quote! {
            Self::#ident(x) => x.node(),
        }
    }
}

macro_rules! modularize {
    ($module:ident) => {
        if $module.is_empty() {
            quote!()
        } else {
            quote! {
                mod $module {
                    #$module
                }
            }
        }
    }
}

impl GeneratedNodeTokens {
    /// Strip extra info converting this into a regular [TokenStream]
    pub fn collapse(self) -> TokenStream {
        let GeneratedNodeTokens {
            toplevel,
            unnamed,
            anon_unions
        } = self;
        let anon_unions = anon_unions.into_values().collect::<TokenStream>();
        let unnamed = modularize!(unnamed);
        let anon_unions = modularize!(anon_unions);
        quote! {
            #toplevel
            #unnamed
            #anon_unions
        }
    }
}