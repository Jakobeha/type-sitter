use std::borrow::Cow;
use std::cell::RefCell;
use indexmap::IndexMap;
use join_lazy_fmt::Join;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{Ident, LitStr, Path};
use crate::mk_syntax::{ident, lit_str};
use crate::names::NodeName;
use crate::node_types::generated_tokens::{AnonUnions, GeneratedNodeTokens};
use crate::node_types::types::{AnonUnionId, Children, NodeModule, NodeType, NodeTypeKind};

impl NodeType {
    pub fn print(&self, tree_sitter: &Path) -> GeneratedNodeTokens {
        let mut tokens = GeneratedNodeTokens::new();
        let NodeName { sexp_name, rust_type_name, rust_method_name: _, is_implicit, module } = &self.name;
        // Node type names are always valid identifiers
        let ident = ident!(rust_type_name, "node kind (rust type name)").unwrap();
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

        tokens.extend(*module, definition);
        tokens
    }
}

impl Children {
    fn print(
        &self,
        (children_name, children_doc, children_body, is_exact_size_iterator): (Cow<'_, str>, TokenStream, TokenStream, bool),
        (child_name, child_doc, mut child_body): (Cow<'_, str>, TokenStream, TokenStream),
        child_i: Option<(Cow<'_, str>, TokenStream, TokenStream)>,
        tree_sitter: &Path,
        anon_unions: &mut AnonUnions,
    ) -> TokenStream {
        if self.multiple {
            let ident = ident!(&children_name, "node field (rust method name)").unwrap();
            let nonempty_doc = if self.required {
                quote! { #[doc = "This is guaranteed to return at least one child"] }
            } else {
                quote! {}
            };
            let mut child_type = NodeName::print_sum_type(&self.types, tree_sitter, anon_unions);
            child_type = quote! { type_sitter_lib::ExtraOr<'tree, #child_type> };
            let iterator_type = if is_exact_size_iterator {
                quote! { ExactSizeIterator }
            } else {
                quote! { Iterator }
            };
            let children_fn = quote! {
                #[doc = #children_doc]
                #nonempty_doc
                #[allow(dead_code)]
                #[inline]
                pub fn #ident<'a>(&self, c: &'a mut #tree_sitter::TreeCursor<'tree>) -> impl #iterator_type<Item = type_sitter_lib::NodeResult<'tree, #child_type>> + 'a {
                    // Fun fact: <#child_type as TryFrom<_>>::try_from without the anonymous closure
                    //     causes a lifetime error, but this works fine. It may be compiler bug
                    #children_body.map(|n| <#child_type as TryFrom<_>>::try_from(n))
                }
            };
            let child_i_fn = child_i.map(|(child_i_name, child_i_doc, child_i_body)| {
                let child_i_ident = ident!(&child_i_name, "node field (rust method name)").unwrap();
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
            let ident = ident!(&child_name, "node field (rust method name)").unwrap();
            let mut child_type = NodeName::print_sum_type(&self.types, tree_sitter, anon_unions);
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
                    quote! { self.0.children_by_field_name(#name_sexp, c) },
                    false
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
            let children_and_fields = if fields.is_empty() {
                Cow::Borrowed(children)
            } else {
                let mut children_and_fields = children.clone();
                children_and_fields.extend(fields.values().cloned());
                Cow::Owned(children_and_fields)
            };
            children_and_fields.print(
                (
                    Cow::Borrowed("children"),
                    quote!("Get the node's named children"),
                    quote! { self.0.named_children(c) },
                    true
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
        let has_implicit_subtypes = subtypes.iter().any(|subtype| subtype.is_implicit);
        let variants = subtypes.iter().map(NodeName::print_variant_definition);
        let variant_accessors = subtypes.iter().map(NodeName::print_variant_accessor);
        let try_from = {
            let error = quote! {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            };
            if has_implicit_subtypes {
                let try_from_ifs = subtypes.iter().map(NodeName::print_try_from_if);
                quote! {
                    #(#try_from_ifs)*
                    #error
                }
            } else {
                let from_cases = subtypes.iter().map(NodeName::print_from_case);
                quote! {
                    match node.kind() {
                        #(#from_cases)*
                        _ => #error
                    }
                }
            }
        };
        let node_cases = subtypes.iter().map(NodeName::print_node_case);
        let node_mut_cases = subtypes.iter().map(NodeName::print_node_mut_case);
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
                    #try_from
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
                        #(#node_mut_cases)*
                    }
                }
            }
        }
    }

    pub fn print_sum_type(
        names: &[NodeName],
        tree_sitter: &Path,
        anon_unions: &mut AnonUnions
    ) -> TokenStream {
        Self::print_general_sum_type(
            names,
            quote! {},
            tree_sitter,
            anon_unions,
            || AnonUnionId::new(names)
        )
    }

    pub(crate) fn print_query_capture_sum_type(
        capture_variant_name: &str,
        names: &[NodeName],
        nodes: &Path,
        tree_sitter: &Path,
        anon_unions: &mut AnonUnions
    ) -> TokenStream {
        Self::print_general_sum_type(
            names,
            quote! { #nodes:: },
            tree_sitter,
            anon_unions,
            || AnonUnionId::query_capture(capture_variant_name)
        )
    }

    fn print_general_sum_type(
        names: &[NodeName],
        nodes_prefix: TokenStream,
        tree_sitter: &Path,
        anon_unions: &mut AnonUnions,
        mk_anon_union_id: impl FnOnce() -> AnonUnionId
    ) -> TokenStream {
        match names.len() {
            // Never type
            0 => quote! { type_sitter_lib::Never },
            // Regular type
            1 => {
                let type_ = NodeName::print_type(&names[0]);
                quote! { #nodes_prefix #type_ }
            },
            // Anonymous union
            _ => {
                let anon_union_id = mk_anon_union_id();
                let anon_union_name = ident!(&anon_union_id.name, "generated (anon union name)").unwrap();
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
                quote! { anon_unions::#anon_union_name }
            }
        }
    }

    pub(crate) fn print_type(&self) -> TokenStream {
        let ident = self.rust_type_ident();
        match self.module {
            NodeModule::Toplevel => quote! { #ident<'tree> },
            NodeModule::Unnamed => quote! { unnamed::#ident<'tree> },
            NodeModule::Symbols => quote! { symbols::#ident<'tree> },
        }
    }

    fn print_variant_definition(&self) -> TokenStream {
        let ident = self.rust_type_ident();
        let type_ = self.print_type();
        quote! {
            #ident(#type_),
        }
    }

    fn print_variant_accessor(&self) -> TokenStream {
        let ident = self.rust_type_ident();
        let type_ = self.print_type();
        let method = self.rust_method_ident();
        let sexp_name = self.sexp_lit_str();
        quote! {
            #[doc = concat!("Returns the node if it is of kind `", #sexp_name, "`, otherwise returns None")]
            #[inline]
            #[allow(unused, non_snake_case)]
            pub fn #method(self) -> Option<#type_> {
                match self {
                    Self::#ident(x) => Some(x),
                    _ => None,
                }
            }
        }
    }

    fn print_try_from_if(&self) -> TokenStream {
        let ident = self.rust_type_ident();
        let type_ = self.print_type();
        quote! {
            if let Ok(this) = <#type_ as TryFrom<_>>::try_from(node) {
                return Ok(Self::#ident(this));
            }
        }
    }

    fn print_from_case(&self) -> TokenStream {
        let ident = self.rust_type_ident();
        let type_ = self.print_type();
        let kind = self.sexp_lit_str();
        quote! {
            #kind => Ok(unsafe { Self::#ident(<#type_ as type_sitter_lib::TypedNode<'tree>>::from_node_unchecked(node)) }),
        }
    }

    fn print_node_case(&self) -> TokenStream {
        let ident = self.rust_type_ident();
        quote! {
            Self::#ident(x) => x.node(),
        }
    }

    fn print_node_mut_case(&self) -> TokenStream {
        let ident = self.rust_type_ident();
        quote! {
            Self::#ident(x) => x.node_mut(),
        }
    }

    pub(crate) fn rust_type_ident(&self) -> Ident {
        ident!(&self.rust_type_name, "node kind (rust type name)").unwrap()
    }

    fn rust_method_ident(&self) -> Ident {
        ident!(&self.rust_method_name, "node kind (rust method name)").unwrap()
    }

    fn sexp_lit_str(&self) -> LitStr {
        lit_str(&self.sexp_name)
    }
}

macro_rules! modularize {
    ($module:ident) => {
        if $module.is_empty() {
            quote!()
        } else {
            quote! {
                pub mod $module {
                    #[allow(unused_imports)]
                    use super::*;
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
            symbols,
            anon_unions
        } = self;
        let anon_unions = anon_unions.into_values().collect::<TokenStream>();
        let unnamed = modularize!(unnamed);
        let symbols = modularize!(symbols);
        let anon_unions = modularize!(anon_unions);
        quote! {
            #toplevel
            #unnamed
            #symbols
            #anon_unions
        }
    }
}