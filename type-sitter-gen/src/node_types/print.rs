use crate::anon_unions::{AnonUnionId, AnonUnions};
use crate::mk_syntax::{concat_doc, ident, lit_str, modularize};
use crate::node_types::detail_doc::{ChildrenKind, DetailDoc};
use crate::node_types::{make_not_reserved, NodeTypeMap};
use crate::{make_valid, unmake_reserved, unmake_reserved_if_raw, Children, GeneratedNodeTokens, NodeModule, NodeName, NodeRustNames, NodeType, NodeTypeKind, PrintCtx};
use indexmap::IndexMap;
use join_lazy_fmt::Join;
use proc_macro2::TokenStream;
use quote::quote;
use std::collections::HashSet;
use std::fmt::Write;
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
                    ctx,
                    &mut tokens.anon_unions
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
                    children,
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
        other_children: &Children,
        ctx @ PrintCtx { tree_sitter, type_sitter_lib, .. }: PrintCtx,
        anon_unions: &mut AnonUnions,
    ) -> TokenStream {
        let mut prev_methods = HashSet::new();

        let child_accessors = Self::child_accessors(
            fields,
            other_children,
            &mut prev_methods,
            ctx,
            anon_unions
        );

        quote! {
            #[doc = #doc]
            #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
            #[repr(transparent)]
            #[allow(non_camel_case_types)]
            pub struct #ident<'tree>(#tree_sitter::Node<'tree>);

            #[automatically_derived]
            impl<'tree> #ident<'tree> {
                #child_accessors
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
        ctx @ PrintCtx { all_types, tree_sitter, type_sitter_lib }: PrintCtx,
        anon_unions: &mut AnonUnions,
    ) -> TokenStream {
        let has_implicit_subtypes = subtypes.iter().any(|subtype| subtype.name.is_implicit());

        // Don't clear `prev_methods`, do clear `prev_variants` before every iteration.
        // Because the variants are in the local `match` scope but the methods are in the same type
        // scope.
        let mut prev_variants = HashSet::new();
        let mut prev_methods = HashSet::new();

        let variants = subtypes.iter()
            .map(|subtype| subtype.print_variant_definition(&mut prev_variants))
            .collect::<TokenStream>();

        prev_variants.clear();
        let variant_accessors = subtypes.iter()
            .map(|subtype| subtype.print_variant_accessor(&mut prev_variants, &mut prev_methods))
            .collect::<TokenStream>();

        let inlined_variant_accessors =
            Self::print_inlined_subtype_variant_accessors(subtypes, &mut prev_methods, all_types);

        // We want accessors for the fields that are common to every variant.
        let common_fields = Self::common_subtype_fields(subtypes, all_types);
        let inlined_child_accessors = Self::child_accessors(
            &common_fields,
            &Children::EMPTY,
            &mut prev_methods,
            ctx,
            anon_unions
        );

        let try_from_raw_body = {
            let error = quote! {
                Err(#type_sitter_lib::IncorrectKind::new::<Self>(node))
            };
            if has_implicit_subtypes {
                prev_variants.clear();
                let try_from_ifs = subtypes.iter()
                    .map(|name| name.print_try_from_if(&mut prev_variants, type_sitter_lib))
                    .collect::<TokenStream>();

                quote! {
                    #try_from_ifs
                    #error
                }
            } else {
                prev_variants.clear();
                let from_cases = subtypes.iter()
                    .map(|name| name.print_from_case(&mut prev_variants, type_sitter_lib))
                    .collect::<TokenStream>();

                quote! {
                    match node.kind() {
                        #from_cases
                        _ => #error
                    }
                }
            }
        };

        prev_variants.clear();
        let node_cases = subtypes.iter()
            .map(|name| name.print_raw_case(&mut prev_variants, type_sitter_lib))
            .collect::<TokenStream>();

        prev_variants.clear();
        let node_mut_cases = subtypes.iter()
            .map(|name| name.print_raw_mut_case(&mut prev_variants, type_sitter_lib))
            .collect::<TokenStream>();

        prev_variants.clear();
        let into_node_cases = subtypes.iter()
            .map(|name| name.print_into_raw_case(&mut prev_variants))
            .collect::<TokenStream>();

        quote! {
            #[doc = #doc]
            #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
            #[allow(non_camel_case_types)]
            pub enum #ident<'tree> {
                #variants
            }

            #[automatically_derived]
            impl<'tree> #ident<'tree> {
                #variant_accessors
                #inlined_variant_accessors
                #inlined_child_accessors
            }

            #[automatically_derived]
            impl<'tree> #type_sitter_lib::Node<'tree> for #ident<'tree> {
                type WithLifetime<'a> = #ident<'a>;

                const KIND: &'static str = #kind;

                #[inline]
                fn try_from_raw(node: #tree_sitter::Node<'tree>) -> #type_sitter_lib::NodeResult<Self> {
                    #try_from_raw_body
                }

                #[inline]
                fn raw(&self) -> &#tree_sitter::Node<'tree> {
                    match self {
                        #node_cases
                    }
                }

                #[inline]
                fn raw_mut(&mut self) -> &mut #tree_sitter::Node<'tree> {
                    match self {
                        #node_mut_cases
                    }
                }

                #[inline]
                fn into_raw(self) -> #tree_sitter::Node<'tree> {
                    match self {
                        #into_node_cases
                    }
                }
            }
        }
    }

    fn print_inlined_subtype_variant_accessors(
        subtypes: &[&NodeType],
        prev_methods: &mut HashSet<String>,
        all_types: &NodeTypeMap
    ) -> TokenStream {
        struct Frame<'a> {
            outer_outer_rust_type: TokenStream,
            outer_sexp_name: &'a str,
            outer_rust_type: TokenStream,
            outer_as_method: Ident
        }

        // TODO: Unify this and `print_variant_accessors` because it doesn't handle the diamond
        //  problem, and the unified version would generate better code and maybe make compilation
        //  faster.
        fn rec<'a>(
            outer_subtypes: &[&'a NodeType],
            path: &mut Vec<Frame<'a>>,
            already_recursed: &mut HashSet<&'a NodeName>,
            already_printed: &mut HashSet<&'a NodeName>,
            prev_methods: &mut HashSet<String>,
            all_types: &'a NodeTypeMap,
        ) -> TokenStream {
            let outer_outer_rust_type = if let Some(Frame { outer_rust_type, .. }) = path.last() {
                outer_rust_type.clone()
            } else {
                quote!(Self)
            };

            let mut prev_outer_methods = HashSet::new();
            outer_subtypes.iter().copied().map(|outer_subtype| {
                if let NodeTypeKind::Supertype { subtypes: inner_subtype_names } = &outer_subtype.kind {
                    let inner_subtypes = inner_subtype_names.iter()
                        .map(|name| &all_types[name])
                        .collect::<Vec<_>>();
                    let outer_sexp_name = &outer_subtype.name.sexp_name;
                    let outer_rust_type = outer_subtype.print_rust_type();
                    let outer_as_method = outer_subtype.rust_as_method_ident(&mut prev_outer_methods);

                    // Importantly, we only continue after adding to `prev_outer_methods` if
                    // necessary. This is because we need the outer method names to be the same as
                    // they were non-transitively printed.
                    if already_recursed.contains(&outer_subtype.name) {
                        return TokenStream::new();
                    }

                    path.push(Frame {
                        outer_outer_rust_type: outer_outer_rust_type.clone(),
                        outer_sexp_name,
                        outer_rust_type,
                        outer_as_method
                    });

                    // Process immediate transitive accessor
                    let mut prev_inner_methods = HashSet::new();
                    let immediate_inlined = inner_subtypes.iter().map(|inner_subtype| {
                        let rust_type = inner_subtype.print_rust_type();
                        let inner_as_method = inner_subtype.rust_as_method_ident(&mut prev_inner_methods);
                        let as_method = inner_subtype.rust_as_method_ident(prev_methods);

                        // Importantly, we only continue after adding to `prev_inner_methods` if
                        // necessary. This is because we need the inner method names to be the same
                        // as they were non-transitively printed (same with `already_recursed`).
                        if already_printed.contains(&inner_subtype.name) {
                            return TokenStream::new();
                        }

                        let mut doc_str = inner_subtype.variant_accessor_doc_str();
                        doc_str.push_str(".\n\nFollows the following chain:");
                        for frame in path.iter() {
                            write!(
                                doc_str,
                                "\n- `{}` ([`{}`], from [`{}`]({}::{}))",
                                frame.outer_sexp_name,
                                frame.outer_rust_type,
                                frame.outer_as_method,
                                frame.outer_outer_rust_type,
                                frame.outer_as_method
                            ).unwrap();
                        }

                        let accessor_chain = path.iter().map(|Frame { outer_as_method, .. }| {
                            quote! { .#outer_as_method()? }
                        }).collect::<TokenStream>();

                        quote! {
                            #[doc = #doc_str]
                            #[inline]
                            pub fn #as_method(self) -> Option<#rust_type> {
                                self #accessor_chain .#inner_as_method()
                            }
                        }
                    }).collect::<TokenStream>();

                    // Process further transitive accessors
                    let transitive_inlined = rec(
                        &inner_subtypes,
                        path,
                        already_recursed,
                        already_printed,
                        prev_methods,
                        all_types
                    );

                    path.pop();

                    quote! {
                        #immediate_inlined
                        #transitive_inlined
                    }
                } else {
                    TokenStream::new()
                }
            }).collect::<TokenStream>()
        }

        rec(
            subtypes,
            &mut Vec::new(),
            &mut HashSet::new(),
            &mut subtypes.iter().map(|t| &t.name).collect(),
            prev_methods,
            all_types
        )
    }

    fn common_subtype_fields(
        subtypes: &[&NodeType],
        all_types: &NodeTypeMap
    ) -> IndexMap<String, Children> {
        let mut common_fields = None::<IndexMap<String, Children>>;

        let mut process = |fields: &IndexMap<String, Children>| {
            // Intersect the field names (keys), union-like merge the children (values).
            if let Some(common_fields) = common_fields.as_mut() {
                common_fields.retain(|name, _| fields.contains_key(name));
                for (name, children) in common_fields {
                    *children |= fields[name].clone();
                }
            } else {
                common_fields = Some(fields.clone());
            }
        };

        // Iterate DFS, so reverse before `extend`ing here and below.
        let mut worklist = subtypes.iter().copied().rev().collect::<Vec<_>>();

        while let Some(subtype) = worklist.pop() {
            match &subtype.kind {
                NodeTypeKind::Supertype { subtypes: subtype_names } => {
                    let subtypes = subtype_names.iter()
                        .map(|name| &all_types[name])
                        .collect::<Vec<_>>();

                    worklist.extend(subtypes.iter().copied().rev());
                }
                // Why do I need to add `&` to `fields` and `children` in `process`?
                NodeTypeKind::Regular { fields, children: _ } => process(&fields)
            }
        }

        common_fields.unwrap_or_default()
    }

    //noinspection DuplicatedCode
    fn child_accessors(
        fields: &IndexMap<String, Children>,
        other_children: &Children,
        prev_methods: &mut HashSet<String>,
        ctx @ PrintCtx { all_types, type_sitter_lib, .. }: PrintCtx,
        anon_unions: &mut AnonUnions
    ) -> TokenStream {
        let field_accessors = fields.iter().map(|(name, field)| {
            let name_sexp = lit_str(name);
            let kind_desc = ChildrenKind::new(field, true, all_types).to_string();

            field.print(
                (
                    make_valid(&format!("{}s", name)),
                    concat_doc!("Get the children of field `", name, "`.\n\nThese children have type ", kind_desc),
                    quote! { #type_sitter_lib::Node::raw(self).children_by_field_name(#name_sexp, &mut c.0) }
                ),
                (
                    make_valid(name),
                    concat_doc!("Get the field `", name, "`.\n\nThis child has type ", kind_desc),
                    concat_doc!("Get the optional field `", name, "`.\n\nThis child has type ", kind_desc),
                    quote! { #type_sitter_lib::Node::raw(self).child_by_field_name(#name_sexp) }
                ),
                prev_methods,
                ctx,
                &mut *anon_unions
            )
        }).collect::<TokenStream>();

        let children_accessor = if other_children.is_empty() {
            TokenStream::new()
        } else {
            let kind_desc = ChildrenKind::new(other_children, true, all_types).to_string();

            let mut other_name = if other_children.types.len() == 1 {
                let other_child_type = &other_children.types[0];
                let mut method_name = all_types[other_child_type].rust_names.method_name.clone();
                unmake_reserved(&mut method_name);
                method_name
            } else {
                if fields.is_empty() {
                    "child"
                } else {
                    "other"
                }.to_string()
            };
            let mut other_name_plural = if other_children.types.len() == 1 {
                format!("{}s", other_name)
            } else {
                if fields.is_empty() {
                    "children"
                } else {
                    "others"
                }.to_string()
            };
            make_not_reserved(&mut other_name);
            make_not_reserved(&mut other_name_plural);

            if fields.is_empty() {
                other_children.print(
                    (
                        other_name_plural,
                        concat_doc!("Get the node's not-extra named children.\n\nThese children have type ", kind_desc),
                        quote! { #type_sitter_lib::Node::raw(self).named_children(&mut c.0).filter(|n| !n.is_extra()) }
                    ),
                    (
                        other_name,
                        concat_doc!("Get the node's only not-extra named child.\n\nThis child has type ", kind_desc),
                        concat_doc!("Get the node's only not-extra named child, if it has one.\n\nThis child has type ", kind_desc),
                        quote! {
                            // We don't use a cursor because usually the first child will pass.
                            (0..#type_sitter_lib::Node::raw(self).named_child_count())
                                .map(|i| #type_sitter_lib::Node::raw(self).named_child(i).unwrap())
                                .filter(|n| !n.is_extra())
                                .next()
                        }
                    ),
                    prev_methods,
                    ctx,
                    &mut *anon_unions
                )
            } else {
                let kind_desc = ChildrenKind::new(other_children, true, all_types).to_string();

                other_children.print(
                    (
                        other_name_plural,
                        concat_doc!("Get the node's non-field not-extra named children.\n\nThese children have type ", kind_desc),
                        quote! {{
                        let raw = *#type_sitter_lib::Node::raw(self);
                        c.0.reset(raw);
                        c.0.goto_first_child();
                        std::iter::from_fn(move || loop {
                            let has_field = c.0.field_name().is_some();
                            let node = c.0.node();
                            let keep_going = c.0.goto_next_sibling();
                            if !has_field && node.is_named() && !node.is_extra() {
                                break Some(node)
                            } else if !keep_going {
                                break None
                            }
                        })
                    }}
                    ),
                    (
                        other_name,
                        concat_doc!("Get the node's only non-field not-extra named child.\n\nThis child has type ", kind_desc),
                        concat_doc!("Get the node's only non-field not-extra named child, if it has one.\n\nThis child has type ", kind_desc),
                        quote! {{
                        // This is the equivalent to the `others` body then calling `next`,
                        // though we create the cursor locally.
                        let raw = *#type_sitter_lib::Node::raw(self);
                        let mut c = raw.walk();
                        c.reset(raw);
                        c.goto_first_child();
                        loop {
                            let has_field = c.field_name().is_some();
                            let node = c.node();
                            let keep_going = c.goto_next_sibling();
                            if !has_field && node.is_named() && !node.is_extra() {
                                break Some(node)
                            } else if !keep_going {
                                break None
                            }
                        }
                    }}
                    ),
                    prev_methods,
                    ctx,
                    &mut *anon_unions
                )
            }
        };

        quote! {
            #field_accessors
            #children_accessor
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
                    // Insert a dummy value so we don't recurse.
                    anon_unions.insert(anon_union_id.clone(), TokenStream::new());

                    let kind_str = NodeName::kind(types.iter().map(|t| &t.name));
                    let kind_refs = format!("- [`{}`]", "`]\n- [`".join(types.iter().map(|t| t.rust_names.type_path())));
                    let kind = lit_str(&kind_str);
                    let definition = NodeType::print_sum_definition(
                        concat_doc!("One of `", kind_str, "`:\n", kind_refs),
                        &anon_union_name,
                        &kind,
                        types,
                        ctx,
                        anon_unions
                    );

                    anon_unions[&anon_union_id] = definition;
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
        let rust_type = self.print_rust_type();
        let as_method = self.rust_as_method_ident(prev_methods);

        let doc = self.variant_accessor_doc_str();

        quote! {
            #[doc = #doc]
            #[inline]
            pub fn #as_method(self) -> Option<#rust_type> {
                #[allow(irrefutable_let_patterns)]
                if let Self::#ident(x) = self {
                    Some(x)
                } else {
                    None
                }
            }
        }
    }

    fn variant_accessor_doc_str(&self) -> String {
        format!(
            "Returns the node if it is of type `{}` ([`{}`]), otherwise returns `None`",
            self.name.sexp_name,
            self.rust_type_path()
        )
    }

    fn print_try_from_if(&self, prev_variants: &mut HashSet<String>, type_sitter_lib: &Path) -> TokenStream {
        let ident = self.rust_variant_ident(prev_variants);
        let rust_type = self.print_rust_type();

        quote! {
            if let Ok(this) = <#rust_type as #type_sitter_lib::Node<'tree>>::try_from_raw(node) {
                return Ok(Self::#ident(this));
            }
        }
    }

    fn print_from_case(&self, prev_variants: &mut HashSet<String>, type_sitter_lib: &Path) -> TokenStream {
        let ident = self.rust_variant_ident(prev_variants);
        let rust_type = self.print_rust_type();
        let kind = self.sexp_lit_str();

        quote! {
            #kind => Ok(unsafe { Self::#ident(<#rust_type as #type_sitter_lib::Node<'tree>>::from_raw_unchecked(node)) }),
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

    fn rust_as_method_ident(&self, prev_methods: &mut HashSet<String>) -> Ident {
        let mut as_method_name = self.rust_names.method_name.clone();
        // We must remove the `r#` prefix because we're prepending `as_` and we don't have to add
        // back because no reserved identifiers start with it.
        unmake_reserved(&mut as_method_name);
        as_method_name.insert_str(0, "as_");

        ident!(disambiguate_then_add(as_method_name, prev_methods), "node kind (rust variant selector method name)").unwrap()
    }

    fn sexp_lit_str(&self) -> LitStr {
        lit_str(&self.name.sexp_name)
    }
}

impl Children {
    fn print(
        &self,
        (children_name, children_doc, children_body): (String, TokenStream, TokenStream),
        (child_name, required_child_doc, optional_child_doc, mut child_body): (String, TokenStream, TokenStream, TokenStream),
        prev_methods: &mut HashSet<String>,
        ctx @ PrintCtx { all_types, type_sitter_lib, .. }: PrintCtx,
        anon_unions: &mut AnonUnions,
    ) -> TokenStream {
        let types = self.types.iter().map(|name| &all_types[name]).collect::<Vec<_>>();

        if self.multiple {
            let children_name = disambiguate_then_add(children_name, prev_methods);
            let ident = ident!(children_name, "node field (rust method name)").unwrap();

            let nonempty_doc = if self.required {
                quote! { #[doc = "\n\nThis is guaranteed to return at least one child."] }
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
            let child_name = disambiguate_then_add(child_name, prev_methods);
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
    /// Strip extra info, converting this into a regular [`TokenStream`].
    ///
    /// To pretty-print, call [`into_string`](Self::into_string).
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