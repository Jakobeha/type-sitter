#[cfg(feature = "yak-sitter")]
use crate::TreeParseError;
use crate::{raw, IncludedRangesError, Language, LanguageError, Node, Range, Tree};
use std::marker::PhantomData;
#[cfg(feature = "yak-sitter")]
use std::path::Path;

/// A stateful object that this is used to produce a tree based on some source code.
#[repr(transparent)]
pub struct Parser<Root: Node<'static>>(pub raw::Parser, PhantomData<Root>);

impl<Root: Node<'static>> Parser<Root> {
    /// Convert a tree-sitter parser into a typed parser by specifying the correct root node type.
    ///
    /// This will succeed even if `Root` is not the language's root node, because
    /// [`Tree::root_node`] checks that the root node is the correct type.
    pub fn wrap(parser: raw::Parser) -> Self {
        Self(parser, PhantomData)
    }

    /// Version of [`wrap`](Self::wrap) for an immutable reference.
    pub fn wrap_ref(parser: &raw::Parser) -> &Self {
        // SAFETY: Same repr
        unsafe { &*(parser as *const raw::Parser as *const Self) }
    }

    /// Version of [`wrap`](Self::wrap) for a mutable reference.
    pub fn wrap_mut(parser: &mut raw::Parser) -> &mut Self {
        // SAFETY: Same repr
        unsafe { &mut *(parser as *mut raw::Parser as *mut Self) }
    }

    /// Create a new parser for the given language. See
    /// [tree-sitter's `Parser::set_language`](tree_sitter::Parser::set_language).
    ///
    /// This will succeed even if `Root` is not the language's root node, because
    /// [`Tree::root_node`] checks that the root node is the correct type.
    ///
    /// If the language changes, the root node should change as well, so `set_language` isn't
    /// exposed by this wrapper. Instead, unwrap the parser, call `set_language`, and re-wrap.
    #[inline]
    #[cfg(feature = "yak-sitter")]
    pub fn new(language: &Language) -> Result<Self, LanguageError> {
        Ok(Self(raw::Parser::new(language)?, PhantomData))
    }

    /// Create a new parser for the given language. See [`tree_sitter::Parser::set_language`]
    #[inline]
    #[cfg(not(feature = "yak-sitter"))]
    pub fn new(language: &Language) -> Result<Self, LanguageError> {
        let mut parser = raw::Parser::new();
        parser.set_language(language)?;
        Ok(Self(parser, PhantomData))
    }

    /// Set the ranges of text the parser should include when parsing. See
    /// [`tree_sitter::Parser::set_included_ranges`]
    #[inline]
    pub fn set_included_ranges(&mut self, ranges: &[Range]) -> Result<(), IncludedRangesError> {
        self.0.set_included_ranges(ranges)
    }

    /// Parse a file. See [`tree_sitter::Parser::parse`].
    ///
    /// Additionally, file must be valid utf-8 or this will return `Err`. The file's path is used
    /// for stable node comparison between trees.
    #[inline]
    #[cfg(feature = "yak-sitter")]
    pub fn parse_file(
        &mut self,
        path: &Path,
        old_tree: Option<&Tree<Root>>,
    ) -> Result<Tree<Root>, TreeParseError> {
        self.0
            .parse_file(path, old_tree.map(|t| &t.0))
            .map(Tree::wrap)
    }

    /// Parse a string. See [`tree_sitter::Parser::parse`].
    ///
    /// The path is stored as metadata that can be accessed from nodes. If you don't need this, pass
    /// `None`.
    #[inline]
    #[cfg(feature = "yak-sitter")]
    pub fn parse_string(
        &mut self,
        text: String,
        path: Option<&Path>,
        old_tree: Option<&Tree<Root>>,
    ) -> Result<Tree<Root>, TreeParseError> {
        self.0
            .parse_string(text, path, old_tree.map(|t| &t.0))
            .map(Tree::wrap)
    }

    /// Parse a byte string. See [`tree_sitter::Parser::parse`].
    ///
    /// Note that the wrappers expect and assume UTF-8, so this will fail if the text is not valid
    /// UTF-8.
    ///
    /// The path is stored as metadata that can be accessed from nodes. If you don't need this, pass
    /// `None`.
    #[inline]
    #[cfg(feature = "yak-sitter")]
    pub fn parse_bytes(
        &mut self,
        byte_text: Vec<u8>,
        path: Option<&Path>,
        old_tree: Option<&Tree<Root>>,
    ) -> Result<Tree<Root>, TreeParseError> {
        self.0
            .parse_bytes(byte_text, path, old_tree.map(|t| &t.0))
            .map(Tree::wrap)
    }

    /// Parse a byte string. See [`tree_sitter::Parser::parse`]
    #[inline]
    #[cfg(not(feature = "yak-sitter"))]
    pub fn parse(
        &mut self,
        text: impl AsRef<[u8]>,
        old_tree: Option<&Tree<Root>>,
    ) -> Result<Tree<Root>, ()> {
        self.0
            .parse(text, old_tree.map(|t| &t.0))
            .ok_or(())
            .map(Tree::wrap)
    }
}
