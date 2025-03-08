use crate::generate_nodes_args::normalize;
use std::path::PathBuf;
use syn::parse::{Parse, ParseStream};
use syn::{LitStr, Token};

/// Args to [`crate::generate_queries`]
#[allow(dead_code)]
pub struct GenerateQueriesArgs {
    path_str: LitStr,
    pub path: PathBuf,
    comma: Token![,],
    language_path_str: LitStr,
    pub language_path: PathBuf,
    comma2: Token![,],
    pub nodes: syn::Path,
}

impl Parse for GenerateQueriesArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let path_str: LitStr = input.parse()?;
        let comma: Token![,] = input.parse()?;
        let language_path_str: LitStr = input.parse()?;
        let comma2: Token![,] = input.parse()?;
        let nodes: syn::Path = input.parse()?;
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
        })
    }
}
