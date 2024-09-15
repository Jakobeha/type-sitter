use proc_macro2::TokenStream;
use syn::{Ident, Path};
use quote::{format_ident, quote};
use slice_group_by::GroupBy;
use tree_sitter::CaptureQuantifier;
use crate::mk_syntax::{concat_doc, ident, lit_array, lit_str, modularize};
use crate::names::{NodeName, sexp_name_to_rust_names};
use crate::anon_unions::AnonUnions;
use crate::queries::GeneratedQueryTokens;
use crate::queries::sexp::SExpSeq;
use crate::queries::sexp_node_type::SExpNodeType;

impl<'tree> SExpSeq<'tree> {
    /// Generate the AST for the typed wrapper of the given query.
    ///
    /// # Parameters
    /// - `ts_query`: The tree-sitter version of the query at compile time
    /// - `def_ident`: Identifier used for the query definition
    /// - `language_ident`: Identifier used for the language to create the query for
    /// - `disable_patterns`: List of patterns to ignore on the query
    /// - `disabled_capture_names`: List of capture names to ignore on the query
    /// - `disabled_capture_idxs`: List of capture indices to ignore on the query (both these and
    ///   all indices with names in `disabled_capture_names` are disabled)
    /// - `nodes`: Path to the crate with the typed node wrappers. Typically
    ///   [type_sitter_gen::super_nodes]
    /// - `use_yak_sitter`: Whether to use `yak_sitter` or `tree_sitter`
    /// - `tree_sitter`: Path to the crate with the tree-sitter bindings. Typically [tree_sitter]
    ///   if `use_yak_sitter` is false, or [yak_sitter] if `use_yak_sitter` is
    ///   true
    /// - `anon_unions`: Anonymous unions for query capture type
    pub fn print(
        &self,
        query_str: &str,
        ts_query: tree_sitter::Query,
        def_ident: &Ident,
        language_ident: &syn::Ident,
        disabled_patterns: &[&str],
        disabled_capture_names: &[&str],
        disabled_capture_idxs: &[usize],
        nodes: &Path,
        use_yak_sitter: bool,
        tree_sitter: &Path,
        anon_unions: &mut AnonUnions
    ) -> TokenStream {
        let disabled_captures = disabled_capture_idxs.iter()
            .copied()
            .chain(disabled_capture_names.iter().flat_map(|&name| {
                ts_query.capture_names().iter().enumerate()
                    .filter(move |(_, n)| **n == name)
                    .map(|(idx, _)| idx)
            }))
            .collect::<Vec<_>>();
        let capture_idxs_and_names = ts_query.capture_names().iter().enumerate()
            .filter(|(capture_idx, _)| !disabled_captures.contains(capture_idx))
            .collect::<Vec<_>>();
        let (capture_idxs, capture_names) = capture_idxs_and_names.iter().map(|(idx, name)| (*idx, **name)).unzip::<_, _, Vec<_>, Vec<_>>();

        let def_name = def_ident.to_string();
        let language_name = language_ident.to_string();
        let query_parse_error = lit_str(&format!(
            "query parsed at compile-time but failed at runtime. Is the language '{}' correct, and did \
            you use the same tree-sitter / {} version?",
            language_name, language_name
        ));
        let internal_query_ident = format_ident!("__{}__", def_ident);
        let mk_internal_query_ident = format_ident!("__Mk__{}", def_ident);
        let query_matches_ident = format_ident!("{}Matches", def_ident);
        let query_captures_ident = format_ident!("{}Captures", def_ident);
        let query_match_ident = format_ident!("{}Match", def_ident);
        let query_capture_ident = format_ident!("{}Capture", def_ident);
        let disabled_patterns = disabled_patterns.iter().map(|p| lit_str(p));
        let full_query_documentation = format!("\n\n```sexp\n{}\n```", query_str);
        let def_doc = concat_doc!("Typed version of the query:", full_query_documentation);
        let matches_doc = concat_doc!("Matches returned by a query cursor running the query [`", def_name, "`]:", full_query_documentation);
        let query_match_doc = concat_doc!("A match returned by the query [`", def_name, "`]:", full_query_documentation);
        let captures_doc = concat_doc!("Captures returned by a query cursor running the query [`", def_name, "`]:", full_query_documentation);
        let query_capture_doc = concat_doc!("A capture returned by the query [`", def_name, "`]:", full_query_documentation);

        let (
            tree_t,
            tree_arg,
            tree_ident,
            tree_capture_node,
            tree_fn,
            tree_static,
            tree_to_raws
        ) = match use_yak_sitter {
            false => (
                quote! { , Text, I },
                quote! {},
                quote! {},
                quote! { capture.node },
                quote! {},
                quote! {},
                capture_idxs.iter().map(|capture_idx| quote! {
                    tree_sitter::QueryCapture {
                        index: #capture_idx as u32,
                        node: *node.node()
                    }
                }).collect::<Vec<_>>()
            ),
            true => (
                quote! {},
                quote! { tree: &'tree yak_sitter::Tree },
                quote! { tree },
                quote! {
                    // SAFETY: Same tree
                    unsafe { yak_sitter::Node::new(capture.node, tree) }
                },
                quote! {
                    #[inline]
                    fn tree(&self) -> &'tree yak_sitter::Tree {
                        self.tree
                    }
                },
                quote! { 'static, },
                capture_idxs_and_names.iter()
                    .map(|(i, c)| (*i, lit_str(c)))
                    .map(|(capture_idx, capture_name)| quote! {
                        yak_sitter::QueryCapture {
                            node: *node.node(),
                            index: #capture_idx,
                            name: #capture_name,
                        }
                    })
                    .collect::<Vec<_>>()
            )
        };

        // Pattern-idx-specific matches and capture-idx-specific captures (TODO)
        // Pattern-idx-agnostic matches and capture-idx-specific captures
        // Capture-idx-agnostic captures
        let capture_methods_and_variants = capture_idxs_and_names
            .binary_group_by_key(|(_, capture_name)| *capture_name)
            .map(|capture_idxs_and_name| {
                let capture_idxs = capture_idxs_and_name.iter().map(|(capture_idx, _)| *capture_idx).collect::<Vec<_>>();
                let capture_name = capture_idxs_and_name[0].1;
                self.print_capture_method_and_variant(capture_name, &capture_idxs, query_str, &ts_query, nodes, use_yak_sitter, tree_sitter, anon_unions)
            })
            .collect::<Vec<_>>();
        let capture_methods = capture_methods_and_variants.iter().map(|x| &x.0).collect::<Vec<_>>();
        let capture_variant_extract_methods = capture_methods_and_variants.iter().map(|x| &x.1).collect::<Vec<_>>();
        let capture_variant_idents = capture_methods_and_variants.iter().map(|x| &x.2).collect::<Vec<_>>();
        let capture_variant_documentations = capture_methods_and_variants.iter().map(|x| &x.3).collect::<Vec<_>>();
        let capture_node_types = capture_methods_and_variants.iter().map(|x| &x.4).collect::<Vec<_>>();
        let non_existent_variant = match capture_methods_and_variants.is_empty() {
            false => quote! {},
            true => quote! {
                /// This node has no captures so the enum has no instantiable variants. This variant
                /// is necessary keep lifetime parameters, but the [type_sitter_lib::Never] type
                /// means it can't be instantiated.
                __NonExistent(type_sitter_lib::Never, std::marker::PhantomData<&'cursor &'tree ()>)
            }
        };

        quote! {
            #[allow(non_upper_case_globals)]
            static #internal_query_ident: std::sync::OnceLock<tree_sitter::Query> = std::sync::OnceLock::new();

            #[allow(non_snake_case)]
            fn #mk_internal_query_ident() -> tree_sitter::Query {
                #[allow(unused_mut)]
                let mut query = tree_sitter::Query::new(
                    &#language_ident::LANGUAGE.into(),
                    #query_str
                ).expect(#query_parse_error);
                #(query.disable_capture(#disabled_captures);)*
                #(query.disable_pattern(#disabled_patterns);)*
                query
            }

            #[doc = #def_doc]
            #[allow(non_camel_case_types)]
            #[derive(Debug, Clone, Copy)]
            pub struct #def_ident;

            #[doc = #matches_doc]
            #[allow(unused, non_camel_case_types)]
            pub type #query_matches_ident<'cursor, 'tree #tree_t> = type_sitter_lib::TypedQueryMatches<'cursor, 'tree, #def_ident #tree_t>;
            #[doc = #captures_doc]
            #[allow(unused, non_camel_case_types)]
            pub type #query_captures_ident<'cursor, 'tree #tree_t> = type_sitter_lib::TypedQueryCaptures<'cursor, 'tree, #def_ident #tree_t>;
            #[doc = #query_match_doc]
            pub struct #query_match_ident<'cursor, 'tree> {
                r#match: tree_sitter::QueryMatch<'cursor, 'tree>,
                #tree_arg
            }
            #[doc = #query_capture_doc]
            pub enum #query_capture_ident<'cursor, 'tree> {
                #(#capture_variant_documentations #capture_variant_idents {
                    node: #capture_node_types,
                    r#match: Option<#query_match_ident<'cursor, 'tree>>
                },)*
                #non_existent_variant
            }

            #[automatically_derived]
            impl type_sitter_lib::TypedQuery for #def_ident {
                type Match<'cursor, 'tree: 'cursor> = #query_match_ident<'cursor, 'tree>;
                type Capture<'cursor, 'tree: 'cursor> = #query_capture_ident<'cursor, 'tree>;

                fn query_str(&self) -> &'static str {
                    #query_str
                }

                fn query(&self) -> &'static tree_sitter::Query {
                    #internal_query_ident.get_or_init(#mk_internal_query_ident)
                }

                #[inline]
                unsafe fn wrap_match<'cursor, 'tree>(
                    &self,
                    r#match: tree_sitter::QueryMatch<'cursor, 'tree>,
                    #tree_arg
                ) -> Self::Match<'cursor, 'tree> {
                    Self::Match { r#match, #tree_ident }
                }

                #[inline]
                unsafe fn wrap_capture<'cursor, 'tree>(
                    &self,
                    capture: tree_sitter::QueryCapture<'tree>,
                    r#match: Option<Self::Match<'cursor, 'tree>>,
                    #tree_arg
                ) -> Self::Capture<'cursor, 'tree> {
                    // SAFETY: As long as the capture came from the query this is safe, because the
                    // query only captures nodes of the correct type
                    match capture.index as usize {
                        #(#capture_idxs => Self::Capture::#capture_variant_idents {
                            node: <#capture_node_types as type_sitter_lib::TypedNode<'tree>>::from_node_unchecked(#tree_capture_node),
                            r#match
                        },)*
                        capture_index => unreachable!("Invalid capture index: {}", capture_index)
                    }
                }
            }

            #[automatically_derived]
            impl<'cursor, 'tree> #query_match_ident<'cursor, 'tree> {
                #(#capture_methods)*
            }

            #[automatically_derived]
            impl<'cursor, 'tree> std::fmt::Debug for #query_match_ident<'cursor, 'tree> {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    f.debug_struct(stringify!(#query_match_ident))
                        .field("r#match", &self.r#match)
                        .finish()
                }
            }

            #[automatically_derived]
            impl<'cursor, 'tree> type_sitter_lib::TypedQueryMatch<'cursor, 'tree> for #query_match_ident<'cursor, 'tree> {
                type Query = #def_ident;

                #[inline]
                fn query(&self) -> &'cursor Self::Query {
                    &#def_ident
                }

                #tree_fn

                #[inline]
                fn raw(&self) -> &tree_sitter::QueryMatch<'cursor, 'tree> {
                    &self.r#match
                }

                #[inline]
                fn into_raw(self) -> tree_sitter::QueryMatch<'cursor, 'tree> {
                    self.r#match
                }
            }

            #[automatically_derived]
            impl<'cursor, 'tree> #query_capture_ident<'cursor, 'tree> {
                #(#capture_variant_extract_methods)*
            }

            #[automatically_derived]
            impl<'cursor, 'tree> std::fmt::Debug for #query_capture_ident<'cursor, 'tree> {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    match self {
                        #(Self::#capture_variant_idents { node, .. } => f.debug_struct(concat!(stringify!(#query_capture_ident), "::", stringify!(#capture_variant_idents)))
                            .field("node", node)
                            .finish(),)*
                        // https://github.com/rust-lang/rust/issues/78123 for empty enums is why this exists
                        #[allow(unreachable_patterns)]
                        _ => unreachable!()
                    }
                }
            }

            #[automatically_derived]
            impl<'cursor, 'tree> Clone for #query_capture_ident<'cursor, 'tree> {
                fn clone(&self) -> Self {
                    match self {
                        #(Self::#capture_variant_idents { node, .. } => Self::#capture_variant_idents { node: *node, r#match: None },)*
                        // https://github.com/rust-lang/rust/issues/78123 for empty enums is why this exists
                        #[allow(unreachable_patterns)]
                        _ => unreachable!()
                    }
                }
            }

            #[automatically_derived]
            impl<'cursor, 'tree> type_sitter_lib::TypedQueryCapture<'cursor, 'tree> for #query_capture_ident<'cursor, 'tree> {
                type Query = #def_ident;

                #[inline]
                fn query(&self) -> &'cursor Self::Query {
                    &#def_ident
                }

                #[inline]
                fn r#match(&self) -> Option<&<Self::Query as type_sitter_lib::TypedQuery>::Match<'cursor, 'tree>> {
                    match self {
                        #(Self::#capture_variant_idents { r#match, .. } => r#match.as_ref(),)*
                        // https://github.com/rust-lang/rust/issues/78123 for empty enums is why this exists
                        #[allow(unreachable_patterns)]
                        _ => unreachable!()
                    }
                }

                #[inline]
                fn into_match(self) -> Option<<Self::Query as type_sitter_lib::TypedQuery>::Match<'cursor, 'tree>> {
                    match self {
                        #(Self::#capture_variant_idents { r#match, .. } => r#match,)*
                        // https://github.com/rust-lang/rust/issues/78123 for empty enums is why this exists
                        #[allow(unreachable_patterns)]
                        _ => unreachable!()
                    }
                }

                #[inline]
                fn to_raw(&self) -> #tree_sitter::QueryCapture<#tree_static 'tree> {
                    #[allow(unused_imports)]
                    use type_sitter_lib::TypedNode;
                    match self {
                        #(Self::#capture_variant_idents { node, .. } => #tree_to_raws,)*
                        // https://github.com/rust-lang/rust/issues/78123 for empty enums is why this exists
                        #[allow(unreachable_patterns)]
                        _ => unreachable!()
                    }
                }

                #[inline]
                fn node(&self) -> &#tree_sitter::Node<'tree> {
                    #[allow(unused_imports)]
                    use type_sitter_lib::TypedNode;
                    match self {
                        #(Self::#capture_variant_idents { node, .. } => node.node(),)*
                        // https://github.com/rust-lang/rust/issues/78123 for empty enums is why this exists
                        #[allow(unreachable_patterns)]
                        _ => unreachable!()
                    }
                }

                #[inline]
                fn node_mut(&mut self) -> &mut #tree_sitter::Node<'tree> {
                    #[allow(unused_imports)]
                    use type_sitter_lib::TypedNode;
                    match self {
                        #(Self::#capture_variant_idents { node, .. } => node.node_mut(),)*
                        // https://github.com/rust-lang/rust/issues/78123 for empty enums is why this exists
                        #[allow(unreachable_patterns)]
                        _ => unreachable!()
                    }
                }

                #[inline]
                fn name(&self) -> &'static str {
                    match self {
                        #(Self::#capture_variant_idents { .. } => #capture_names,)*
                        // https://github.com/rust-lang/rust/issues/78123 for empty enums is why this exists
                        #[allow(unreachable_patterns)]
                        _ => unreachable!()
                    }
                }

                #[inline]
                fn index(&self) -> usize {
                    match self {
                        #(Self::#capture_variant_idents { .. } => #capture_idxs,)*
                        // https://github.com/rust-lang/rust/issues/78123 for empty enums is why this exists
                        #[allow(unreachable_patterns)]
                        _ => unreachable!()
                    }
                }
            }
        }
    }


    fn print_capture_method_and_variant(
        &self,
        capture_name: &str,
        capture_idxs: &[usize],
        query_str: &str,
        ts_query: &tree_sitter::Query,
        nodes: &Path,
        use_yak_sitter: bool,
        tree_sitter: &Path,
        anon_unions: &mut AnonUnions
    ) -> (TokenStream, TokenStream, Ident, TokenStream, TokenStream) {
        let (capture_variant_name, capture_method_name) = sexp_name_to_rust_names(&capture_name.replace(".", "_"));
        let capture_method_ident = ident!(capture_method_name, "capture name (capture method)")
            .expect("ident should be valid because we get them from a names function");
        let capture_variant_ident = ident!(capture_variant_name, "capture name (capture variant)")
            .expect("ident should be valid because we get them from a names function");

        let captured_sexps = self.captured_patterns(capture_name).collect::<Vec<_>>();
        let captured_sexp_strs = captured_sexps.iter().map(|s| &query_str[s.span()]);
        let capture_node_type = captured_sexps.iter().map(|s| s.node_type(false))
            .collect::<SExpNodeType>();
        let capture_node_type_tokens = capture_node_type
            .print(&capture_variant_name, nodes, tree_sitter, anon_unions);

        let capture_doc = format!("`{}` ([{}])", capture_name, capture_node_type.rust_type_path(nodes, &capture_variant_name));

        let capture_quantifier = capture_idxs.iter()
            .flat_map(|capture_idx| ts_query.capture_quantifiers(*capture_idx)).copied()
            .reduce(CaptureQuantifierExt::union)
            .unwrap_or(CaptureQuantifier::Zero);
        let captured_type = capture_quantifier.print_type(&capture_node_type_tokens);
        let captured_nonempty_iterator_doc = capture_quantifier.print_nonempty_iterator_doc();
        let capture_expr_n = match use_yak_sitter {
            false => quote! { n },
            true => quote! { yak_sitter::Node::new(n, self.tree) }
        };
        let capture_idxs_array = lit_array(capture_idxs.iter().map(|i| *i as u32));
        let captured_expr = capture_quantifier.print_expr(&quote! {
            #capture_idxs_array.into_iter().flat_map(|i| self.r#match.nodes_for_capture_index(i))
            // SAFETY: Query only captures nodes of the correct type and tree
                .map(|n| unsafe { <#capture_node_type_tokens as type_sitter_lib::TypedNode<'tree>>::from_node_unchecked(#capture_expr_n) })
        });

        let full_capture_pattern_doc = captured_sexp_strs.map(|captured_sexp_str| {
            let doc = concat_doc!(captured_sexp_str, " @", capture_name);
            quote! { #[doc = #doc] }
        });
        let full_capture_documentation = quote! {
            #[doc = ""]
            #[doc = "The full capture including pattern(s) is:"]
            #[doc = "```sexp"]
            #(#full_capture_pattern_doc)*
            #[doc = "```"]
        };

        let capture_method_doc = concat_doc!("Returns an iterator over the nodes captured by ", capture_doc);
        let capture_method = quote! {
            #[doc = #capture_method_doc]
            #captured_nonempty_iterator_doc
            #full_capture_documentation
            #[inline]
            #[allow(unused, non_snake_case)]
            pub fn #capture_method_ident(&self) -> #captured_type {
                #captured_expr
            }
        };
        let capture_variant_extract_method_doc = concat_doc!("Try to interpret this capture as a ", capture_doc);
        let capture_variant_extract_method = quote! {
            #[doc = #capture_variant_extract_method_doc]
            #full_capture_documentation
            #[inline]
            #[allow(unused, non_snake_case)]
            pub fn #capture_method_ident(&self) -> Option<&#capture_node_type_tokens> {
                match self {
                    Self::#capture_variant_ident { node, .. } => Some(node),
                    #[allow(unreachable_patterns)]
                    _ => None
                }
            }
        };
        let capture_variant_main_doc = concat_doc!("A ", capture_doc);
        let capture_variant_documentation = quote! {
            #[doc = #capture_variant_main_doc]
            #full_capture_documentation
        };
        (capture_method, capture_variant_extract_method, capture_variant_ident, capture_variant_documentation, capture_node_type_tokens)
    }
}

impl SExpNodeType {
    fn print(
        &self,
        capture_variant_name: &str,
        nodes: &Path,
        tree_sitter: &Path,
        anon_unions: &mut AnonUnions
    ) -> TokenStream {
        match self {
            Self::Single { name } => {
                let type_ = name.print_type();
                quote! { #nodes::#type_ }
            }
            Self::Union { names, .. } => {
                let mut names = names.iter().cloned().collect::<Vec<_>>();
                // `names.sort_by_key(|name| &name.sexp_name)` and
                // `names.dedup_by_key(|name| &name.sexp_name)` give borrow-check error
                //   and I have NO idea why (either it's a compiler bug or I'm clueless)
                names.sort_unstable_by(|lhs, rhs| lhs.sexp_name.cmp(&rhs.sexp_name));
                names.dedup_by(|lhs, rhs| lhs.sexp_name == rhs.sexp_name);
                NodeName::print_query_capture_sum_type(capture_variant_name, &names, nodes, &tree_sitter, anon_unions)
            }
            Self::Untyped { is_named } => match is_named {
                false => quote! { type_sitter_lib::UntypedNode<'tree> },
                true => quote! { type_sitter_lib::UntypedNamedNode<'tree> }
            }
        }
    }
}

trait CaptureQuantifierExt {
    fn union(self, other: CaptureQuantifier) -> CaptureQuantifier;
    fn print_type(&self, inner_type: &TokenStream) -> TokenStream;
    fn print_nonempty_iterator_doc(&self) -> TokenStream;
    fn print_expr(&self, iterator: &TokenStream) -> TokenStream;
}

impl CaptureQuantifierExt for CaptureQuantifier {
    fn union(self, rhs: CaptureQuantifier) -> CaptureQuantifier {
        match (self, rhs) {
            (CaptureQuantifier::Zero, CaptureQuantifier::Zero) => CaptureQuantifier::Zero,
            (CaptureQuantifier::Zero, CaptureQuantifier::ZeroOrOne) => CaptureQuantifier::ZeroOrOne,
            (CaptureQuantifier::Zero, CaptureQuantifier::ZeroOrMore) => CaptureQuantifier::ZeroOrMore,
            (CaptureQuantifier::Zero, CaptureQuantifier::One) => CaptureQuantifier::ZeroOrOne,
            (CaptureQuantifier::Zero, CaptureQuantifier::OneOrMore) => CaptureQuantifier::ZeroOrMore,
            (CaptureQuantifier::ZeroOrOne, CaptureQuantifier::Zero) => CaptureQuantifier::ZeroOrOne,
            (CaptureQuantifier::ZeroOrOne, CaptureQuantifier::ZeroOrOne) => CaptureQuantifier::ZeroOrOne,
            (CaptureQuantifier::ZeroOrOne, CaptureQuantifier::ZeroOrMore) => CaptureQuantifier::ZeroOrMore,
            (CaptureQuantifier::ZeroOrOne, CaptureQuantifier::One) => CaptureQuantifier::ZeroOrOne,
            (CaptureQuantifier::ZeroOrOne, CaptureQuantifier::OneOrMore) => CaptureQuantifier::ZeroOrMore,
            (CaptureQuantifier::ZeroOrMore, CaptureQuantifier::Zero) => CaptureQuantifier::ZeroOrMore,
            (CaptureQuantifier::ZeroOrMore, CaptureQuantifier::ZeroOrOne) => CaptureQuantifier::ZeroOrMore,
            (CaptureQuantifier::ZeroOrMore, CaptureQuantifier::ZeroOrMore) => CaptureQuantifier::ZeroOrMore,
            (CaptureQuantifier::ZeroOrMore, CaptureQuantifier::One) => CaptureQuantifier::ZeroOrMore,
            (CaptureQuantifier::ZeroOrMore, CaptureQuantifier::OneOrMore) => CaptureQuantifier::ZeroOrMore,
            (CaptureQuantifier::One, CaptureQuantifier::Zero) => CaptureQuantifier::ZeroOrOne,
            (CaptureQuantifier::One, CaptureQuantifier::ZeroOrOne) => CaptureQuantifier::ZeroOrOne,
            (CaptureQuantifier::One, CaptureQuantifier::ZeroOrMore) => CaptureQuantifier::ZeroOrMore,
            (CaptureQuantifier::One, CaptureQuantifier::One) => CaptureQuantifier::One,
            (CaptureQuantifier::One, CaptureQuantifier::OneOrMore) => CaptureQuantifier::OneOrMore,
            (CaptureQuantifier::OneOrMore, CaptureQuantifier::Zero) => CaptureQuantifier::ZeroOrMore,
            (CaptureQuantifier::OneOrMore, CaptureQuantifier::ZeroOrOne) => CaptureQuantifier::ZeroOrMore,
            (CaptureQuantifier::OneOrMore, CaptureQuantifier::ZeroOrMore) => CaptureQuantifier::ZeroOrMore,
            (CaptureQuantifier::OneOrMore, CaptureQuantifier::One) => CaptureQuantifier::OneOrMore,
            (CaptureQuantifier::OneOrMore, CaptureQuantifier::OneOrMore) => CaptureQuantifier::OneOrMore,
        }
    }

    fn print_type(&self, inner_type: &TokenStream) -> TokenStream {
        match self {
            CaptureQuantifier::Zero => quote! { () },
            CaptureQuantifier::ZeroOrOne => quote! { Option<#inner_type> },
            CaptureQuantifier::ZeroOrMore => quote! { impl Iterator<Item=#inner_type> + '_ },
            CaptureQuantifier::One => quote! { #inner_type },
            CaptureQuantifier::OneOrMore => quote! { impl Iterator<Item=#inner_type> + '_ },
        }
    }

    fn print_nonempty_iterator_doc(&self) -> TokenStream {
        match self {
            CaptureQuantifier::Zero => quote! {},
            CaptureQuantifier::ZeroOrOne => quote! {},
            CaptureQuantifier::ZeroOrMore => quote! {},
            CaptureQuantifier::One => quote! {},
            CaptureQuantifier::OneOrMore => quote! { #[doc = "This is guaranteed to return at least one child"] }
        }
    }

    fn print_expr(&self, iterator: &TokenStream) -> TokenStream {
        let iterator = quote! {{ #iterator }};
        match self {
            CaptureQuantifier::Zero => quote! { debug_assert!(#iterator.next().is_none(), "zero quantifier returned an item") },
            CaptureQuantifier::ZeroOrOne => quote! { #iterator.next() },
            CaptureQuantifier::ZeroOrMore => quote! { #iterator },
            CaptureQuantifier::One => quote! {
                let result = #iterator.next().expect("one quantifier returned nothing");
                debug_assert!(#iterator.next().is_none(), "one quantifier returned more than one item");
                result
            },
            CaptureQuantifier::OneOrMore => quote! { #iterator },
        }
    }
}

impl GeneratedQueryTokens {
    /// Strip extra info, converting this into a regular [TokenStream]
    ///
    /// ## Parameters
    /// - `nodes`: Path to the crate with the typed node wrappers. Typically
    ///   [type_sitter_gen::super_nodes]
    pub fn collapse(self, nodes: &Path) -> TokenStream {
        let nodes = match nodes.segments.first().map_or(false, |s| s.ident.to_string() == "super") {
            false => quote! { #nodes },
            true => quote! { super::#nodes },
        };
        let GeneratedQueryTokens {
            toplevel,
            anon_unions
        } = self;
        let anon_unions = anon_unions.into_values().collect::<TokenStream>();
        let anon_unions = modularize!(anon_unions (use #nodes::*));
        quote! {
            #toplevel
            #anon_unions
        }
    }
}