use indexmap::IndexMap;
use proc_macro2::TokenStream;
use crate::node_types::types::{AnonUnionId, NodeModule};

/// Generated AST tokens from calling [NodeType::print] on a single instance or each element of a
/// collection.
///
/// We can't just collect the output of [NodeType::print] into a [TokenStream] because some
/// declarations go in specific submodules (`unnamed`, `symbols`, `anon_unions`), and also we don't want
/// duplicate definitions of the anonymous unions.
#[derive(Default)]
pub struct GeneratedNodeTokens {
    /// Toplevel declaration tokens
    pub toplevel: TokenStream,
    /// Tokens for the `unnamed` submodule
    pub unnamed: TokenStream,
    /// Tokens for the `symbols` submodule
    pub symbols: TokenStream,
    /// Anonymous unions and tokens for the `anon_union` submodule
    pub anon_unions: AnonUnions,
}

pub type AnonUnions = IndexMap<AnonUnionId, TokenStream>;

impl GeneratedNodeTokens {
    /// Empty instance
    pub fn new() -> Self {
        Self::default()
    }

    /// Append the tokens to the given module
    pub fn extend(&mut self, module: NodeModule, tokens: TokenStream) {
        match module {
            NodeModule::Toplevel => self.toplevel.extend(tokens),
            NodeModule::Unnamed => self.unnamed.extend(tokens),
            NodeModule::Symbols => self.symbols.extend(tokens),
        }
    }
}

impl Extend<GeneratedNodeTokens> for GeneratedNodeTokens {
    fn extend<T: IntoIterator<Item = GeneratedNodeTokens>>(&mut self, iter: T) {
        for x in iter {
            self.toplevel.extend(x.toplevel);
            self.unnamed.extend(x.unnamed);
            self.symbols.extend(x.symbols);
            self.anon_unions.extend(x.anon_unions);
        }
    }
}

impl FromIterator<GeneratedNodeTokens> for GeneratedNodeTokens {
    fn from_iter<T: IntoIterator<Item = GeneratedNodeTokens>>(iter: T) -> Self {
        let mut this = Self::new();
        Extend::<GeneratedNodeTokens>::extend(&mut this, iter);
        this
    }
}