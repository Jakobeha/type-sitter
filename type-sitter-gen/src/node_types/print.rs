use crate::anon_unions::{AnonUnionId, AnonUnions};
use crate::mk_syntax::{concat_doc, ident, lit_str, modularize};
use crate::node_types::detail_doc::{ChildrenKind, DetailDoc};
use crate::{make_valid, unmake_reserved, unmake_reserved_if_raw, Children, GeneratedNodeTokens, NodeModule, NodeName, NodeRustNames, NodeType, NodeTypeKind, PrintCtx};
use indexmap::IndexMap;
use join_lazy_fmt::Join;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use std::borrow::Cow;
use std::cell::RefCell;
use std::collections::HashSet;
use syn::{Ident, LitStr, Path};

impl NodeType {
    pub(crate) fn print(&self, ctx @ PrintCtx { all_types, .. }: PrintCtx) -> GeneratedNodeTokens {
        let mut tokens = GeneratedNodeTokens::new();

        let NodeName { sexp_name, is_named: _ } = &self.name;
        let NodeRustNames { type_name: rust_type_name, method_name: _, module } = &self.rust_names;
        let is_implicit = self.name.is_implicit();

        // Node type names are always valid identifiers
        let ident = ident!(rust_type_name, "node kind (rust type name)").unwrap();
        let kind = lit_str(sexp_name);
        let doc = concat_doc!("Typed node `", sexp_name, "`\n\n", DetailDoc::new(self, all_types).to_string());

        let definition = match &self.kind {
            NodeTypeKind::Supertype { subtypes: subtype_names } => {
                let subtypes = subtype_names.iter()
                    .map(|name| &all_types[name])
                    .collect::<Vec<_>>();

                Self::print_sum_definition(
                    doc,
                    &ident,
                    &kind,
                    &subtypes,
                    ctx
                )
            }
            NodeTypeKind::Regular { fields, children } => {
                if is_implicit {
                    panic!("Node types without subtypes must not be implicit (not start with \"_\"): {}", sexp_name)
                }

                Self::print_product_definition(
                    doc,
                    &ident,
                    &kind,
                    fields,
                    children.as_ref(),
                    ctx,
                    &mut tokens.anon_unions
                )
            }
        };

        tokens.append_tokens(*module, definition);
        tokens
    }

    fn print_product_definition(
        doc: TokenStream,
        ident: &Ident,
        kind: &LitStr,
        fields: &IndexMap<String, Children>,
        children: Option<&Children>,
        ctx @ PrintCtx { all_types, tree_sitter, type_sitter_lib }: PrintCtx,
        anon_unions: &mut AnonUnions,
    ) -> TokenStream {
        let fieldless_children = children.filter(|_| !fields.is_empty());
        let all_children = children.map(|children| {
            if fields.is_empty() {
                Cow::Borrowed(children)
            } else {
                let mut children_and_fields = children.clone();
                children_and_fields.extend(fields.values().cloned());
                Cow::Owned(children_and_fields)
            }
        });

        let anon_unions_cell = RefCell::new(anon_unions);

        let mut taken_names = HashSet::new();
        taken_names.extend(
            (&["children", "child", "others", "other"]).iter().map(|s| (*s).to_owned())
        );

        let field_accessors = fields.iter().map(|(name, field)| {
            let mut anon_unions = anon_unions_cell.borrow_mut();
            let name_sexp = lit_str(name);
            let kind_desc = ChildrenKind::new(field, true, all_types).to_string();

            let sanitized_name = disambiguate_then_add(make_valid(name), &mut taken_names);
            let sanitized_plural_name = disambiguate_then_add(make_valid(&format!("{}s", name)), &mut taken_names);

            field.print(
                (
                    Cow::Owned(sanitized_plural_name),
                    concat_doc!("Get the children of field `", name, "`.\n\nThese children have type ", kind_desc),
                    quote! { self.0.children_by_field_name(#name_sexp, &mut c.0) }
                ),
                (
                    Cow::Owned(sanitized_name),
                    concat_doc!("Get the field `", name, "`.\n\nThis child has type ", kind_desc),
                    concat_doc!("Get the optional field `", name, "`.\n\nThis child has type ", kind_desc),
                    quote! { self.0.child_by_field_name(#name_sexp) }
                ),
                ctx,
                &mut *anon_unions
            )
        });
        let fieldless_children_accessors = fieldless_children.map(|fieldless_children| {
            let mut anon_unions = anon_unions_cell.borrow_mut();
            let kind_desc = ChildrenKind::new(fieldless_children, true, all_types).to_string();
            fieldless_children.print(
                (
                    Cow::Borrowed("others"),
                    concat_doc!("Get the node's non-field not-extra named children.\n\nThese children have type ", kind_desc),
                    quote! {{
                        c.0.reset(self.0);
                        c.0.goto_first_child();
                        (0..self.0.child_count()).filter_map(move |_| {
                            let has_field = c.0.field_name().is_some();
                            let node = c.0.node();
                            c.0.goto_next_sibling();
                            if has_field && node.is_named() && !node.is_extra() {
                                Some(node)
                            } else {
                                None
                            }
                        })
                    }}
                ),
                (
                    Cow::Borrowed("other"),
                    concat_doc!("Get the node's only non-field not-extra named child.\n\nThis child has type ", kind_desc),
                    concat_doc!("Get the node's only non-field not-extra named child, if it has one.\n\nThis child has type ", kind_desc),
                    quote! {
                        // The cast `i as usize` is necessary in `tree-sitter` but not `yak-sitter`
                        #[allow(clippy::unnecessary_cast)]
                        (0..)
                            .filter(|i| self.0.field_name_for_child(*i).is_some())
                            .filter_map(|i| self.0.named_child(i as usize))
                            .filter(|n| !n.is_extra())
                            .next()
                    }
                ),
                ctx,
                &mut *anon_unions
            )
        });
        let all_children_accessors = all_children.map(|all_children| {
            let mut anon_unions = anon_unions_cell.borrow_mut();
            let kind_desc = ChildrenKind::new(&*all_children, true, all_types).to_string();
            all_children.print(
                (
                    Cow::Borrowed("children"),
                    concat_doc!("Get the node's not-extra named children.\n\nThese children have type ", kind_desc),
                    quote! { self.0.named_children(&mut c.0).filter(|n| !n.is_extra()) }
                ),
                (
                    Cow::Borrowed("child"),
                    concat_doc!("Get the node's only not-extra named child.\n\nThis child has type ", kind_desc),
                    concat_doc!("Get the node's only not-extra named child, if it has one.\n\nThis child has type ", kind_desc),
                    quote! {
                        (0..)
                            .filter_map(|i| self.0.named_child(i))
                            .filter(|n| !n.is_extra())
                            .next()
                    }
                ),
                ctx,
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
                #fieldless_children_accessors
                #all_children_accessors
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
        subtypes: &[&NodeType],
        PrintCtx { tree_sitter, type_sitter_lib, .. }: PrintCtx,
    ) -> TokenStream {
        let has_implicit_subtypes = subtypes.iter().any(|subtype| subtype.name.is_implicit());

        let mut prev_variants = HashSet::new();
        let variants = subtypes.iter()
            .map(|subtype| subtype.print_variant_definition(&mut prev_variants));

        let mut prev_variants2 = HashSet::new();
        let mut prev_methods = HashSet::new();
        let variant_accessors = subtypes.iter()
            .map(|subtype| subtype.print_variant_accessor(&mut prev_variants2, &mut prev_methods));

        let try_from = {
            let error = quote! {
                Err(#type_sitter_lib::IncorrectKind::new::<Self>(node))
            };
            if has_implicit_subtypes {
                let mut prev_variants3 = HashSet::new();
                let try_from_ifs = subtypes.iter()
                    .map(|name| name.print_try_from_if(&mut prev_variants3, type_sitter_lib));

                quote! {
                    #(#try_from_ifs)*
                    #error
                }
            } else {
                let mut prev_variants3 = HashSet::new();
                let from_cases = subtypes.iter()
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
        let node_cases = subtypes.iter()
            .map(|name| name.print_raw_case(&mut prev_variants4, type_sitter_lib));

        let mut prev_variants5 = HashSet::new();
        let node_mut_cases = subtypes.iter()
            .map(|name| name.print_raw_mut_case(&mut prev_variants5, type_sitter_lib));

        let mut prev_variants6 = HashSet::new();
        let into_node_cases = subtypes.iter()
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

    pub(crate) fn print_sum_type(
        types: &[&NodeType],
        ctx: PrintCtx,
        anon_unions: &mut AnonUnions
    ) -> TokenStream {
        Self::print_general_sum_type(
            types,
            quote! {},
            ctx,
            anon_unions,
            || AnonUnionId::new(types)
        )
    }

    pub(crate) fn print_query_capture_sum_type(
        capture_variant_name: &str,
        types: &[&NodeType],
        nodes: &Path,
        ctx: PrintCtx,
        anon_unions: &mut AnonUnions
    ) -> TokenStream {
        Self::print_general_sum_type(
            types,
            quote! { #nodes:: },
            ctx,
            anon_unions,
            || AnonUnionId::query_capture(capture_variant_name)
        )
    }

    fn print_general_sum_type(
        types: &[&NodeType],
        nodes_prefix: TokenStream,
        ctx @ PrintCtx { type_sitter_lib, .. }: PrintCtx,
        anon_unions: &mut AnonUnions,
        mk_anon_union_id: impl FnOnce() -> AnonUnionId
    ) -> TokenStream {
        match types.len() {
            // Never type
            0 => quote! { #type_sitter_lib::Never },
            // Regular type
            1 => {
                let type_ = types[0].print_rust_type();
                quote! { #nodes_prefix #type_ }
            },
            // Anonymous union
            _ => {
                let anon_union_id = mk_anon_union_id();
                let anon_union_name = ident!(anon_union_id.name, "generated (anon union name)").unwrap();
                if !anon_unions.contains_key(&anon_union_id) {
                    let kind_str = NodeName::kind(types.iter().map(|t| &t.name));
                    let kind_refs = format!("- [`{}`]", "`]\n- [`".join(types.iter().map(|t| t.rust_names.type_path())));
                    let kind = lit_str(&kind_str);
                    let definition = NodeType::print_sum_definition(
                        concat_doc!("One of `", kind_str, "`:\n", kind_refs),
                        &anon_union_name,
                        &kind,
                        types,
                        ctx
                    );
                    anon_unions.insert(anon_union_id, definition);
                }
                quote! { anon_unions::#anon_union_name<'tree> }
            }
        }
    }

    pub(crate) fn print_rust_type(&self) -> TokenStream {
        let ident = self.rust_type_ident();
        match self.rust_names.module {
            NodeModule::Toplevel => quote! { #ident<'tree> },
            NodeModule::Unnamed => quote! { unnamed::#ident<'tree> },
            NodeModule::Symbols => quote! { symbols::#ident<'tree> },
        }
    }

    fn print_variant_definition(&self, prev_variants: &mut HashSet<String>) -> TokenStream {
        let ident = self.rust_variant_ident(prev_variants);
        let type_ = self.print_rust_type();
        quote! {
            #ident(#type_),
        }
    }

    fn print_variant_accessor(
        &self,
        prev_variants: &mut HashSet<String>,
        prev_methods: &mut HashSet<String>
    ) -> TokenStream {
        let ident = self.rust_variant_ident(prev_variants);
        let type_ = self.print_rust_type();

        let mut method_name = self.rust_method_ident(prev_methods).to_string();
        // We must remove the `r#` prefix because we're prepending `as_` and we don't have to add
        // back because no reserved identifiers start with it.
        unmake_reserved(&mut method_name);
        let as_method = format_ident!("as_{}", method_name);

        let doc = concat_doc!("Returns the node if it is of type `", self.name.sexp_name, "` ([`", self.rust_type_path(), "`]), otherwise returns `None`");
        quote! {
            #[doc = #doc]
            #[inline]
            pub fn #as_method(self) -> Option<#type_> {
                #[allow(irrefutable_let_patterns)]
                if let Self::#ident(x) = self {
                    Some(x)
                } else {
                    None
                }
            }
        }
    }

    fn print_try_from_if(&self, prev_variants: &mut HashSet<String>, type_sitter_lib: &Path) -> TokenStream {
        let ident = self.rust_variant_ident(prev_variants);
        let type_ = self.print_rust_type();
        quote! {
            if let Ok(this) = <#type_ as #type_sitter_lib::Node<'tree>>::try_from_raw(node) {
                return Ok(Self::#ident(this));
            }
        }
    }

    fn print_from_case(&self, prev_variants: &mut HashSet<String>, type_sitter_lib: &Path) -> TokenStream {
        let ident = self.rust_variant_ident(prev_variants);
        let type_ = self.print_rust_type();
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
        ident!(self.rust_names.type_name, "node kind (rust type name)").unwrap()
    }

    fn rust_variant_ident(&self, prev_variants: &mut HashSet<String>) -> Ident {
        ident!(disambiguate_then_add(self.rust_names.type_name.clone(), prev_variants), "node kind (rust variant name)").unwrap()
    }

    fn rust_method_ident(&self, prev_methods: &mut HashSet<String>) -> Ident {
        ident!(disambiguate_then_add(self.rust_names.method_name.clone(), prev_methods), "node kind (rust method name)").unwrap()
    }

    fn sexp_lit_str(&self) -> LitStr {
        lit_str(&self.name.sexp_name)
    }
}

impl Children {
    fn print(
        &self,
        (children_name, children_doc, children_body): (Cow<'_, str>, TokenStream, TokenStream),
        (child_name, required_child_doc, optional_child_doc, mut child_body): (Cow<'_, str>, TokenStream, TokenStream, TokenStream),
        ctx @ PrintCtx { all_types, type_sitter_lib, .. }: PrintCtx,
        anon_unions: &mut AnonUnions,
    ) -> TokenStream {
        let types = self.types.iter().map(|name| &all_types[name]).collect::<Vec<_>>();

        if self.multiple {
            let ident = ident!(children_name, "node field (rust method name)").unwrap();
            let nonempty_doc = if self.required {
                quote! { #[doc = "\nThis is guaranteed to return at least one child."] }
            } else {
                quote! {}
            };
            let child_type = NodeType::print_sum_type(&types, ctx, anon_unions);
            quote! {
                #[doc = #children_doc]
                #nonempty_doc
                #[inline]
                pub fn #ident<'a>(&self, c: &'a mut #type_sitter_lib::TreeCursor<'tree>) -> impl Iterator<Item = #type_sitter_lib::NodeResult<'tree, #child_type>> + 'a {
                    #children_body.map(<#child_type as #type_sitter_lib::Node<'tree>>::try_from_raw)
                }
            }
        } else {
            let ident = ident!(child_name, "node field (rust method name)").unwrap();
            let mut child_type = NodeType::print_sum_type(&types, ctx, anon_unions);
            child_body = quote! { #child_body.map(<#child_type as #type_sitter_lib::Node<'tree>>::try_from_raw) };
            child_type = quote! { #type_sitter_lib::NodeResult<'tree, #child_type> };
            if self.required {
                child_body = quote! { #child_body.expect("required child not present, there should at least be a MISSING node in its place") };
            } else {
                child_type = quote! { Option<#child_type> };
            }
            let child_doc = if self.required {
                required_child_doc
            } else {
                optional_child_doc
            };
            quote! {
                #[doc = #child_doc]
                #[inline]
                pub fn #ident(&self) -> #child_type {
                    #child_body
                }
            }
        }
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
fn disambiguate_then_add(mut ident: String, prev_idents: &mut HashSet<String>) -> String {
    while !prev_idents.insert(ident.clone()) {
        // We can remove the r# prefix because it's no longer reserved (there's no reserved
        // identifier which is another reserved identifier plus underscore).
        unmake_reserved_if_raw(&mut ident);
        ident.push('_');
    }
    ident
}