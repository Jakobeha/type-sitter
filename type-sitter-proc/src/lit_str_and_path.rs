use std::path::PathBuf;
use syn::{LitStr, Path, Token};
use syn::parse::{Parse, ParseStream};

/// Literal string, comma, path
#[allow(dead_code)]
pub struct LitStrAndPath {
    lit_str: LitStr,
    pub lit_str_path_buf: PathBuf,
    comma: Token![,],
    pub path: Path,
}

impl Parse for LitStrAndPath {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let lit_str: LitStr = input.parse()?;
        let comma: Token![,] = input.parse()?;
        let path: Path = input.parse()?;
        let lit_str_path_buf = PathBuf::from(lit_str.value());
        Ok(Self {
            lit_str,
            lit_str_path_buf,
            comma,
            path,
        })
    }
}