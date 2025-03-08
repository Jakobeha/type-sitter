use crate::anon_unions::AnonUnions;
use crate::{pretty_print, NodeModule};
use proc_macro2::TokenStream;

/// Generated AST tokens from calling [`NodeType::print`] on a single instance or each element of a
/// collection.
///
/// We can't just collect the output of [`NodeType::print`] into a [`TokenStream`] because some
/// declarations go in specific submodules (`unnamed`, `symbols`, `anon_unions`), and also we don't want
/// duplicate definitions of the anonymous unions.
#[derive(Debug, Default, Clone)]
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

impl GeneratedNodeTokens {
    /// Empty instance
    pub fn new() -> Self {
        Self::default()
    }

    /// Append the tokens to the given module
    pub fn append_tokens(&mut self, module: NodeModule, tokens: TokenStream) {
        match module {
            NodeModule::Toplevel => self.toplevel.extend(tokens),
            NodeModule::Unnamed => self.unnamed.extend(tokens),
            NodeModule::Symbols => self.symbols.extend(tokens),
        }
    }

    /// Append the other's tokens into this
    pub fn append(&mut self, other: GeneratedNodeTokens) {
        self.toplevel.extend(other.toplevel);
        self.unnamed.extend(other.unnamed);
        self.symbols.extend(other.symbols);
        self.anon_unions.extend(other.anon_unions);
    }

    /// Convert into a pretty-printed string.
    ///
    /// Specifically, the code is formatted with
    /// [prettyplease](https://crates.io/crates/prettyplease).
    ///
    /// To convert not pretty-printed, use [`collapse`](Self::collapse) then
    /// [display](std::fmt::Display) the returned [`TokenStream`].
    pub fn into_string(self) -> String {
        pretty_print(&self.collapse())
    }
}

impl Extend<GeneratedNodeTokens> for GeneratedNodeTokens {
    fn extend<T: IntoIterator<Item = GeneratedNodeTokens>>(&mut self, iter: T) {
        for x in iter {
            self.append(x)
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
