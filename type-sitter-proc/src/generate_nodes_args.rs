use std::path::PathBuf;
use syn::parse::{Parse, ParseStream};
use syn::LitStr;

/// Args to [`crate::generate_nodes`]
#[allow(dead_code)]
pub struct GenerateNodesArgs {
    path_str: LitStr,
    pub path: PathBuf,
}

impl Parse for GenerateNodesArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let path_str: LitStr = input.parse()?;
        let path = normalize(path_str.value());
        Ok(Self { path_str, path })
    }
}

/// Convert into PathBuf and prepend manifest
pub(crate) fn normalize(path: impl AsRef<std::path::Path>) -> PathBuf {
    let mut path = path.as_ref().to_owned();
    if let Ok(cargo_manifest) = std::env::var("CARGO_MANIFEST_DIR") {
        path = PathBuf::from(cargo_manifest).join(path);
    }
    path
}