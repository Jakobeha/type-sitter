use crate::anon_unions::AnonUnions;
use crate::mk_syntax::{concat_doc, ident, lit_array, lit_str, modularize};
use crate::{sexp_name_to_rust_names, unmake_reserved};
use crate::NodeType;
use crate::queries::sexp::SExpSeq;
use crate::queries::sexp_node_type::SExpNodeType;
use crate::queries::GeneratedQueryTokens;
use crate::PrintCtx;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use slice_group_by::GroupBy;
use syn::{Ident, Path};
use tree_sitter::CaptureQuantifier;

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
    /// - `ctx.all_types`: Map of node type names to their types.
    /// - `ctx.tree_sitter`: Path to the crate with the tree-sitter API. For cli-generated sources,
    ///    use [crate::tree_sitter] if `use_yak_sitter` is false or [crate::yak_sitter] if
    ///    `use_yak_sitter` is true. For proc-macro generated sources, use [crate::type_sitter_raw]
    ///    either way.
    /// - `ctx.type_sitter_lib`: Path to the crate with the type-sitter API. For cli-generated
    ///   sources, use [crate::type_sitter_lib]. For proc-macro generated sources, use
    ///   [crate::type_sitter].
    /// - `anon_unions`: Anonymous unions for query capture type
    pub(crate) fn print(
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
        ctx @ PrintCtx { tree_sitter, type_sitter_lib, .. }: PrintCtx,
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
        let internal_query = format_ident!("__{}__", def_ident);
        let query_matches = format_ident!("{}Matches", def_ident);
        let query_captures = format_ident!("{}Captures", def_ident);
        let query_match = format_ident!("{}Match", def_ident);
        let query_capture = format_ident!("{}Capture", def_ident);
        let disabled_patterns = disabled_patterns.iter().map(|p| lit_str(p));
        let full_query_documentation = format!("\n\n```sexp\n{}\n```", query_str);
        let def_doc = concat_doc!("Typed version of the query:", full_query_documentation);
        let matches_doc = concat_doc!("Matches returned by a query cursor running the query [`", def_name, "`]:", full_query_documentation);
        let query_match_doc = concat_doc!("A match returned by the query [`", def_name, "`]:", full_query_documentation);
        let captures_doc = concat_doc!("Captures returned by a query cursor running the query [`", def_name, "`]:", full_query_documentation);
        let query_capture_doc = concat_doc!("A capture returned by the query [`", def_name, "`]:", full_query_documentation);

        let (
            tree_t,
            tree_fn,
            tree_query,
            tree_to_raws
        ) = match use_yak_sitter {
            false => (
                quote! { , Text, I },
                quote! {},
                quote! {},
                capture_idxs.iter().map(|capture_idx| quote! {
                    #tree_sitter::QueryCapture {
                        index: #capture_idx as u32,
                        node: *node.raw()
                    }
                }).collect::<Vec<_>>()
            ),
            true => (
                quote! {},
                quote! {
                    #[inline]
                    fn tree(&self) -> &'tree yak_sitter::Tree {
                        self.0.tree()
                    }
                },
                quote! { 'query, },
                capture_idxs_and_names.iter()
                    .map(|(i, c)| (*i, lit_str(c)))
                    .map(|(capture_idx, capture_name)| quote! {
                        yak_sitter::QueryCapture {
                            node: *node.raw(),
                            index: #capture_idx,
                            name: #capture_name
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
                self.print_capture_method_and_variant(capture_name, &capture_idxs, query_str, &ts_query, nodes, ctx, anon_unions)
            })
            .collect::<Vec<_>>();
        let capture_methods = capture_methods_and_variants.iter().map(|x| x.0.clone()).collect::<TokenStream>();
        let capture_variant_extract_methods = capture_methods_and_variants.iter().map(|x| x.1.clone()).collect::<TokenStream>();
        let capture_variants = capture_methods_and_variants.iter().map(|x| &x.2).collect::<Vec<_>>();
        let capture_variant_documentations = capture_methods_and_variants.iter().map(|x| &x.3).collect::<Vec<_>>();
        let capture_node_types = capture_methods_and_variants.iter().map(|x| &x.4).collect::<Vec<_>>();
        let non_existent_variant = match capture_methods_and_variants.is_empty() {
            false => quote! {},
            true => quote! {
                /// This node has no captures so the enum has no instantiable variants. This variant
                /// is necessary to keep lifetime parameters, but the `Never` type means it can't be
                /// instantiated.
                __NonExistent(#type_sitter_lib::Never, std::marker::PhantomData<&'query &'tree ()>)
            }
        };

        quote! {
            #[doc = #def_doc]
            #[allow(non_camel_case_types)]
            #[derive(Debug, Clone, Copy)]
            pub struct #def_ident;

            #[doc = #matches_doc]
            #[allow(unused, non_camel_case_types)]
            pub type #query_matches<'query, 'tree #tree_t> = #type_sitter_lib::QueryMatches<'query, 'tree, #def_ident #tree_t>;
            #[doc = #captures_doc]
            #[allow(unused, non_camel_case_types)]
            pub type #query_captures<'query, 'tree #tree_t> = #type_sitter_lib::QueryCaptures<'query, 'tree, #def_ident #tree_t>;
            #[doc = #query_match_doc]
            #[repr(transparent)]
            pub struct #query_match<'query, 'tree: 'query>(#tree_sitter::QueryMatch<'query, 'tree>);
            #[doc = #query_capture_doc]
            #[derive(Clone, Debug)]
            pub enum #query_capture<'tree> {
                #(#capture_variant_documentations #capture_variants(#capture_node_types),)*
                #non_existent_variant
            }

            #[automatically_derived]
            impl #type_sitter_lib::Query for #def_ident {
                type Match<'query, 'tree: 'query> = #query_match<'query, 'tree>;
                type Capture<'query, 'tree: 'query> = #query_capture<'tree>;

                fn as_str(&self) -> &'static str {
                    #query_str
                }

                fn raw(&self) -> &'static #tree_sitter::Query {
                    #[allow(non_upper_case_globals)]
                    static #internal_query: std::sync::OnceLock<#tree_sitter::Query> = std::sync::OnceLock::new();

                    #internal_query.get_or_init(|| {
                        #[allow(unused_mut)]
                        let mut query = #tree_sitter::Query::new(
                            &#language_ident::LANGUAGE.into(),
                            #query_str
                        ).expect(#query_parse_error);
                        #(query.disable_capture(#disabled_captures);)*
                        #(query.disable_pattern(#disabled_patterns);)*
                        query
                    })
                }

                #[inline]
                unsafe fn wrap_match<'query, 'tree>(
                    &self,
                    r#match: #tree_sitter::QueryMatch<'query, 'tree>
                ) -> #query_match<'query, 'tree> {
                    #query_match(r#match)
                }

                #[inline]
                unsafe fn wrap_match_ref<'m, 'query, 'tree>(
                    &self,
                    r#match: &'m #tree_sitter::QueryMatch<'query, 'tree>
                ) -> &'m #query_match<'query, 'tree> {
                    // SAFETY: Same repr
                    &*(r#match as *const #tree_sitter::QueryMatch<'query, 'tree> as *const #query_match<'query, 'tree>)
                }

                #[inline]
                unsafe fn wrap_capture<'query, 'tree: 'query>(
                    &self,
                    capture: #tree_sitter::QueryCapture<#tree_query 'tree>,
                ) -> #query_capture<'tree> {
                    // SAFETY: As long as the capture came from the query this is safe, because the
                    // query only captures nodes of the correct type
                    match capture.index as usize {
                        #(#capture_idxs => #query_capture::#capture_variants(
                            <#capture_node_types as #type_sitter_lib::Node<'tree>>::from_raw_unchecked(capture.node)
                        ),)*
                        capture_index => unreachable!("Invalid capture index: {}", capture_index)
                    }
                }
            }

            #[automatically_derived]
            impl<'query, 'tree: 'query> #query_match<'query, 'tree> {
                #capture_methods
            }

            #[automatically_derived]
            impl<'query, 'tree: 'query> std::fmt::Debug for #query_match<'query, 'tree> {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    f.debug_tuple(stringify!(#query_match))
                        .field(&self.0)
                        .finish()
                }
            }

            #[automatically_derived]
            impl<'query, 'tree: 'query> #type_sitter_lib::QueryMatch<'query, 'tree> for #query_match<'query, 'tree> {
                type Query = #def_ident;

                #[inline]
                fn query(&self) -> &'query Self::Query {
                    &#def_ident
                }

                #tree_fn

                #[inline]
                fn raw(&self) -> &#tree_sitter::QueryMatch<'query, 'tree> {
                    &self.0
                }

                #[inline]
                fn into_raw(self) -> #tree_sitter::QueryMatch<'query, 'tree> {
                    self.0
                }
            }

            #[automatically_derived]
            impl<'tree> #query_capture<'tree> {
                #capture_variant_extract_methods
            }

            #[automatically_derived]
            impl<'query, 'tree: 'query> #type_sitter_lib::QueryCapture<'query, 'tree> for #query_capture<'tree> {
                type Query = #def_ident;

                #[inline]
                fn query(&self) -> &'query Self::Query {
                    &#def_ident
                }

                #[inline]
                fn raw(&self) -> #tree_sitter::QueryCapture<#tree_query 'tree> {
                    #[allow(unused_imports)]
                    use #type_sitter_lib::Node;
                    match self {
                        #(Self::#capture_variants(node) => #tree_to_raws,)*
                        // https://github.com/rust-lang/rust/issues/78123 for empty enums is why this exists
                        #[allow(unreachable_patterns)]
                        _ => unreachable!()
                    }
                }

                #[inline]
                fn node(&self) -> &#type_sitter_lib::UntypedNode<'tree> {
                    #[allow(unused_imports)]
                    use #type_sitter_lib::Node;
                    match self {
                        #(Self::#capture_variants(node) => #type_sitter_lib::UntypedNode::r#ref(node.raw()),)*
                        // https://github.com/rust-lang/rust/issues/78123 for empty enums is why this exists
                        #[allow(unreachable_patterns)]
                        _ => unreachable!()
                    }
                }

                #[inline]
                fn node_mut(&mut self) -> &mut #type_sitter_lib::UntypedNode<'tree> {
                    #[allow(unused_imports)]
                    use #type_sitter_lib::Node;
                    match self {
                        #(Self::#capture_variants(node) => #type_sitter_lib::UntypedNode::r#mut(node.raw_mut()),)*
                        // https://github.com/rust-lang/rust/issues/78123 for empty enums is why this exists
                        #[allow(unreachable_patterns)]
                        _ => unreachable!()
                    }
                }

                #[inline]
                fn name(&self) -> &'query str {
                    match self {
                        #(Self::#capture_variants { .. } => #capture_names,)*
                        // https://github.com/rust-lang/rust/issues/78123 for empty enums is why this exists
                        #[allow(unreachable_patterns)]
                        _ => unreachable!()
                    }
                }

                #[inline]
                fn index(&self) -> usize {
                    match self {
                        #(Self::#capture_variants { .. } => #capture_idxs,)*
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
        ctx @ PrintCtx { all_types, type_sitter_lib, .. }: PrintCtx,
        anon_unions: &mut AnonUnions
    ) -> (TokenStream, TokenStream, Ident, TokenStream, TokenStream) {
        let (capture_variant_name, capture_method_name) = sexp_name_to_rust_names(&capture_name.replace(".", "_"));
        let capture_method = ident!(capture_method_name, "capture name (capture method)")
            .expect("ident should be valid because we get them from a names function");
        let capture_variant = ident!(capture_variant_name, "capture name (capture variant)")
            .expect("ident should be valid because we get them from a names function");

        let mut capture_method_name = capture_method.to_string();
        // We must remove the `r#` prefix because we're prepending `as_` and we don't have to add
        // back because no reserved identifiers start with it.
        unmake_reserved(&mut capture_method_name);
        let as_capture_method = format_ident!("as_{}", capture_method_name);

        let captured_sexps = self.captured_patterns(capture_name).collect::<Vec<_>>();
        let captured_sexp_strs = captured_sexps.iter().map(|s| &query_str[s.span()]);
        let capture_node_type = captured_sexps.iter().map(|s| s.node_type(false, all_types))
            .collect::<SExpNodeType>();
        let capture_node_type_tokens = capture_node_type
            .print(&capture_variant_name, nodes, ctx, anon_unions);

        let capture_doc = format!("`{}` ([`{}`])", capture_name, capture_node_type.rust_type_path(nodes, &capture_variant_name));

        let capture_quantifier = capture_idxs.iter()
            .flat_map(|capture_idx| ts_query.capture_quantifiers(*capture_idx)).copied()
            .reduce(CaptureQuantifierExt::union)
            .unwrap_or(CaptureQuantifier::Zero);
        let captured_type = capture_quantifier.print_type(&capture_node_type_tokens);
        let captured_nonempty_iterator_doc = capture_quantifier.print_nonempty_iterator_doc();
        let capture_idxs_array = lit_array(capture_idxs.iter().map(|i| *i as u32));
        let captured_expr = capture_quantifier.print_expr(&quote! {
            #capture_idxs_array.into_iter().flat_map(|i| self.0.nodes_for_capture_index(i))
            // SAFETY: Query only captures nodes of the correct type and tree
                .map(|n| unsafe { <#capture_node_type_tokens as #type_sitter_lib::Node<'tree>>::from_raw_unchecked(n) })
        });

        let full_capture_pattern_doc = captured_sexp_strs.map(|captured_sexp_str| {
            let doc = concat_doc!(captured_sexp_str, " @", capture_name);
            quote! { #[doc = #doc] }
        }).collect::<TokenStream>();
        let full_capture_documentation = quote! {
            #[doc = ""]
            #[doc = "The full capture including pattern(s) is:"]
            #[doc = "```sexp"]
            #full_capture_pattern_doc
            #[doc = "```"]
        };

        let capture_method_doc = concat_doc!("Returns an iterator over the nodes captured by ", capture_doc);
        let capture_method = quote! {
            #[doc = #capture_method_doc]
            #captured_nonempty_iterator_doc
            #full_capture_documentation
            #[inline]
            pub fn #capture_method(&self) -> #captured_type {
                #captured_expr
            }
        };
        let capture_variant_extract_method_doc = concat_doc!("Try to interpret this capture as a ", capture_doc);
        let capture_variant_extract_method = quote! {
            #[doc = #capture_variant_extract_method_doc]
            #full_capture_documentation
            #[inline]
            pub fn #as_capture_method(&self) -> Option<&#capture_node_type_tokens> {
                #[allow(irrefutable_let_patterns)]
                if let Self::#capture_variant(node) = self {
                    Some(node)
                } else {
                    None
                }
            }
        };
        let capture_variant_main_doc = concat_doc!("A ", capture_doc);
        let capture_variant_documentation = quote! {
            #[doc = #capture_variant_main_doc]
            #full_capture_documentation
        };
        (capture_method, capture_variant_extract_method, capture_variant, capture_variant_documentation, capture_node_type_tokens)
    }
}

impl SExpNodeType {
    fn print(
        &self,
        capture_variant_name: &str,
        nodes: &Path,
        ctx @ PrintCtx { type_sitter_lib, .. }: PrintCtx,
        anon_unions: &mut AnonUnions
    ) -> TokenStream {
        match self {
            Self::Single { r#type } => {
                let type_ = r#type.print_rust_type();
                quote! { #nodes::#type_ }
            }
            Self::Union { types, .. } => {
                let mut types = types.iter().collect::<Vec<_>>();
                // Can't do `..._by_key` because `K` has an existential lifetime, which Rust can't express.
                types.sort_unstable_by(|lhs, rhs| lhs.name.cmp(&rhs.name));
                types.dedup_by(|lhs, rhs| lhs.name == rhs.name);

                NodeType::print_query_capture_sum_type(capture_variant_name, &types, nodes, ctx, anon_unions)
            }
            Self::Untyped { is_named } => match is_named {
                false => quote! { #type_sitter_lib::UntypedNode<'tree> },
                true => quote! { #type_sitter_lib::UntypedNamedNode<'tree> }
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
            CaptureQuantifier::OneOrMore =>
                quote! { #[doc = "\n\nThis is guaranteed to return at least one child"] }
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
    /// Strip extra info, converting this into a regular [`TokenStream`].
    ///
    /// To pretty-print, call [`into_string`](Self::into_string).
    ///
    /// ## Parameters
    /// - `nodes`: Path to the crate with the typed node wrappers. Typically
    ///   [`type_sitter_gen::super_nodes`]
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