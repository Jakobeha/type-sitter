use std::borrow::Cow;
use std::cell::RefCell;
use std::collections::HashSet;
use indexmap::IndexMap;
use join_lazy_fmt::Join;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{Ident, LitStr, Path};
use crate::anon_unions::{AnonUnionId, AnonUnions};
use crate::mk_syntax::{concat_doc, ident, lit_str, modularize};
use crate::names::NodeName;
use crate::node_types::detail_doc::{DetailDoc, ChildrenKind};
use crate::node_types::generated_tokens::GeneratedNodeTokens;
use crate::node_types::types::{Children, NodeModule, NodeType, NodeTypeKind};

impl NodeType {
    pub fn print(&self, tree_sitter: &Path, type_sitter_lib: &Path) -> GeneratedNodeTokens {
        let mut tokens = GeneratedNodeTokens::new();
        let NodeName { sexp_name, rust_type_name, rust_method_name: _, is_implicit, module } = &self.name;
        // Node type names are always valid identifiers
        let ident = ident!(rust_type_name, "node kind (rust type name)").unwrap();
        let kind = lit_str(sexp_name);
        let doc = concat_doc!("Typed node `", sexp_name, "`\n\n", DetailDoc::new(self).to_string());

        let definition = match &self.kind {
            NodeTypeKind::Supertype { subtypes } => {
                NodeName::print_sum_definition(
                    doc,
                    &ident,
                    &kind,
                    subtypes,
                    tree_sitter,
                    type_sitter_lib,
                )
            }
            NodeTypeKind::Regular { fields, children } => {
                if *is_implicit {
                    panic!("Node types without subtypes must not be implicit (not start with \"_\"): {}", sexp_name)
                }

                NodeName::print_product_definition(
                    doc,
                    &ident,
                    &kind,
                    fields,
                    children.as_ref(),
                    tree_sitter,
                    type_sitter_lib,
                    &mut tokens.anon_unions
                )
            }
        };

        tokens.append_tokens(*module, definition);
        tokens
    }
}

impl Children {
    fn print(
        &self,
        (children_name, children_doc, children_body): (Cow<'_, str>, TokenStream, TokenStream),
        (child_name, child_doc, mut child_body): (Cow<'_, str>, TokenStream, TokenStream),
        tree_sitter: &Path,
        type_sitter_lib: &Path,
        anon_unions: &mut AnonUnions,
    ) -> TokenStream {
        if self.multiple {
            let ident = ident!(children_name, "node field (rust method name)").unwrap();
            let nonempty_doc = if self.required {
                quote! { #[doc = "This is guaranteed to return at least one child"] }
            } else {
                quote! {}
            };
            let child_type = NodeName::print_sum_type(&self.types, tree_sitter, type_sitter_lib, anon_unions);
            quote! {
                #[doc = #children_doc]
                #nonempty_doc
                #[allow(dead_code)]
                #[inline]
                pub fn #ident<'a>(&self, c: &'a mut #type_sitter_lib::TreeCursor<'tree>) -> impl Iterator<Item = #type_sitter_lib::NodeResult<'tree, #child_type>> + 'a {
                    #children_body.map(<#child_type as #type_sitter_lib::Node<'tree>>::try_from_raw)
                }
            }
        } else {
            let ident = ident!(child_name, "node field (rust method name)").unwrap();
            let mut child_type = NodeName::print_sum_type(&self.types, tree_sitter, type_sitter_lib, anon_unions);
            child_body = quote! { #child_body.map(<#child_type as #type_sitter_lib::Node<'tree>>::try_from_raw) };
            child_type = quote! { #type_sitter_lib::NodeResult<'tree, #child_type> };
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
        type_sitter_lib: &Path,
        anon_unions: &mut AnonUnions,
    ) -> TokenStream {
        let anon_unions_ref = RefCell::new(anon_unions);
        let field_accessors = fields.iter().map(|(name, field)| {
            let mut anon_unions = anon_unions_ref.borrow_mut();
            let name_sexp = lit_str(name);
            field.print(
                (
                    Cow::Owned(format!("{}s", name)),
                    concat_doc!("Get the field `", name, "` which has kind ", ChildrenKind::new(field, false).to_string()),
                    quote! { self.0.children_by_field_name(#name_sexp, &mut c.0) }
                ),
                (
                    Cow::Borrowed(name),
                    concat_doc!("Get the field `", name, "` which has kind ", ChildrenKind::new(field, false).to_string()),
                    quote! { self.0.child_by_field_name(#name_sexp) }
                ),
                tree_sitter,
                type_sitter_lib,
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
                    quote! { "Get the node's not-extra named children" },
                    quote! { self.0.named_children(&mut c.0).filter(|n| !n.is_extra()) }
                ),
                (
                    Cow::Borrowed("child"),
                    quote! { "Get the node's only not-extra named child.\n\nIff this returns `Option`, it means the node may not have any non-extra named children." },
                    quote! {
                        (0..)
                            .filter_map(|i| self.0.named_child(i))
                            .filter(|n| !n.is_extra())
                            .next()
                    }
                ),
                tree_sitter,
                type_sitter_lib,
                &mut *anon_unions
            )
        });
        quote! {
            #[doc = #doc]
            #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
            #[repr(transparent)]
            #[allow(non_camel_case_types)]
            pub struct #ident<'tree>(#tree_sitter::Node<'tree>);

            #[automatically_derived]
            impl<'tree> #ident<'tree> {
                #(#field_accessors)*
                #children_accessors
            }

            #[automatically_derived]
            impl<'tree> #type_sitter_lib::Node<'tree> for #ident<'tree> {
                type WithLifetime<'a> = #ident<'a>;

                const KIND: &'static str = #kind;

                #[inline]
                fn try_from_raw(node: #tree_sitter::Node<'tree>) -> #type_sitter_lib::NodeResult<Self> {
                    if node.kind() == #kind {
                        Ok(Self(node))
                    } else {
                        Err(#type_sitter_lib::IncorrectKind::new::<Self>(node))
                    }
                }

                #[inline]
                unsafe fn from_raw_unchecked(node: #tree_sitter::Node<'tree>) -> Self {
                    debug_assert_eq!(node.kind(), #kind);
                    Self(node)
                }

                #[inline]
                fn raw(&self) -> &#tree_sitter::Node<'tree> {
                    &self.0
                }

                #[inline]
                fn raw_mut(&mut self) -> &mut #tree_sitter::Node<'tree> {
                    &mut self.0
                }

                #[inline]
                fn into_raw(self) -> #tree_sitter::Node<'tree> {
                    self.0
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
        type_sitter_lib: &Path,
    ) -> TokenStream {
        let has_implicit_subtypes = subtypes.iter().any(|subtype| subtype.is_implicit);

        let mut prev_variants = HashSet::new();
        let variants = subtypes
            .iter()
            .map(|name| name.print_variant_definition(&mut prev_variants));

        let mut prev_variants2 = HashSet::new();
        let mut prev_methods = HashSet::new();
        let variant_accessors = subtypes
            .iter()
            .map(|name| name.print_variant_accessor(&mut prev_variants2, &mut prev_methods, type_sitter_lib));

        let try_from = {
            let error = quote! {
                Err(#type_sitter_lib::IncorrectKind::new::<Self>(node))
            };
            if has_implicit_subtypes {
                let mut prev_variants3 = HashSet::new();
                let try_from_ifs = subtypes
                    .iter()
                    .map(|name| name.print_try_from_if(&mut prev_variants3, type_sitter_lib));

                quote! {
                    #(#try_from_ifs)*
                    #error
                }
            } else {
                let mut prev_variants3 = HashSet::new();
                let from_cases = subtypes
                    .iter()
                    .map(|name| name.print_from_case(&mut prev_variants3, type_sitter_lib));

                quote! {
                    match node.kind() {
                        #(#from_cases)*
                        _ => #error
                    }
                }
            }
        };

        let mut prev_variants4 = HashSet::new();
        let node_cases = subtypes
            .iter()
            .map(|name| name.print_raw_case(&mut prev_variants4, type_sitter_lib));

        let mut prev_variants5 = HashSet::new();
        let node_mut_cases = subtypes
            .iter()
            .map(|name| name.print_raw_mut_case(&mut prev_variants5, type_sitter_lib));

        let mut prev_variants6 = HashSet::new();
        let into_node_cases = subtypes
            .iter()
            .map(|name| name.print_into_raw_case(&mut prev_variants6));

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
            impl<'tree> #type_sitter_lib::Node<'tree> for #ident<'tree> {
                type WithLifetime<'a> = #ident<'a>;

                const KIND: &'static str = #kind;

                #[inline]
                fn try_from_raw(node: #tree_sitter::Node<'tree>) -> #type_sitter_lib::NodeResult<Self> {
                    #try_from
                }

                #[inline]
                fn raw(&self) -> &#tree_sitter::Node<'tree> {
                    match self {
                        #(#node_cases)*
                    }
                }

                #[inline]
                fn raw_mut(&mut self) -> &mut #tree_sitter::Node<'tree> {
                    match self {
                        #(#node_mut_cases)*
                    }
                }

                #[inline]
                fn into_raw(self) -> #tree_sitter::Node<'tree> {
                    match self {
                        #(#into_node_cases)*
                    }
                }
            }
        }
    }

    pub fn print_sum_type(
        names: &[NodeName],
        tree_sitter: &Path,
        type_sitter_lib: &Path,
        anon_unions: &mut AnonUnions
    ) -> TokenStream {
        Self::print_general_sum_type(
            names,
            quote! {},
            tree_sitter,
            type_sitter_lib,
            anon_unions,
            || AnonUnionId::new(names)
        )
    }

    pub(crate) fn print_query_capture_sum_type(
        capture_variant_name: &str,
        names: &[NodeName],
        nodes: &Path,
        tree_sitter: &Path,
        type_sitter_lib: &Path,
        anon_unions: &mut AnonUnions
    ) -> TokenStream {
        Self::print_general_sum_type(
            names,
            quote! { #nodes:: },
            tree_sitter,
            type_sitter_lib,
            anon_unions,
            || AnonUnionId::query_capture(capture_variant_name)
        )
    }

    fn print_general_sum_type(
        names: &[NodeName],
        nodes_prefix: TokenStream,
        tree_sitter: &Path,
        type_sitter_lib: &Path,
        anon_unions: &mut AnonUnions,
        mk_anon_union_id: impl FnOnce() -> AnonUnionId
    ) -> TokenStream {
        match names.len() {
            // Never type
            0 => quote! { #type_sitter_lib::Never },
            // Regular type
            1 => {
                let type_ = NodeName::print_type(&names[0]);
                quote! { #nodes_prefix #type_ }
            },
            // Anonymous union
            _ => {
                let anon_union_id = mk_anon_union_id();
                let anon_union_name = ident!(anon_union_id.name, "generated (anon union name)").unwrap();
                if !anon_unions.contains_key(&anon_union_id) {
                    let kind_str = NodeName::kind(names);
                    let kind_refs = format!("- [{}]", "]\n- [".join(names.iter().map(NodeName::rust_type_path)));
                    let kind = lit_str(&kind_str);
                    let definition = NodeName::print_sum_definition(
                        concat_doc!("one of `", kind_str, "`:\n", kind_refs),
                        &anon_union_name,
                        &kind,
                        names,
                        tree_sitter,
                        type_sitter_lib,
                    );
                    anon_unions.insert(anon_union_id, definition);
                }
                quote! { anon_unions::#anon_union_name<'tree> }
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

    fn print_variant_definition(&self, prev_variants: &mut HashSet<String>) -> TokenStream {
        let ident = self.rust_variant_ident(prev_variants);
        let type_ = self.print_type();
        quote! {
            #ident(#type_),
        }
    }

    fn print_variant_accessor(
        &self,
        prev_variants: &mut HashSet<String>,
        prev_methods: &mut HashSet<String>,
        type_sitter_lib: &Path
    ) -> TokenStream {
        let ident = self.rust_variant_ident(prev_variants);
        let type_ = self.print_type();

        let mut method_name = self.rust_method_ident(prev_methods).to_string();
        // We must remove the `r#` prefix because we're prepending `as_` and we don't have to add
        // back because no reserved identifiers start with it.
        if method_name.starts_with("r#") {
            method_name = method_name[2..].to_owned();
        }
        let as_method = format_ident!("as_{}", method_name);

        let doc = concat_doc!("Returns the node if it is of kind `", self.sexp_name, "` ([`", self.rust_type_path(), "`]), otherwise returns None");
        quote! {
            #[doc = #doc]
            #[inline]
            #[allow(unused, non_snake_case)]
            pub fn #as_method(self) -> #type_sitter_lib::NodeResult<'tree, #type_> {
                #[allow(irrefutable_let_patterns)]
                if let Self::#ident(x) = self {
                    Ok(x)
                } else {
                    Err(#type_sitter_lib::IncorrectKind::new::<Self>(*#type_sitter_lib::Node::raw(&self)))
                }
            }
        }
    }

    fn print_try_from_if(&self, prev_variants: &mut HashSet<String>, type_sitter_lib: &Path) -> TokenStream {
        let ident = self.rust_variant_ident(prev_variants);
        let type_ = self.print_type();
        quote! {
            if let Ok(this) = <#type_ as #type_sitter_lib::Node<'tree>>::try_from_raw(node) {
                return Ok(Self::#ident(this));
            }
        }
    }

    fn print_from_case(&self, prev_variants: &mut HashSet<String>, type_sitter_lib: &Path) -> TokenStream {
        let ident = self.rust_variant_ident(prev_variants);
        let type_ = self.print_type();
        let kind = self.sexp_lit_str();
        quote! {
            #kind => Ok(unsafe { Self::#ident(<#type_ as #type_sitter_lib::Node<'tree>>::from_raw_unchecked(node)) }),
        }
    }

    fn print_raw_case(&self, prev_variants: &mut HashSet<String>, type_sitter_lib: &Path) -> TokenStream {
        let ident = self.rust_variant_ident(prev_variants);
        quote! {
            Self::#ident(x) => #type_sitter_lib::Node::raw(x),
        }
    }

    fn print_raw_mut_case(&self, prev_variants: &mut HashSet<String>, type_sitter_lib: &Path) -> TokenStream {
        let ident = self.rust_variant_ident(prev_variants);
        quote! {
            Self::#ident(x) => #type_sitter_lib::Node::raw_mut(x),
        }
    }

    fn print_into_raw_case(&self, prev_variants: &mut HashSet<String>) -> TokenStream {
        let ident = self.rust_variant_ident(prev_variants);
        quote! {
            Self::#ident(x) => x.into_raw(),
        }
    }

    fn rust_type_ident(&self) -> Ident {
        ident!(self.rust_type_name, "node kind (rust type name)").unwrap()
    }

    fn rust_variant_ident(&self, prev_variants: &mut HashSet<String>) -> Ident {
        ident!(disambiguate_then_add(&self.rust_type_name, prev_variants), "node kind (rust variant name)").unwrap()
    }

    fn rust_method_ident(&self, prev_methods: &mut HashSet<String>) -> Ident {
        ident!(disambiguate_then_add(&self.rust_method_name, prev_methods), "node kind (rust method name)").unwrap()
    }

    fn sexp_lit_str(&self) -> LitStr {
        lit_str(&self.sexp_name)
    }
}

impl GeneratedNodeTokens {
    /// Strip extra info, converting this into a regular [TokenStream]
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

/// Ensure the identifier is different than those in the set by appending underscores until it is.
/// Then add it to the set so future identifiers also get disambiguated.
fn disambiguate_then_add(ident: &str, prev_idents: &mut HashSet<String>) -> String {
    let mut ident = ident.to_string();
    while !prev_idents.insert(ident.clone()) {
        // We can remove the r# prefix because it's no longer reserved (there's no reserved
        // identifier which is another reserved identifier plus underscore).
        if ident.starts_with("r#") {
            ident = ident[2..].to_owned();
        }
        ident.push('_');
    }
    ident
}