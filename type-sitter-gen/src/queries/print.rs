use std::iter::zip;
use proc_macro2::TokenStream;
use syn::{Ident, Path};
use quote::{format_ident, quote};
use tree_sitter::CaptureQuantifier;
use crate::mk_syntax::{concat_doc, ident, lit_str};
use crate::names::{NodeName, sexp_name_to_rust_names};
use crate::node_types::generated_tokens::AnonUnions;
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
    /// - `disabled_captures`: List of captur indices to ignore on the query
    /// - `nodes`: Path to the crate with the typed node wrappers. Typically
    ///   [type_sitter_gen::super_nodes]
    /// - `use_wrapper`: Whether to use `tree_sitter_wrapper` or `tree_sitter`
    /// - `tree_sitter`: Path to the crate with the tree-sitter bindings. Typically [tree_sitter]
    ///   if `use_wrapper` is false, or [type_sitter_lib::tree_sitter_wrapper] if `use_wrapper` is
    ///   true
    /// - `anon_unions`: Anonymous unions for query capture type
    pub fn print(
        &self,
        query_str: &str,
        ts_query: tree_sitter::Query,
        def_ident: &Ident,
        language_ident: &syn::Ident,
        disabled_patterns: &[&str],
        disabled_captures: &[usize],
        nodes: &Path,
        use_wrapper: bool,
        tree_sitter: &Path,
        anon_unions: &mut AnonUnions
    ) -> TokenStream {
        let (capture_idxs, capture_names) = ts_query.capture_names().iter().enumerate()
            .filter(|(capture_idx, _)| !disabled_captures.contains(capture_idx))
            .unzip::<_, _, Vec<_>, Vec<_>>();

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
        let matches_doc = concat_doc!("Matches returned by a query cursor running the query [", def_name, "]:", full_query_documentation);
        let query_match_doc = concat_doc!("A match returned by the query [", def_name, "]:", full_query_documentation);
        let captures_doc = concat_doc!("Captures returned by a query cursor running the query [", def_name, "]:", full_query_documentation);
        let query_capture_doc = concat_doc!("A capture returned by the query [", def_name, "]:", full_query_documentation);

        let (
            tree_t,
            tree_arg,
            tree_ident,
            tree_capture_node,
            tree_fn,
            tree_static,
            tree_to_raws
        ) = match use_wrapper {
            false => (
                quote! { , Text },
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
                quote! { tree: &'tree type_sitter_lib::tree_sitter_wrapper::Tree },
                quote! { tree },
                quote! {
                    // SAFETY: Same tree
                    unsafe { type_sitter_lib::tree_sitter_wrapper::Node::new(capture.node, tree) }
                },
                quote! {
                    #[inline]
                    fn tree(&self) -> &'tree type_sitter_lib::tree_sitter_wrapper::Tree {
                        self.tree
                    }
                },
                quote! { 'static, },
                capture_names.iter().map(|c| lit_str(c)).map(|capture_name| quote! {
                    type_sitter_lib::tree_sitter_wrapper::QueryCapture {
                        node: *node.node(),
                        name: #capture_name,
                    }
                }).collect::<Vec<_>>()
            )
        };

        // Pattern-idx-specific matches and capture-idx-specific captures (TODO)
        // Pattern-idx-agnostic matches and capture-idx-specific captures
        // Capture-idx-agnostic captures
        let capture_methods_and_variants = zip(&capture_idxs, &capture_names).map(|(capture_idx, capture_name)| {
            self.print_capture_method_and_variant(*capture_idx, capture_name, query_str, &ts_query, nodes, use_wrapper, tree_sitter, anon_unions)
        }).collect::<Vec<_>>();
        let capture_methods = capture_methods_and_variants.iter().map(|x| &x.0).collect::<Vec<_>>();
        let capture_variant_extract_methods = capture_methods_and_variants.iter().map(|x| &x.1).collect::<Vec<_>>();
        let capture_variant_idents = capture_methods_and_variants.iter().map(|x| &x.2).collect::<Vec<_>>();
        let capture_variant_documentations = capture_methods_and_variants.iter().map(|x| &x.3).collect::<Vec<_>>();
        let capture_node_types = capture_methods_and_variants.iter().map(|x| &x.4).collect::<Vec<_>>();

        quote! {
            #[allow(non_upper_case_globals)]
            static #internal_query_ident: type_sitter_lib::gen_internal::TypedQueryOnceBox<tree_sitter::Query> = type_sitter_lib::gen_internal::TypedQueryOnceBox::new();

            #[allow(non_snake_case)]
            fn #mk_internal_query_ident() -> Box<tree_sitter::Query> {
                #[allow(unused_mut)]
                let mut query = tree_sitter::Query::new(
                    #language_ident::language(),
                    #query_str
                ).expect(#query_parse_error);
                #(query.disable_capture(#disabled_captures);)*
                #(query.disable_pattern(#disabled_patterns);)*
                Box::new(query)
            }

            #[doc = #def_doc]
            #[allow(non_camel_case_types)]
            #[derive(Debug, Clone, Copy)]
            pub struct #def_ident;

            #[doc = #matches_doc]
            #[allow(unused, non_camel_case_types)]
            pub type #query_matches_ident<'cursor, 'tree #tree_t> = type_sitter_lib::TypedQueryMatches<'cursor, 'tree, #query_match_ident<'cursor, 'tree> #tree_t>;
            #[doc = #captures_doc]
            #[allow(unused, non_camel_case_types)]
            pub type #query_captures_ident<'cursor, 'tree #tree_t> = type_sitter_lib::TypedQueryCaptures<'cursor, 'tree, #query_match_ident<'cursor, 'tree> #tree_t>;
            #[doc = #query_match_doc]
            #[derive(Debug)]
            pub struct #query_match_ident<'cursor, 'tree> {
                match_: tree_sitter::QueryMatch<'cursor, 'tree>,
                #tree_arg
            }
            #[doc = #query_capture_doc]
            #[derive(Debug)]
            pub enum #query_capture_ident<'cursor, 'tree> {
                #(#capture_variant_documentations #capture_variant_idents {
                    node: #capture_node_types,
                    match_: Option<#query_match_ident<'cursor, 'tree>>
                },)*
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
                    match_: tree_sitter::QueryMatch<'cursor, 'tree>,
                    #tree_arg
                ) -> Self::Match<'cursor, 'tree> {
                    Self::Match { match_, #tree_ident }
                }

                #[inline]
                unsafe fn wrap_capture<'cursor, 'tree>(
                    &self,
                    capture: tree_sitter::QueryCapture<'tree>,
                    match_: Option<Self::Match<'cursor, 'tree>>,
                    #tree_arg
                ) -> Self::Capture<'cursor, 'tree> {
                    // SAFETY: As long as the capture came from the query this is safe, because the
                    // query only captures nodes of the correct type
                    match capture.index as usize {
                        #(#capture_idxs => Self::Capture::#capture_variant_idents {
                            node: <#capture_node_types as type_sitter_lib::TypedNode<'tree>>::from_node_unchecked(#tree_capture_node),
                            match_
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
            impl<'cursor, 'tree> type_sitter_lib::TypedQueryMatch<'cursor, 'tree> for #query_match_ident<'cursor, 'tree> {
                type Query = #def_ident;

                #[inline]
                fn query(&self) -> &'cursor Self::Query {
                    &#def_ident
                }

                #tree_fn

                #[inline]
                fn raw(&self) -> &tree_sitter::QueryMatch<'cursor, 'tree> {
                    &self.match_
                }

                #[inline]
                fn into_raw(self) -> tree_sitter::QueryMatch<'cursor, 'tree> {
                    self.match_
                }
            }

            #[automatically_derived]
            impl<'cursor, 'tree> #query_capture_ident<'cursor, 'tree> {
                #(#capture_variant_extract_methods)*
            }

            #[automatically_derived]
            impl<'cursor, 'tree> Clone for #query_capture_ident<'cursor, 'tree> {
                fn clone(&self) -> Self {
                    match self {
                        #(Self::#capture_variant_idents { node, .. } => Self::#capture_variant_idents { node: *node, match_: None }),*
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
                fn match_(&self) -> Option<&<Self::Query as type_sitter_lib::TypedQuery>::Match<'cursor, 'tree>> {
                    match self {
                        #(Self::#capture_variant_idents { match_, .. } => match_.as_ref()),*
                    }
                }

                #[inline]
                fn into_match(self) -> Option<<Self::Query as type_sitter_lib::TypedQuery>::Match<'cursor, 'tree>> {
                    match self {
                        #(Self::#capture_variant_idents { match_, .. } => match_),*
                    }
                }

                #[inline]
                fn to_raw(&self) -> #tree_sitter::QueryCapture<#tree_static 'tree> {
                    use type_sitter_lib::TypedNode;
                    match self {
                        #(Self::#capture_variant_idents { node, .. } => #tree_to_raws,)*
                    }
                }

                #[inline]
                fn node(&self) -> &#tree_sitter::Node<'tree> {
                    use type_sitter_lib::TypedNode;
                    match self {
                        #(Self::#capture_variant_idents { node, .. } => node.node()),*
                    }
                }

                #[inline]
                fn node_mut(&mut self) -> &mut #tree_sitter::Node<'tree> {
                    use type_sitter_lib::TypedNode;
                    match self {
                        #(Self::#capture_variant_idents { node, .. } => node.node_mut()),*
                    }
                }
            }
        }
    }


    fn print_capture_method_and_variant(
        &self,
        capture_idx: usize,
        capture_name: &str,
        query_str: &str,
        ts_query: &tree_sitter::Query,
        nodes: &Path,
        use_wrapper: bool,
        tree_sitter: &Path,
        anon_unions: &mut AnonUnions
    ) -> (TokenStream, TokenStream, Ident, TokenStream, TokenStream) {
        let (capture_variant_name, capture_method_name) = sexp_name_to_rust_names(&capture_name.replace(".", "_"));
        let capture_method_ident = ident!(capture_method_name, "capture name (capture method)")
            .expect("ident should be valid because we get them from a names function");
        let capture_variant_ident = ident!(capture_variant_name, "capture name (capture variant)")
            .expect("ident should be valid because we get them from a names function");

        let Some(captured_sexp) = self.captured_pattern(capture_name) else {
            panic!("capture name was found by tree-sitter but not by type-sitter: {}", capture_name)
        };
        let captured_sexp_str = &query_str[captured_sexp.span()];
        let capture_node_type = captured_sexp.node_type(false)
            .print(&capture_variant_name, nodes, tree_sitter, anon_unions);

        let capture_quantifier = ts_query.capture_quantifiers(capture_idx).iter().copied()
            .reduce(CaptureQuantifierExt::union)
            .unwrap_or(CaptureQuantifier::Zero);
        let captured_type = capture_quantifier.print_type(&capture_node_type);
        let captured_nonempty_iterator_doc = capture_quantifier.print_nonempty_iterator_doc();
        let capture_expr_n = match use_wrapper {
            false => quote! { n },
            true => quote! { type_sitter_lib::tree_sitter_wrapper::Node::new(n, self.tree) }
        };
        let captured_expr = capture_quantifier.print_expr(&quote! {
            // SAFETY: Query only captures nodes of the correct type and tree
            self.match_
                .nodes_for_capture_index(#capture_idx as u32)
                .map(|n| unsafe { <#capture_node_type as type_sitter_lib::TypedNode::<'tree>>::from_node_unchecked(#capture_expr_n) })
        });

        let full_capture_pattern_doc = concat_doc!(captured_sexp_str, " @", capture_name);
        let full_capture_documentation = quote! {
            #[doc = ""]
            #[doc = "The full capture including pattern is:"]
            #[doc = "```sexp"]
            #[doc = #full_capture_pattern_doc]
            #[doc = "```"]
        };

        let capture_method_doc = concat_doc!("Returns an iterator over the nodes captured by `", capture_name, "`");
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
        let capture_variant_extract_method_doc = concat_doc!("Try to interpret this capture as a `", capture_name, "`");
        let capture_variant_extract_method = quote! {
            #[doc = #capture_variant_extract_method_doc]
            #full_capture_documentation
            #[inline]
            #[allow(unused, non_snake_case)]
            pub fn #capture_method_ident(&self) -> Option<&#capture_node_type> {
                match self {
                    Self::#capture_variant_ident { node, .. } => Some(node),
                    #[allow(unreachable_patterns)]
                    _ => None
                }
            }
        };
        let capture_variant_main_doc = concat_doc!("A `", capture_name, "`");
        let capture_variant_documentation = quote! {
            #[doc = #capture_variant_main_doc]
            #full_capture_documentation
        };
        (capture_method, capture_variant_extract_method, capture_variant_ident, capture_variant_documentation, capture_node_type)
    }
}

impl SExpNodeType {
    fn print(
        &mut self,
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
                let names = names.make_contiguous();
                NodeName::print_query_capture_sum_type(capture_variant_name, names, nodes, &tree_sitter, anon_unions)
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