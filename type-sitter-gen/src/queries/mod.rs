/// Print query
mod print;
/// Dynamically-load a [tree_sitter::Language] to create [Query]s
mod dyload_language;
/// Tree-sitter query s-expression dialect
mod sexp;
mod sexp_node_type;
mod generated_tokens;

use crate::mk_syntax::ident;
use crate::queries::dyload_language::dyload_language;
use crate::queries::sexp::SExpSeq;
use crate::{make_valid, parse_node_type_map, type_sitter, type_sitter_raw, Error, PrintCtx};
use convert_case::{Case, Casing};
pub use generated_tokens::*;
use quote::quote;
use std::fs::{read_dir, read_to_string};
use std::path::Path;
use tree_sitter::Query;

/// Generate source code (tokens) of wrappers for queries, and the generated code will refer to the
/// provided modules instead of `type_sitter::raw` and `type_sitter` respectively.
///
/// # Parameters
/// - `path`: Path to the queries. Must point to a `.scm` or directory of `.scm` files. If a
///   directory, this function will generate submodules for each `.scm`.
/// - `language_path`: path to the tree-sitter language module, where the [tree_sitter::Language]
///   will be dynamically loaded. It must also contain `src/node-types.json`.
/// - `nodes`: Path to the crate with the typed node wrappers. Typically [crate::super_nodes]
/// - `use_yak_sitter`: Whether to generate queries that depend on the `yak_sitter` feature.
///
/// # Example
///
/// ```no_run
/// use type_sitter_gen::{generate_queries, super_nodes};
///
/// fn main() {
///     println!("{}", generate_queries(
///         "vendor/tree-sitter-typescript/queries/tags.scm",
///         "vendor/tree-sitter-typescript",
///         &super_nodes(),
///         false,
///     ).unwrap().into_string());
///     println!("{}", generate_queries(
///         "vendor/tree-sitter-rust/queries",
///         "vendor/tree-sitter-rust",
///         &super_nodes(),
///         false,
///     ).unwrap().into_string());
/// }
/// ```
pub fn generate_queries(
    path: impl AsRef<Path>,
    language_path: impl AsRef<Path>,
    nodes: &syn::Path,
    use_yak_sitter: bool,
) -> Result<GeneratedQueryTokens, Error> {
    generate_queries_with_custom_module_paths(
        path,
        language_path,
        nodes,
        use_yak_sitter,
        &type_sitter_raw(),
        &type_sitter(),
    )
}

/// Generate source code (tokens) of wrappers for queries.
///
/// # Parameters
/// - `path`: Path to the queries. Must point to a `.scm` or directory of `.scm` files. If a
///   directory, this function will generate submodules for each `.scm`.
/// - `language_path`: path to the tree-sitter language module, where the [tree_sitter::Language]
///   will be dynamically loaded. It must also contain `src/node-types.json`.
/// - `nodes`: Path to the crate with the typed node wrappers. Typically [crate::super_nodes]
/// - `use_yak_sitter`: Whether to generate queries that depend on the `yak_sitter` feature.
/// - `tree_sitter`: Path to the crate with the tree-sitter API. In [`generate_nodes`] this is
///   [`type_sitter_raw`] but you can provide something else, like the re-exported [`tree_sitter`]
///   or [`yak_sitter`] directly.
/// - `type_sitter_lib`: Path to the crate with the type-sitter API. In [`generate_nodes`] this is
///   [`type_sitter`] but you can provide something else, like the re-exported [`type_sitter_lib`]
///   directly.
///
/// # Example
///
/// ```no_run
/// use type_sitter_gen::{generate_queries_with_custom_module_paths, super_nodes, tree_sitter, type_sitter_lib};
///
/// fn main() {
///     println!("{}", generate_queries_with_custom_module_paths(
///         "vendor/tree-sitter-typescript/queries/tags.scm",
///         "vendor/tree-sitter-typescript",
///         &super_nodes(),
///         false,
///         &tree_sitter(),
///         &type_sitter_lib(),
///     ).unwrap().into_string());
///     println!("{}", generate_queries_with_custom_module_paths(
///         "vendor/tree-sitter-rust/queries",
///         "vendor/tree-sitter-rust",
///         &super_nodes(),
///         false,
///         &tree_sitter(),
///         &type_sitter_lib(),
///     ).unwrap().into_string());
/// }
/// ```
pub fn generate_queries_with_custom_module_paths(
    path: impl AsRef<Path>,
    language_path: impl AsRef<Path>,
    nodes: &syn::Path,
    use_yak_sitter: bool,
    tree_sitter: &syn::Path,
    type_sitter_lib: &syn::Path,
) -> Result<GeneratedQueryTokens, Error> {
    let language_path = language_path.as_ref();
    let node_types_path = language_path.join("src/node-types.json");
    let all_types = parse_node_type_map(node_types_path)?;

    _generate_queries(
        path,
        language_path,
        nodes,
        use_yak_sitter,
        PrintCtx {
            all_types: &all_types,
            tree_sitter,
            type_sitter_lib
        }
    )
}

fn _generate_queries(
    path: impl AsRef<Path>,
    language_path: impl AsRef<Path>,
    nodes: &syn::Path,
    use_yak_sitter: bool,
    ctx: PrintCtx,
) -> Result<GeneratedQueryTokens, Error>{
    let path = path.as_ref();
    if path.is_dir() {
        _generate_queries_from_dir(path, language_path, nodes, use_yak_sitter, ctx)
    } else {
        _generate_query_from_file(path, language_path, &[], &[], &[], nodes, use_yak_sitter, ctx)
    }
}

fn _generate_queries_from_dir(
    path: impl AsRef<Path>,
    language_path: impl AsRef<Path>,
    nodes: &syn::Path,
    use_yak_sitter: bool,
    ctx: PrintCtx,
) -> Result<GeneratedQueryTokens, Error> {
    let path = path.as_ref();
    let language_path = language_path.as_ref();

    let mut entries = read_dir(path)?.collect::<Result<Vec<_>, _>>()?;
    entries.sort_by_key(|e| e.path());

    let mut queries = GeneratedQueryTokens::new();

    for entry in entries {
        let entry_path = entry.path();
        let entry_is_dir = entry.metadata()?.is_dir();

        if entry_is_dir || has_extension(&entry_path, "scm") {
            let entry_name = entry_path.file_stem().unwrap().to_string_lossy();

            let entry_code = _generate_queries(&entry_path, language_path, nodes, use_yak_sitter, ctx)?;

            match entry_is_dir {
                false => queries.append(entry_code),
                true => {
                    let entry_ident = ident!(make_valid(&*entry_name), "query module name (subfolder name)")?;
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

    Ok(queries)
}

/// Same as [`generate_queries`], but `path` must point to a file and you can specify patterns and
/// captures to skip.
pub fn generate_query_from_file(
    path: impl AsRef<Path>,
    language_path: impl AsRef<Path>,
    disabled_patterns: &[&str],
    disabled_capture_names: &[&str],
    disabled_capture_idxs: &[usize],
    nodes: &syn::Path,
    use_yak_sitter: bool,
) -> Result<GeneratedQueryTokens, Error> {
    generate_query_from_file_with_custom_module_paths(
        path,
        language_path,
        disabled_patterns,
        disabled_capture_names,
        disabled_capture_idxs,
        nodes,
        use_yak_sitter,
        &type_sitter_raw(),
        &type_sitter(),
    )
}

/// Same as [`generate_queries_with_custom_module_paths`], but `path` must point to a file and you
/// can specify patterns and captures to skip.
pub fn generate_query_from_file_with_custom_module_paths(
    path: impl AsRef<Path>,
    language_path: impl AsRef<Path>,
    disabled_patterns: &[&str],
    disabled_capture_names: &[&str],
    disabled_capture_idxs: &[usize],
    nodes: &syn::Path,
    use_yak_sitter: bool,
    tree_sitter: &syn::Path,
    type_sitter_lib: &syn::Path,
) -> Result<GeneratedQueryTokens, Error> {
    let language_path = language_path.as_ref();
    let node_types_path = language_path.join("src/node-types.json");
    let all_types = parse_node_type_map(node_types_path)?;

    _generate_query_from_file(
        path,
        language_path,
        disabled_patterns,
        disabled_capture_names,
        disabled_capture_idxs,
        nodes,
        use_yak_sitter,
        PrintCtx {
            all_types: &all_types,
            tree_sitter,
            type_sitter_lib
        }
    )
}

fn _generate_query_from_file(
    path: impl AsRef<Path>,
    language_path: impl AsRef<Path>,
    disabled_patterns: &[&str],
    disabled_capture_names: &[&str],
    disabled_capture_idxs: &[usize],
    nodes: &syn::Path,
    use_yak_sitter: bool,
    ctx: PrintCtx,
) -> Result<GeneratedQueryTokens, Error> {
    let path = path.as_ref();
    let language_path = language_path.as_ref();
    let language_name = language_name(language_path)?;
    let language_ident = ident!(language_name, "language name")?;
    let language = dyload_language(language_path)?;
    let def_ident = ident!(
        make_valid(&path.file_stem().and_then(|f| f.to_str()).unwrap_or("�").to_case(Case::Pascal)),
        "query name (filename)"
    )?;
    let query_str = read_to_string(path)?;
    let ts_query = Query::new(&language, &query_str)?;
    let query = SExpSeq::try_from(query_str.as_str()).unwrap_or_else(|err| {
        panic!(
            "query was parsed by tree-sitter but can't be parsed by type-sitter: {} ({})\n\n{}",
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
        ctx,
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
