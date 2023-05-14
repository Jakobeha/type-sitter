use std::collections::VecDeque;
use proc_macro2::TokenStream;
use syn::{Ident, Path};
use quote::{format_ident, quote};
use tree_sitter::CaptureQuantifier;
use crate::mk_syntax::{concat_doc, ident, lit_str};
use crate::names::{NodeName, sexp_name_to_rust_names};
use crate::node_types::generated_tokens::AnonUnions;
use crate::node_types::types::NodeModule;
use crate::queries::sexp::{Atom, GroupType, SExp, SExpSeq};

enum SExpNodeType {
    Single { name: NodeName },
    Union { names: VecDeque<NodeName>, are_all_variants_named: bool },
    Untyped { is_named: bool },
}
// quote! { type_sitter_lib::UntypedAnonymousNode<'tree> }
impl SExpNodeType {
    fn union_with(self, other: Self) -> Self {
        match (self, other) {
            (this, Self::Untyped { is_named }) => {
                Self::Untyped { is_named: this.is_named() && is_named }
            }
            (Self::Untyped { is_named: self_is_named }, other) => {
                Self::Untyped { is_named: self_is_named && other.is_named() }
            },
            (Self::Single { name: self_name }, Self::Single { name }) => {
                let self_is_named = matches!(self_name.module, NodeModule::Toplevel);
                let is_named = matches!(name.module, NodeModule::Toplevel);
                Self::Union {
                    names: VecDeque::from([self_name, name]),
                    are_all_variants_named: self_is_named && is_named
                }
            }
            (Self::Union { mut names, are_all_variants_named }, Self::Single { name }) => {
                let is_named = matches!(name.module, NodeModule::Toplevel);
                names.push_back(name);
                Self::Union {
                    names,
                    are_all_variants_named: are_all_variants_named && is_named
                }
            }
            (Self::Single { name: self_name }, Self::Union { mut names, are_all_variants_named }) => {
                let self_is_named = matches!(self_name.module, NodeModule::Toplevel);
                names.push_front(self_name);
                Self::Union {
                    names,
                    are_all_variants_named: self_is_named && are_all_variants_named
                }
            }
            (Self::Union { names: mut self_names, are_all_variants_named: self_are_all_variants_named }, Self::Union { mut names, are_all_variants_named }) => {
                self_names.append(&mut names);
                Self::Union {
                    names: self_names,
                    are_all_variants_named: self_are_all_variants_named && are_all_variants_named
                }
            }
        }
    }

    fn is_named(&self) -> bool {
        match self {
            Self::Untyped { is_named } => *is_named,
            Self::Single { name } => matches!(name.module, NodeModule::Toplevel),
            Self::Union { are_all_variants_named, .. } => *are_all_variants_named
        }
    }

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
                false => quote! { type_sitter_lib::UntypedAnonymousNode<'tree> },
                true => quote! { type_sitter_lib::UntypedNamedNode<'tree> }
            }
        }
    }
}

impl From<NodeName> for SExpNodeType {
    fn from(name: NodeName) -> Self {
        Self::Single { name }
    }
}

impl FromIterator<SExpNodeType> for SExpNodeType {
    fn from_iter<T: IntoIterator<Item=SExpNodeType>>(iter: T) -> Self {
        let mut iter = iter.into_iter();
        let mut this = iter.next()
            .expect("can't create s-expression node type from empty iterator (shouldn't have tree-sitter query with empty union)");
        for next in iter {
            this = this.union_with(next);
        }
        this
    }
}

impl<'tree> SExpSeq<'tree> {
    /// Generate the AST for the typed wrapper of the given query.
    ///
    /// # Parameters
    /// - `ts_query`: The tree-sitter version of the query at compile time
    /// - `def_ident`: Identifier used for the query definition
    /// - `language_ident`: Identifier used for the language to create the query for
    /// - `disable_patterns`: List of patterns to ignore on the query
    /// - `disabled_captures`: List of captur indices to ignore on the query
    /// - `nodes`: Path to the crate with the typed node wrappers. Typically [type_sitter_gen::nodes]
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
        let captures = || ts_query.capture_names().iter().enumerate()
            .filter(|(capture_idx, _)| !disabled_captures.contains(capture_idx));

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
        let query_match_doc = concat_doc!("A match returned by the query [", def_name, "]:", full_query_documentation);
        let query_capture_doc = concat_doc!("A capture returned by the query [", def_name, "]:", full_query_documentation);

        let query_matches_t = match use_wrapper {
            true => quote! {},
            false => quote! { , T }
        };

        // Pattern-idx-specific matches and capture-idx-specific captures (TODO)
        // Pattern-idx-agnostic matches and capture-idx-specific captures
        // Capture-idx-agnostic captures
        let capture_methods_and_variants = captures().map(|(capture_idx, capture_name)| {
            self.print_capture_method_and_variant(capture_idx, capture_name, query_str, &ts_query, nodes, tree_sitter, anon_unions)
        }).collect::<Vec<_>>();
        let capture_idxs = captures().map(|(capture_idx, _)| capture_idx).collect::<Vec<_>>();
        let capture_methods = capture_methods_and_variants.iter().map(|x| &x.0).collect::<Vec<_>>();
        let capture_variant_extract_methods = capture_methods_and_variants.iter().map(|x| &x.1).collect::<Vec<_>>();
        let capture_variant_idents = capture_methods_and_variants.iter().map(|x| &x.2).collect::<Vec<_>>();
        let capture_variant_documentations = capture_methods_and_variants.iter().map(|x| &x.3).collect::<Vec<_>>();
        let capture_node_types = capture_methods_and_variants.iter().map(|x| &x.4).collect::<Vec<_>>();

        quote! {
            #[allow(non_upper_case_globals)]
            static #internal_query_ident: once_cell::race::OnceBox<tree_sitter::Query> = once_cell::race::OnceBox::new();

            #[allow(non_snake_case)]
            fn #mk_internal_query_ident() -> tree_sitter::Query {
                let mut query = tree_sitter::Query::new(
                    #language_ident::language(),
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

            pub type #query_matches_ident<'a, 'tree #query_matches_t> = TypedQueryMatches<'a, 'tree #query_matches_t, #query_match_ident<'a, 'tree>>;
            pub type #query_captures_ident<'a, 'tree #query_matches_t> = TypedQueryCaptures<'a, 'tree #query_matches_t, #query_match_ident<'a, 'tree>>;
            #[doc = #query_match_doc]
            #[derive(Debug)]
            pub struct #query_match_ident<'cursor, 'tree>(tree_sitter::QueryMatch<'cursor, 'tree>);
            #[doc = #query_capture_doc]
            #[derive(Debug, Clone, Copy)]
            pub enum #query_capture_ident<'tree> {
                #(#capture_variant_documentations #capture_variant_idents(#capture_node_types),)*
            }

            #[automatically_derived]
            impl TypedQuery for #def_ident {
                type Match<'cursor, 'tree> = #query_match_ident<'cursor, 'tree>;
                type Capture<'tree> = #query_capture_ident<'tree>;

                fn query_str(&self) -> &'static str {
                    #query_str
                }

                fn query(&self) -> &'static tree_sitter::Query {
                    #internal_query_ident.get_or_init(#mk_internal_query_ident)
                }

                #[inline]
                unsafe fn wrap_match<'cursor, 'tree>(
                    &self,
                    match_: tree_sitter::QueryMatch<'cursor, 'tree>
                ) -> Self::Match<'cursor, 'tree> {
                    Self::Match(match_)
                }

                #[inline]
                unsafe fn wrap_capture<'tree>(
                    &self,
                    capture: tree_sitter::QueryCapture<'tree>
                ) -> Self::Capture<'tree> {
                    // SAFETY: As long as the capture came from the query this is safe, because the
                    // query only captures nodes of the correct type
                    match capture.index {
                        #(#capture_idxs => Self::Capture::#capture_variant_idents(<#capture_node_types as type_sitter_lib::TypedNode<'tree>>::from_unchecked(capture.node)),)*
                    }
                }
            }

            #[automatically_derived]
            impl<'cursor, 'tree> #query_match_ident<'cursor, 'tree> {
                #(#capture_methods)*
            }

            #[automatically_derived]
            impl<'cursor, 'tree> TypedQueryMatch<'cursor, 'tree> for #query_match_ident<'cursor, 'tree> {
                type Query = #def_ident;

                #[inline]
                fn query(&self) -> &'static Self::Query {
                    #def_ident
                }

                #[inline]
                fn raw(&self) -> &tree_sitter::QueryMatch<'cursor, 'tree> {
                    &self.0
                }

                #[inline]
                fn into_raw(self) -> tree_sitter::QueryMatch<'cursor, 'tree> {
                    self.0
                }
            }

            #[automatically_derived]
            impl<'tree> #query_capture_ident<'tree> {
                #(#capture_variant_extract_methods)*
            }

            #[automatically_derived]
            impl<'tree> TypedQueryCapture<'tree> for #query_capture_ident<'tree> {
                type Query = #def_ident;

                #[inline]
                fn query(&self) -> &'static Self::Query {
                    #def_ident
                }

                #[inline]
                fn to_raw(&self) -> tree_sitter::QueryCapture<'tree> {
                    match self {
                        #(Self::#capture_variant_idents(node) => tree_sitter::QueryCapture {
                            index: #capture_idxs as u32,
                            node: *node.node()
                        },)*
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
        let capture_node_type = captured_sexp.node_type(false).print(&capture_variant_name, nodes, tree_sitter, anon_unions);

        let capture_quantifier = ts_query.capture_quantifiers(capture_idx).iter().copied()
            .reduce(CaptureQuantifierExt::union)
            .unwrap_or(CaptureQuantifier::Zero);
        let captured_type = capture_quantifier.print_type(&capture_node_type);
        let captured_nonempty_iterator_doc = capture_quantifier.print_nonempty_iterator_doc();
        let captured_expr = capture_quantifier.print_expr(&quote! {
            // SAFETY: Query only captures nodes of the correct type
            unsafe { self.nodes_for_capture_ix(capture_idx).map(TypedNode::from_unchecked) }
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
            pub fn #capture_method_ident(&self) -> #captured_type {
                #captured_expr
            }
        };
        let capture_variant_extract_method_doc = concat_doc!("Try to interpret this capture as a `", capture_name, "`");
        let capture_variant_extract_method = quote! {
            #[doc = #capture_variant_extract_method_doc]
            #full_capture_documentation
            #[inline]
            pub fn #capture_method_ident(&self) -> Option<#capture_node_type> {
                match self {
                    Self::#capture_variant_ident(node) => Some(node),
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

impl<'tree> SExp<'tree> {
    fn node_type(&self, is_head: bool) -> SExpNodeType {
        match self {
            SExp::Atom { atom, .. } => atom.node_type(is_head),
            SExp::Group { group_type, items, .. } => match group_type {
                GroupType::Paren => match items.get(0) {
                    None => panic!("empty paren group is not in a valid tree-sitter query"),
                    Some(item) => item.node_type(true)
                },
                GroupType::Bracket => items.iter().map(|item| item.node_type(is_head)).collect()
            }
        }
    }
}

impl<'tree> Atom<'tree> {
    fn node_type(&self, is_head: bool) -> SExpNodeType {
        match self {
            Atom::Wildcard => SExpNodeType::Untyped { is_named: is_head },
            Atom::Anchor => panic!("capturing an anchor is not in a valid tree-sitter query"),
            Atom::Field { .. } => panic!("capturing a field is not in a valid tree-sitter query"),
            Atom::Ident { name } => SExpNodeType::from(NodeName::new(name.to_string(), true)),
            Atom::String { content } => SExpNodeType::from(NodeName::new(content.to_string(), false)),
            Atom::Negation { .. } => SExpNodeType::Untyped { is_named: true },
            Atom::Capture { name } => panic!("capturing a capture is not in a valid tree-sitter query (captured capture name = {})", name),
            Atom::Predicate { name } => panic!("capturing a predicate is not in a valid tree-sitter query (captured predicate name = {})", name)
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
            CaptureQuantifier::ZeroOrMore => quote! { impl Iterator<Item=#inner_type> },
            CaptureQuantifier::One => quote! { #inner_type },
            CaptureQuantifier::OneOrMore => quote! { impl Iterator<Item=#inner_type> },
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