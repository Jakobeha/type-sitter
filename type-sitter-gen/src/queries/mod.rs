/// Print query
mod print;
/// Dynamically-load a [Language] to create [Query]s
mod dyload_language;
/// Tree-sitter query s-expression dialect
mod sexp;

use std::fs::read_to_string;
use std::path::Path;
use convert_case::{Case, Casing};
use proc_macro2::TokenStream;
use quote::quote;
use syn::parse_quote;
use tree_sitter::Query;
use crate::Error;
use crate::mk_syntax::ident;
use crate::queries::dyload_language::dyload_language;
use crate::queries::print::print_query;
use crate::queries::sexp::SExpSeq;
use crate::node_types::generated_tokens::AnonUnions;

/// Generate source code (tokens) of wrappers for queries.
///
/// # Parameters
/// - `path`: Path to the queries. Must point to a `.scm` or directory of `.scm` files. If a
///   directory, this function will generate submodules for each `.scm`.
/// - `language_path`: path to the tree-sitter language module, where the [Language] will be
///   dynamically loaded
/// - `nodes`: Path to the crate with the typed node wrappers. Typically [type_sitter_gen::nodes]
/// - `use_wrapper`: Whether to use `tree_sitter_wrapper` or `tree_sitter`
///
/// # Example
///
/// ```no_run
/// use type_sitter_gen::generate_queries;
///
/// fn main() {
///     use type_sitter_gen::nodes;
///     println!("{}", generate_queries("vendor/tree-sitter-typescript/queries/tags.scm", "vendor/tree-sitter-typescript", &nodes(), false).unwrap());
///     println!("{}", generate_queries("vendor/tree-sitter-rust/queries", "vendor/tree-sitter-rust", &nodes(), false).unwrap());
/// }
/// ```
pub fn generate_queries(path: impl AsRef<Path>, language_path: impl AsRef<Path>, nodes: &syn::Path, use_wrapper: bool) -> Result<TokenStream, Error> {
    let path = path.as_ref();
    if path.is_dir() {
        generate_queries_from_dir(path, language_path, nodes, use_wrapper)
    } else {
        generate_query_from_file(path, language_path, &[], &[], nodes, use_wrapper)
    }
}

/// Generate source code (tokens) of wrappers for queries.
///
/// # Parameters
/// - `path`: Path to the queries. Must point to directory of `.scm` files. This function will
///   generate submodules for each `.scm`.
/// - `language_path`: path to the tree-sitter language module, where the [Language] will be
///   dynamically loaded
/// - `nodes`: Path to the crate with the typed node wrappers. Typically [type_sitter_gen::nodes]
/// - `use_wrapper`: Whether to use `tree_sitter_wrapper` or `tree_sitter`
///
/// # Example
///
/// ```no_run
/// use type_sitter_gen::generate_queries_from_dir;
///
/// fn main() {
///     use type_sitter_gen::nodes;
///     println!("{}", generate_queries_from_dir("vendor/tree-sitter-rust/queries", "vendor/tree-sitter-rust", &nodes(), false).unwrap());
/// }
/// ```
pub fn generate_queries_from_dir(path: impl AsRef<Path>, language_path: impl AsRef<Path>, nodes: &syn::Path, use_wrapper: bool) -> Result<TokenStream, Error> {
    let path = path.as_ref();
    let language_path = language_path.as_ref();
    let mut queries = TokenStream::new();
    for entry in std::fs::read_dir(path)? {
        let entry = entry?;
        let entry_path = entry.path();
        let entry_is_dir = entry.metadata()?.is_dir();
        if entry_is_dir || has_extension(&entry_path, "scm") {
            let entry_name = entry_path.file_stem().unwrap().to_string_lossy();
            let entry_code = generate_queries(entry_path, language_path, nodes, use_wrapper)?;
            queries.extend(match entry_is_dir {
                false => entry_code,
                true => {
                    let entry_ident = ident!(&entry_name, "query module name (subfolder name)")?;
                    quote! {
                        pub mod #entry_ident {
                            #entry_code
                        }
                    }
                }
            }
            );
        }
    }
    return Ok(queries);
}

/// Generate source code (tokens) of a wrapper for a single query.
///
/// # Parameters
/// - `path`: Path to the query. Must point to a `.scm` file.
/// - `language_path`: path to the tree-sitter language module, where the [Language] will be
///   dynamically loaded
/// - `disabled_patterns`: Patterns to disable. See [Query::disable_pattern].
/// - `disabled_captures`: Captures to disable. See [Query::disable_capture].
/// - `nodes`: Path to the crate with the typed node wrappers. Typically [type_sitter_gen::nodes]
/// - `use_wrapper`: Whether to use `tree_sitter_wrapper` or `tree_sitter`
///
/// # Example
///
/// ```no_run
/// use type_sitter_gen::generate_query_from_file;
///
/// fn main() {
///     use type_sitter_gen::nodes;
///     println!("{}", generate_query_from_file("vendor/tree-sitter-typescript/queries/tags.scm", "vendor/tree-sitter-typescript", &[], &[], &nodes(), false).unwrap());
/// }
/// ```
pub fn generate_query_from_file(
    path: impl AsRef<Path>,
    language_path: impl AsRef<Path>,
    disabled_patterns: &[&str],
    disabled_captures: &[usize],
    nodes: &syn::Path,
    use_wrapper: bool
) -> Result<TokenStream, Error> {
    let path = path.as_ref();
    let language_path = language_path.as_ref();
    let language_name = language_name(language_path);
    let language_ident = ident!(language_name, "language name")?;
    let language = dyload_language(language_path)?;
    let def_ident = ident!(
        path.file_name().and_then(|f| f.to_str()).unwrap_or("�").to_case(Case::Pascal),
        "query name (filename)"
    )?;
    let query_str = read_to_string(path)?;
    let ts_query = Query::new(language, &query_str)?;
    let query = SExpSeq::try_from(&query_str)
        .expect("query was already parsed by tree-sitter but can't be parsed by type-sitter");
    let mut anon_unions = AnonUnions::new();
    let query = query.print(
        ts_query,
        &def_ident,
        &language_ident,
        disabled_patterns,
        disabled_captures,
        nodes,
        use_wrapper,
        &mut anon_unions,
    );
    let anon_unions = anon_unions.into_values().collect::<TokenStream>();
    let anon_unions = if anon_unions.is_empty() {
        quote! {}
    } else {
        quote! {
            pub mod anon_unions {
                #[allow(unused_imports)]
                use #nodes::*;
                #anon_unions
            }
        }
    };
    Ok(quote! {
        #query,
        #anon_unions
    })
}

fn language_name(path: &Path) -> String {
    path.file_name()
        .and_then(|s| s.to_str())
        .ok_or(Error::UnknownTSLanguageSymbolName)?
        .replace("-", "_")
}

/// Check if the path has the given extension
fn has_extension(path: &Path, extension: &str) -> bool {
    path.extension().and_then(|e| e.to_str()) == Some(extension)
}

/// = `parse_quote!(super::nodes)`. The default path to the typed nodes crate from the queries crate.
pub fn nodes() -> syn::Path {
    parse_quote!(super::nodes)
}