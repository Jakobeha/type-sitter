use indexmap::IndexMap;
use proc_macro2::TokenStream;
use crate::types::AnonUnionId;

/// Generated AST tokens from calling [NodeType::print] on a single instance or each element of a
/// collection.
///
/// We can't just collect the output of [NodeType::print] into a [TokenStream] because some
/// declarations go in specific submodules (`unnamed`, `anon_unions`), and also we don't want
/// duplicate definitions of the anonymous unions.
#[derive(Default)]
pub struct GeneratedNodeTokens {
    /// Toplevel declaration tokens
    pub toplevel: TokenStream,
    /// Tokens for the `unnamed` submodule
    pub unnamed: TokenStream,
    /// Anonymous unions and tokens for the `anon_union` submodule
    pub anon_unions: AnonUnions,
}

pub type AnonUnions = IndexMap<AnonUnionId, TokenStream>;

impl GeneratedNodeTokens {
    /// Empty instance
    pub fn new() -> Self {
        Self::default()
    }
}

impl Extend<GeneratedNodeTokens> for GeneratedNodeTokens {
    fn extend<T: IntoIterator<Item = GeneratedNodeTokens>>(&mut self, iter: T) {
        for x in iter {
            self.toplevel.extend(x.toplevel);
            self.unnamed.extend(x.unnamed);
            self.anon_unions.extend(x.anon_unions);
        }
    }
}

impl FromIterator<GeneratedNodeTokens> for GeneratedNodeTokens {
    fn from_iter<T: IntoIterator<Item = GeneratedNodeTokens>>(iter: T) -> Self {
        let mut this = Self::new();
        this.extend(iter);
        this
    }
}