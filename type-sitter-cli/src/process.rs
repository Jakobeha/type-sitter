use crate::args::{Args, InOutPair, InputType};
use crate::errors::Error;
use crate::path_utils::{language_name, write};
use crate::{errors, path_utils};
use std::fs::{create_dir, remove_dir_all};
use std::path::{Path, PathBuf};
use type_sitter_gen::super_nodes;

/** Process `item` configured with the other arguments, overwriting old data if it exists. */
pub fn reprocess(item: &InOutPair, args: &Args, use_yak_sitter: bool) -> errors::Result<()> {
    // Get input type
    let input_type = args
        .input_type
        .map_or_else(|| InputType::infer(&item.input), Ok)?;
    // Get language
    let language_dir = args
        .language_dir
        .clone()
        .or_else(|| input_type.infer_language_dir(&item.input));
    // Get output path
    let output_name = item
        .output
        .clone()
        .or_else(|| {
            language_dir
                .as_ref()
                .map(|l| PathBuf::from(language_name(l)))
        })
        .ok_or(Error::CouldntInferOutputName)?;
    let mut output_path = args.output_dir.join(output_name);
    if !input_type.is_output_a_dir() {
        output_path.set_extension("rs");
    }

    // Process
    do_reprocess(
        &item.input,
        &output_path,
        input_type,
        language_dir.as_deref(),
        use_yak_sitter,
    )
}

fn do_reprocess(
    input_path: &Path,
    output_path: &Path,
    input_type: InputType,
    language_dir: Option<&Path>,
    use_yak_sitter: bool,
) -> errors::Result<()> {
    match input_type {
        InputType::NodeTypes => write(
            &output_path,
            type_sitter_gen::generate_nodes(input_path)?.collapse(),
        )?,
        InputType::Query => {
            let language_dir = language_dir.ok_or(Error::CouldntInferLanguage)?;
            write(
                &output_path,
                type_sitter_gen::generate_queries(
                    input_path,
                    language_dir,
                    &super_nodes(),
                    use_yak_sitter,
                )?
                .collapse(&super_nodes()),
            )?;
        }
        InputType::LanguageRoot => {
            // Remove old dir.
            // For safety, we only remove if it contains all rust files.
            if output_path.exists() {
                if !path_utils::is_dir_of_only_rust_files(&output_path) {
                    return Err(Error::CodegenDirNotOnlyRustFiles);
                }
                remove_dir_all(&output_path)
                    .map_err(Error::io("removing old language codegen directory"))?;
            }

            create_dir(&output_path).map_err(Error::io("creating language codegen directory"))?;
            std::fs::write(
                &output_path.join("mod.rs"),
                format!(
                    r#"
//! Generated node and query wrappers for {}
pub mod nodes;
pub mod queries;
            "#,
                    language_name(input_path)
                ),
            )
            .map_err(Error::io("creating language codegen mod.rs"))?;
            do_reprocess(
                &input_path.join("src/node-types.json"),
                &output_path.join("nodes.rs"),
                InputType::NodeTypes,
                language_dir,
                use_yak_sitter,
            )
            .map_err(|e| e.nested("node types"))?;
            do_reprocess(
                &input_path.join("queries"),
                &output_path.join("queries.rs"),
                InputType::Query,
                language_dir,
                use_yak_sitter,
            )
            .map_err(|e| e.nested("queries"))?;
        }
    }
    Ok(())
}
