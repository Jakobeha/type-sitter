use crate::anon_unions::AnonUnions;
use crate::{pretty_print, super_nodes};
use proc_macro2::TokenStream;

/// Generated AST tokens from calling [SExpSeq::print] on a single instance or each element of a
/// collection.
///
/// We can't just collect the output of [SExpSeq::print] into a [TokenStream] because anonymous
/// unions go in a specific single submodule (`anon_unions`), and we also don't want duplicate
/// definitions of them
#[derive(Debug, Default, Clone)]
pub struct GeneratedQueryTokens {
    /// Toplevel declaration tokens
    pub toplevel: TokenStream,
    /// Anonymous unions and tokens for the `anon_union` submodule
    pub anon_unions: AnonUnions,
}

impl GeneratedQueryTokens {
    /// Empty instance
    pub fn new() -> Self {
        Self::default()
    }

    /// Append the tokens to the given module
    pub fn append_tokens(&mut self, tokens: TokenStream) {
        self.toplevel.extend(tokens);
    }

    /// Append the other's tokens into this
    pub fn append(&mut self, other: GeneratedQueryTokens) {
        self.toplevel.extend(other.toplevel);
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
        pretty_print(&self.collapse(&super_nodes()))
    }
}

impl Extend<GeneratedQueryTokens> for GeneratedQueryTokens {
    fn extend<T: IntoIterator<Item = GeneratedQueryTokens>>(&mut self, iter: T) {
        for x in iter {
            self.append(x)
        }
    }
}

impl FromIterator<GeneratedQueryTokens> for GeneratedQueryTokens {
    fn from_iter<T: IntoIterator<Item = GeneratedQueryTokens>>(iter: T) -> Self {
        let mut this = Self::new();
        Extend::<GeneratedQueryTokens>::extend(&mut this, iter);
        this
    }
}
