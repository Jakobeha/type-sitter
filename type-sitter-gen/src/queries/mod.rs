/// Print query
mod print;
/// Dynamically-load a [tree_sitter::Language] to create [Query]s
mod dyload_language;
/// Tree-sitter query s-expression dialect
mod sexp;
mod sexp_node_type;
mod generated_tokens;

use std::fs::read_to_string;
use std::path::Path;
use convert_case::{Case, Casing};
use quote::quote;
use tree_sitter::Query;
use crate::Error;
use crate::mk_syntax::ident;
use crate::queries::dyload_language::dyload_language;
use crate::queries::sexp::SExpSeq;
pub use generated_tokens::GeneratedQueryTokens;

/// Generate source code (tokens) of wrappers for queries.
///
/// # Parameters
/// - `path`: Path to the queries. Must point to a `.scm` or directory of `.scm` files. If a
///   directory, this function will generate submodules for each `.scm`.
/// - `language_path`: path to the tree-sitter language module, where the [tree_sitter::Language]
///   will be dynamically loaded
/// - `nodes`: Path to the crate with the typed node wrappers. Typically [crate::super_nodes]
/// - `use_yak_sitter`: Whether to use `yak_sitter` or `tree_sitter`
/// - `tree_sitter`: Path to the crate with the tree-sitter API. Typically [crate::tree_sitter] if
///    `use_yak_sitter` is false, or [crate::type_sitter_lib_wrapper] if `use_yak_sitter` is true
///
/// # Example
///
/// ```no_run
/// use type_sitter_gen::{generate_queries, super_nodes, tree_sitter};
///
/// fn main() {
///     println!("{}", generate_queries("vendor/tree-sitter-typescript/queries/tags.scm", "vendor/tree-sitter-typescript", &super_nodes(), false, &tree_sitter()).unwrap());
///     println!("{}", generate_queries("vendor/tree-sitter-rust/queries", "vendor/tree-sitter-rust", &super_nodes(), false, &tree_sitter()).unwrap());
/// }
/// ```
pub fn generate_queries(
    path: impl AsRef<Path>,
    language_path: impl AsRef<Path>,
    nodes: &syn::Path,
    use_yak_sitter: bool,
    tree_sitter: &syn::Path
) -> Result<GeneratedQueryTokens, Error> {
    let path = path.as_ref();
    if path.is_dir() {
        generate_queries_from_dir(path, language_path, nodes, use_yak_sitter, tree_sitter)
    } else {
        generate_query_from_file(path, language_path, &[], &[], &[], nodes, use_yak_sitter, tree_sitter)
    }
}

/// Generate source code (tokens) of wrappers for queries.
///
/// # Parameters
/// - `path`: Path to the queries. Must point to directory of `.scm` files. This function will
///   generate submodules for each `.scm`.
/// - `language_path`: path to the tree-sitter language module, where the [tree_sitter::Language]
///   will be dynamically loaded
/// - `nodes`: Path to the crate with the typed node wrappers. Typically [crate::super_nodes]
/// - `use_yak_sitter`: Whether to use `yak_sitter` or `tree_sitter`
/// - `tree_sitter`: Path to the crate with the tree-sitter API. Typically [crate::tree_sitter] if
///   `use_yak_sitter` is false, or [crate::type_sitter_lib_wrapper] if `use_yak_sitter` is true
///
/// # Example
///
/// ```no_run
/// use type_sitter_gen::{generate_queries_from_dir, super_nodes, tree_sitter};
///
/// fn main() {
///     println!("{}", generate_queries_from_dir("vendor/tree-sitter-rust/queries", "vendor/tree-sitter-rust", &super_nodes(), false, &tree_sitter()).unwrap());
/// }
/// ```
pub fn generate_queries_from_dir(
    path: impl AsRef<Path>,
    language_path: impl AsRef<Path>,
    nodes: &syn::Path,
    use_yak_sitter: bool,
    tree_sitter: &syn::Path,
) -> Result<GeneratedQueryTokens, Error> {
    let path = path.as_ref();
    let language_path = language_path.as_ref();
    let mut queries = GeneratedQueryTokens::new();
    for entry in std::fs::read_dir(path)? {
        let entry = entry?;
        let entry_path = entry.path();
        let entry_is_dir = entry.metadata()?.is_dir();
        if entry_is_dir || has_extension(&entry_path, "scm") {
            let entry_name = entry_path.file_stem().unwrap().to_string_lossy();
            let entry_code = generate_queries(&entry_path, language_path, nodes, use_yak_sitter, tree_sitter)?;
            match entry_is_dir {
                false => queries.append(entry_code),
                true => {
                    let entry_ident = ident!(entry_name, "query module name (subfolder name)")?;
                    let entry_tokens = entry_code.collapse(nodes);
                    queries.append_tokens(quote! {
                        pub mod #entry_ident {
                            #entry_tokens
                        }
                    })
                }
            }
        }
    }
    return Ok(queries);
}

/// Generate source code (tokens) of a wrapper for a single query.
///
/// # Parameters
/// - `path`: Path to the query. Must point to a `.scm` file.
/// - `language_path`: path to the tree-sitter language module, where the [tree_sitter::Language]
///   will be dynamically loaded
/// - `disabled_patterns`: Patterns to disable. See [Query::disable_pattern].
/// - `disabled_capture_names`: List of capture names to ignore on the query
/// - `disabled_capture_idxs`: List of capture indices to ignore on the query (both these and
///   all indices with names in `disabled_capture_names` are disabled)
/// - `nodes`: Path to the crate with the typed node wrappers. Typically
///   [crate::super_nodes]
/// - `use_yak_sitter`: Whether to use `yak_sitter` or `tree_sitter`
/// - `tree_sitter`: Path to the crate with the tree-sitter API. Typically [tree_sitter] if
///    `use_yak_sitter` is false, or [crate::type_sitter_lib_wrapper] if `use_yak_sitter` is true
///
/// # Example
///
/// ```no_run
/// use type_sitter_gen::{generate_query_from_file, super_nodes, tree_sitter};
///
/// fn main() {
///     println!("{}", generate_query_from_file("vendor/tree-sitter-typescript/queries/tags.scm", "vendor/tree-sitter-typescript", &[], &[], &[], &super_nodes(), false, &tree_sitter()).unwrap());
/// }
/// ```
pub fn generate_query_from_file(
    path: impl AsRef<Path>,
    language_path: impl AsRef<Path>,
    disabled_patterns: &[&str],
    disabled_capture_names: &[&str],
    disabled_capture_idxs: &[usize],
    nodes: &syn::Path,
    use_yak_sitter: bool,
    tree_sitter: &syn::Path,
) -> Result<GeneratedQueryTokens, Error> {
    let path = path.as_ref();
    let language_path = language_path.as_ref();
    let language_name = language_name(language_path)?;
    let language_ident = ident!(language_name, "language name")?;
    let language = dyload_language(language_path)?;
    let def_ident = ident!(
        path.file_stem().and_then(|f| f.to_str()).unwrap_or("�").to_case(Case::Pascal),
        "query name (filename)"
    )?;
    let query_str = read_to_string(path)?;
    let ts_query = Query::new(language, &query_str)?;
    let query = SExpSeq::try_from(query_str.as_str()).unwrap_or_else(|err| {
        panic!(
            "query was already parsed by tree-sitter but can't be parsed by type-sitter: {} ({})\n\n{}",
            err,
            &query_str.as_str()[*err.span()],
            query_str
        )
    });
    let mut generated = GeneratedQueryTokens::new();
    let query_tokens = query.print(
        &query_str,
        ts_query,
        &def_ident,
        &language_ident,
        disabled_patterns,
        disabled_capture_names,
        disabled_capture_idxs,
        nodes,
        use_yak_sitter,
        tree_sitter,
        &mut generated.anon_unions,
    );
    generated.append_tokens(query_tokens);
    Ok(generated)
}

fn language_name(path: &Path) -> Result<String, Error> {
    let mut name = path.file_name()
        .and_then(|s| s.to_str())
        .ok_or(Error::IllegalTSLanguageSymbolName)?
        .replace("-", "_");
    if !name.starts_with("tree_sitter_") {
        name = format!("tree_sitter_{}", name);
    }
    Ok(name)
}

/// Check if the path has the given extension
fn has_extension(path: &Path, extension: &str) -> bool {
    path.extension().and_then(|e| e.to_str()) == Some(extension)
}
