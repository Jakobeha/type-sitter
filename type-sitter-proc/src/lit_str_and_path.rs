use std::path::PathBuf;
use syn::{LitStr, Token};
use syn::parse::{Parse, ParseStream};

/// Literal string, comma, path
#[allow(dead_code)]
pub struct LitStrAndPath {
    lit_str: LitStr,
    pub lit_str_path_buf: PathBuf,
    comma: Token![,],
    pub path: syn::Path,
}

impl Parse for LitStrAndPath {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let lit_str: LitStr = input.parse()?;
        let comma: Token![,] = input.parse()?;
        let path: syn::Path = input.parse()?;
        let lit_str_path_buf = normalize(lit_str.value());
        Ok(Self {
            lit_str,
            lit_str_path_buf,
            comma,
            path,
        })
    }
}

/// Convert into PathBuf and prepend manifest
fn normalize(path: impl AsRef<std::path::Path>) -> PathBuf {
    let mut path = path.as_ref().to_owned();
    if let Ok(cargo_manifest) = std::env::var("CARGO_MANIFEST_DIR") {
        path = PathBuf::from(cargo_manifest).join(path);
    }
    path
}