use std::path::PathBuf;
use syn::{LitStr, Token};
use syn::parse::{Parse, ParseStream};
use crate::generate_nodes_args::normalize;

/// Args to [crate::generate_queries]
#[allow(dead_code)]
pub struct GenerateQueriesArgs {
    path_str: LitStr,
    pub path: PathBuf,
    comma: Token![,],
    language_path_str: LitStr,
    pub language_path: PathBuf,
    comma2: Token![,],
    pub nodes: syn::Path,
    comma3: Token![,],
    pub use_wrapper: syn::LitBool,
    comma4: Option<Token![,]>,
    pub tree_sitter: Option<syn::Path>,
}

impl Parse for GenerateQueriesArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let path_str: LitStr = input.parse()?;
        let comma: Token![,] = input.parse()?;
        let language_path_str: LitStr = input.parse()?;
        let comma2: Token![,] = input.parse()?;
        let nodes: syn::Path = input.parse()?;
        let comma3: Token![,] = input.parse()?;
        let use_wrapper: syn::LitBool = input.parse()?;
        let (comma4, tree_sitter): (Option<Token![,]>, Option<syn::Path>) = match input.is_empty() {
            false => {
                let comma4: Token![,] = input.parse()?;
                let path2: syn::Path = input.parse()?;
                (Some(comma4), Some(path2))
            }
            true => (None, None),
        };
        let path = normalize(path_str.value());
        let language_path = normalize(language_path_str.value());
        Ok(Self {
            path_str,
            path,
            comma,
            language_path_str,
            language_path,
            comma2,
            nodes,
            comma3,
            use_wrapper,
            comma4,
            tree_sitter,
        })
    }
}