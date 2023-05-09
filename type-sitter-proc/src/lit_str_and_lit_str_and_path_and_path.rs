use std::path::PathBuf;
use syn::{LitStr, Token};
use syn::parse::{Parse, ParseStream};
use crate::lit_str_and_path::normalize;

/// Literal string, comma, path
#[allow(dead_code)]
pub struct LitStrAndLitStrAndPathAndPath {
    lit_str: LitStr,
    pub lit_str_path_buf: PathBuf,
    comma: Token![,],
    lit_str2: LitStr,
    pub lit_str_path_buf2: PathBuf,
    comma2: Token![,],
    pub path: syn::Path,
    comma3: Token![,],
    pub path2: syn::Path,
}

impl Parse for LitStrAndLitStrAndPathAndPath {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let lit_str: LitStr = input.parse()?;
        let comma: Token![,] = input.parse()?;
        let lit_str2: LitStr = input.parse()?;
        let comma2: Token![,] = input.parse()?;
        let path: syn::Path = input.parse()?;
        let comma3: Token![,] = input.parse()?;
        let path2: syn::Path = input.parse()?;
        let lit_str_path_buf = normalize(lit_str.value());
        let lit_str_path_buf2 = normalize(lit_str2.value());
        Ok(Self {
            lit_str,
            lit_str_path_buf,
            comma,
            lit_str2,
            lit_str_path_buf2,
            comma2,
            path,
            comma3,
            path2,
        })
    }
}