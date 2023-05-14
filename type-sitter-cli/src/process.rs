use std::path::{Path, PathBuf};
use std::fs::create_dir;
use type_sitter_gen::nodes;
use crate::args::{Args, InOutPair, InputType};
use crate::errors;
use crate::errors::Error;
use crate::path_utils::{language_name, write};

pub fn process(item: &InOutPair, args: &Args, use_wrapper: bool, tree_sitter: &syn::Path) -> errors::Result<()> {
    // Get input type
    let input_type = args.input_type
        .map_or_else(|| InputType::infer(&item.input), Ok)?;
    // Get language
    let language_dir = args.language_dir.clone()
        .or_else(|| input_type.infer_language_dir(&item.input));
    // Get output path
    let output_name = item.output.clone()
        .or_else(|| language_dir.as_ref().map(|l| PathBuf::from(language_name(l))))
        .ok_or(Error::CouldntInferOutputName)?;
    let mut output_path = args.output_dir.join(output_name);
    if !input_type.is_output_a_dir() {
        output_path.set_extension("rs");
    }
    do_process(&item.input, &output_path, input_type, language_dir.as_deref(), use_wrapper, tree_sitter)
}

fn do_process(
    input_path: &Path,
    output_path: &Path,
    input_type: InputType,
    language_dir: Option<&Path>,
    use_wrapper: bool,
    tree_sitter: &syn::Path
) -> errors::Result<()> {
    match input_type {
        InputType::NodeTypes => {
            write(&output_path, type_sitter_gen::generate_nodes(input_path, &tree_sitter)?)?
        }
        InputType::Query => {
            let language_dir = language_dir.ok_or(Error::CouldntInferLanguage)?;
            write(&output_path, type_sitter_gen::generate_queries(input_path, language_dir, &nodes(), use_wrapper, &tree_sitter)?)?;
        }
        InputType::LanguageRoot => {
            create_dir(&output_path)?;
            std::fs::write(&output_path.join("mod.rs"), format!(r#"
//! Generated node and query wrappers for {}
pub mod nodes;
pub mod queries;
            "#, language_name(input_path))).map_err(|e| Error::from(e).nested("root"))?;
            do_process(
                &input_path.join("src/node-types.json"),
                &output_path.join("nodes.rs"),
                InputType::NodeTypes,
                language_dir,
                use_wrapper,
                tree_sitter
            ).map_err(|e| e.nested("node types"))?;
            do_process(
                &input_path.join("queries"),
                &output_path.join("queries.rs"),
                InputType::Query,
                language_dir,
                use_wrapper,
                tree_sitter
            ).map_err(|e| e.nested("queries"))?;
        }
    }
    Ok(())
}
