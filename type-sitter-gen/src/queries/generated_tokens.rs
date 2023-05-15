use proc_macro2::TokenStream;
use std::fmt::{Display, Formatter};
use crate::anon_unions::AnonUnions;
use crate::super_nodes;

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

impl Display for GeneratedQueryTokens {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.clone().collapse(&super_nodes()))
    }
}