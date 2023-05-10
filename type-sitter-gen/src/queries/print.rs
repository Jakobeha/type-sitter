use convert_case::{Case, Casing};
use proc_macro2::TokenStream;
use syn::Ident;
use quote::{format_ident, quote};
use tree_sitter::Query;
use crate::Error;
use crate::mk_syntax::{ident, lit_str};
use crate::names::sexp_name_to_rust_names;

/// Generate the AST for the typed wrapper of the given query.
///
/// # Parameters
/// - `query`: The query to generate the typed wrapper AST for
/// - `nodes`: Path to the crate with the typed node wrappers. Typically [type_sitter_gen::nodes]
/// - `tree_sitter`: Path to the `tree_sitter` crate. Typically either
///   [type_sitter_gen::tree_sitter] or [type_sitter_gen::type_sitter_lib_wrapper], but you can
///   provide a path to your own wrapper as well.
pub fn print_query(
    query: Query,
    query_str: &str,
    query_ident: &Ident,
    language_ident: &syn::Ident,
    disable_patterns: impl Iterator<Item=&str>,
    disable_captures: impl Iterator<Item=usize>,
    nodes: &syn::Path,
    tree_sitter: &syn::Path
) -> TokenStream {
    let language_name = language.to_string();
    let query_parse_error = lit_str(&format!(
        "query parsed at compile-time but failed at runtime. Is the language '{}' correct, and did \
        you use the same tree-sitter / {} version?",
        language_name, language_name
    ));
    let internal_query_ident = format_ident!("__{}", query_ident);
    let mk_internal_query_ident = format_ident!("__mk__{}", query_ident);
    let query_match_type_name_base = query_ident.to_string().from_case(Case::UpperSnake).to_case(Case::Pascal);
    let query_matches_ident = format_ident!("QueryMatches")
    let disable_patterns = disable_patterns.map(lit_str);

    let captures = query.capture_names().iter().map(|capture_name| {
        let (_, capture_method) = sexp_name_to_rust_names(capture_name);
        // These idents are valid because we get them from a names function
        let capture_method = ident!(capture_method, "capture name (capture method)").unwrap();
        let capture_name_str = lit_str(capture_name);
        quote! {
            pub fn #capture_method(&self) -> #nodes::#capture_name_ident {
                #nodes::#capture_name_ident::new(self.query(), #capture_name_str)
            }
        }
    });

    quote! {
        static #internal_query_ident: once_cell::race::OnceBox<#tree_sitter::Query> = once_cell::race::OnceBox::new();

        fn #mk_internal_query_ident() -> #tree_sitter::Query {
            let mut query = #tree_sitter::Query::new(
                #language_ident::language(),
                #query_str
            ).expect(#query_parse_error);
            #(query.disable_capture(#disable_captures);)*
            #(query.disable_pattern(#disable_patterns);)*
            query
        }

        #[doc = concat!("Typed version of the query:\n\n```sexp\n", query_str, "\n```")]
        #[allow(non_camel_case_types)]
        #[derive(Debug, Clone, Copy)]
        struct #query_ident;

        #[automatically_derived]
        impl #query_ident {

        }

        #[automatically_derived]
        impl TypedQuery for #query_ident {
            fn query_str(&self) -> &'static str {
                #query_str
            }

            fn query(&self) -> &'static #tree_sitter::Query {
                #internal_query_ident.get_or_init(#mk_internal_query_ident)
            }
        }
    }
}