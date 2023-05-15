use std::borrow::Cow;
use std::fmt::{Display, Formatter};
use std::iter::{empty, zip};
use std::ops::Index;
use logos::Logos;

/// Parsed tree-sitter query = sequence of s-expressions
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SExpSeq<'a>(Vec<SExp<'a>>);

/// Tree-sitter query s-expression
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SExp<'a> {
    Atom { span: Span, atom: Atom<'a> },
    Group { span: Span, group_type: GroupType, items: SExpSeq<'a> }
}

/// S-expression "parenthesis type"
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GroupType {
    Paren,
    Bracket
}

/// S-expression which is not a group
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Atom<'a> {
    /// `_`
    Wildcard,
    /// `.`
    Anchor,
    /// `foo_bar:`
    Field { name: &'a str },
    /// `foo_bar`
    Ident { name: &'a str },
    /// `"foo bar"`
    String { content: Cow<'a, str> },
    /// `!foo_bar`
    Negation { name: &'a str },
    /// `@foo_bar`
    Capture { name: &'a str },
    /// `#foo_bar`
    Predicate { name: &'a str },
}

/// Tree-sitter query stream parser
struct Parser<'a> {
    lexer: Lexer<'a>,
}

/// Tree-sitter query lexer
type Lexer<'a> = logos::Lexer<'a, Token<'a>>;

/// Tree-sitter query lex token
#[derive(Logos)]
#[logos(skip r"[ \t\n\f]+")]
#[logos(skip r";[^\n]*")]
enum Token<'a> {
    // Group types
    #[token("(")]
    LParen,
    #[token(")")]
    RParen,
    #[token("[")]
    LBracket,
    #[token("]")]
    RBracket,
    // Atom types (can't be merged with [Atom] because logos is limited)
    #[token("_")]
    Wildcard,
    #[token(".")]
    Anchor,
    #[regex(r#"[a-zA-Z_][a-zA-Z0-9_\-+\.!?@#$%^&*|'/<>]*:"#, lex_snoc)]
    Field(&'a str),
    #[regex(r#"[a-zA-Z_][a-zA-Z0-9_\-+\.!?@#$%^&*|'/<>]*"#, Lexer::slice)]
    Ident(&'a str),
    #[regex(r#""([^"\\]|\\.)*""#, unquote_simple)]
    String(Cow<'a, str>),
    #[regex(r#"![a-zA-Z_][a-zA-Z0-9_\-+\.!?@#$%^&*|'/<>]*"#, lex_tail)]
    Negation(&'a str),
    #[regex(r#"@[a-zA-Z0-9_\-+\.!?@#$%^&*|'/<>]*"#, lex_tail)]
    Capture(&'a str),
    /// `#foo_bar`
    #[regex(r#"#[a-zA-Z0-9_\-+\.!?@#$%^&*|'/<>]*"#, lex_tail)]
    Predicate(&'a str),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParseError {
    Eof { span: Span },
    BadToken { span: Span },
    IllegalGroupClose { span: Span, group_type: GroupType },
    UnclosedGroup { span: Span, group_type: GroupType }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Span {
    /// Byte offset immediately before first char
    start: usize,
    /// Byte offset immediately after last char
    end: usize
}

impl<'a> SExpSeq<'a> {
    pub fn captured_patterns(&self, name: &'a str) -> impl Iterator<Item=SExp<'a>> + '_ {
        self.toplevel_captured_patterns(name).chain(
            self.iter().flat_map(|sexp| match sexp {
                SExp::Atom { .. } => Box::new(empty()) as Box<dyn Iterator<Item=SExp<'a>> + '_>,
                SExp::Group { items, .. } => Box::new(items.captured_patterns(name)) as Box<dyn Iterator<Item=SExp<'a>> + '_>
            })
        )
    }

    fn toplevel_captured_patterns(&self, name: &'a str) -> impl Iterator<Item=SExp<'a>> + '_ {
        zip(self, self.iter().skip(1))
            .filter(|(captured, capture)| {
                !matches!(captured, SExp::Atom { atom: Atom::Predicate { .. }, .. }) &&
                    capture.is_capture(name)
            })
            .map(|(pattern, _)| pattern.clone())
    }
}

impl<'a> TryFrom<&'a str> for SExpSeq<'a> {
    type Error = ParseError;

    fn try_from(source: &'a str) -> Result<Self, Self::Error> {
        Parser::new(source).collect()
    }
}

impl<'a> Display for SExpSeq<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut sexps = self.iter();
        if let Some(sexp) = sexps.next() {
            write!(f, "{}", sexp)?;
        }
        for sexp in sexps {
            write!(f, " {}", sexp)?;
        }
        Ok(())
    }
}

impl<'a> SExp<'a> {
    pub fn is_capture(&self, name: &'a str) -> bool {
        match self {
            Self::Atom { atom: Atom::Capture { name: atom_name }, .. } => name == *atom_name,
            _ => false
        }
    }

    pub fn is_predicate(&self) -> bool {
        match self {
            Self::Atom { atom: Atom::Predicate { .. }, .. } => true,
            _ => false
        }
    }

    pub fn is_predicate_group(&self) -> bool {
        match self {
            Self::Atom { .. } => false,
            Self::Group { items, .. } => !items.is_empty() && items[0].is_predicate()
        }
    }

    pub fn span(&self) -> Span {
        match self {
            Self::Atom { span, .. } => *span,
            Self::Group { span, .. } => *span
        }
    }
}

impl<'a> Display for SExp<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Atom { atom, .. } => write!(f, "{}", atom),
            Self::Group { items, group_type, .. } => {
                write!(f, "{}{}{}", group_type.start_char(), items, group_type.end_char())
            }
        }
    }
}

impl GroupType {
    pub fn start_char(&self) -> char {
        match self {
            Self::Paren => '(',
            Self::Bracket => '['
        }
    }

    pub fn end_char(&self) -> char {
        match self {
            Self::Paren => ')',
            Self::Bracket => ']'
        }
    }
}

impl Display for GroupType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Paren => write!(f, "paren"),
            Self::Bracket => write!(f, "bracket")
        }
    }
}

impl<'a> TryFrom<Token<'a>> for Atom<'a> {
    type Error = Token<'a>;

    fn try_from(value: Token<'a>) -> Result<Self, Self::Error> {
        match value {
            Token::Wildcard => Ok(Self::Wildcard),
            Token::Anchor => Ok(Self::Anchor),
            Token::Field(name) => Ok(Self::Field { name }),
            Token::Ident(name) => Ok(Self::Ident { name }),
            Token::String(content) => Ok(Self::String { content }),
            Token::Negation(name) => Ok(Self::Negation { name }),
            Token::Capture(name) => Ok(Self::Capture { name }),
            Token::Predicate(name) => Ok(Self::Predicate { name }),
            token => Err(token)
        }
    }
}

impl<'a> Display for Atom<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Atom::Wildcard => write!(f, "_"),
            Atom::Anchor => write!(f, "^"),
            Atom::Field { name } => write!(f, "{}:", name),
            Atom::Ident { name } => write!(f, "{}", name),
            Atom::String { content } => write!(f, "{:?}", content),
            Atom::Negation { name } => write!(f, "!{}", name),
            Atom::Capture { name } => write!(f, "@{}", name),
            Atom::Predicate { name } => write!(f, "#{}", name)
        }
    }
}

impl<'a> Parser<'a> {
    pub fn new(source: &'a str) -> Self {
        Self { lexer: Lexer::new(source) }
    }

    fn parse_next(&mut self) -> Result<SExp<'a>, ParseError> {
        let next = self.lexer.next();
        let span = Span::of(&self.lexer);
        match next {
            None => Err(ParseError::Eof { span }),
            Some(Err(())) => Err(ParseError::BadToken { span }),
            Some(Ok(token)) => match Atom::try_from(token) {
                Ok(atom) => Ok(SExp::Atom { span, atom }),
                Err(token) => match token {
                    Token::LParen => self.finish_parsing_group(GroupType::Paren),
                    Token::LBracket => self.finish_parsing_group(GroupType::Bracket),
                    Token::RParen => Err(ParseError::IllegalGroupClose { span, group_type: GroupType::Paren }),
                    Token::RBracket => Err(ParseError::IllegalGroupClose { span, group_type: GroupType::Bracket }),
                    _ => unreachable!("should've been handled by atom or group")
                }
            }
        }
    }

    fn finish_parsing_group(&mut self, group_type: GroupType) -> Result<SExp<'a>, ParseError> {
        let span_start = Span::of(&self.lexer).start;
        let mut items = SExpSeq::new();
        loop {
            match self.parse_next() {
                Ok(item) => items.push(item),
                Err(ParseError::IllegalGroupClose { span, group_type: close_type }) if group_type == close_type => {
                    let span_end = span.end;
                    let span = Span { start: span_start, end: span_end };
                    break Ok(SExp::Group { span, group_type, items })
                },
                Err(ParseError::Eof { span }) => {
                    let span_end = span.end;
                    let span = Span { start: span_start, end: span_end };
                    break Err(ParseError::UnclosedGroup { span, group_type })
                }
                Err(err) => return Err(err)
            }
        }
    }
}

impl<'a> Iterator for Parser<'a> {
    type Item = Result<SExp<'a>, ParseError>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.parse_next() {
            Err(ParseError::Eof { .. }) => None,
            next => Some(next)
        }
    }
}

impl ParseError {
    pub fn span(&self) -> &Span {
        match self {
            Self::Eof { span } => span,
            Self::BadToken { span } => span,
            Self::IllegalGroupClose { span, .. } => span,
            Self::UnclosedGroup { span, .. } => span
        }
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Eof { span } => write!(f, "unexpected end of input at {}", span),
            Self::BadToken { span } => write!(f, "bad token at {}", span),
            Self::IllegalGroupClose { span, group_type } => write!(f, "illegal group close at {} (expected {})", span, group_type),
            Self::UnclosedGroup { span, group_type } => write!(f, "unclosed group at {} (expected {})", span, group_type)
        }
    }
}

impl Span {
    fn of(lexer: &Lexer<'_>) -> Self {
        Self::from(lexer.span())
    }

    pub fn range(&self) -> std::ops::Range<usize> {
        self.start..self.end
    }
}

impl From<logos::Span> for Span {
    fn from(value: logos::Span) -> Self {
        Span { start: value.start, end: value.end }
    }
}

impl Index<Span> for str {
    type Output = str;

    fn index(&self, index: Span) -> &Self::Output {
        &self[index.range()]
    }
}

impl Display for Span {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}..{}", self.start, self.end)
    }
}

fn lex_snoc<'a>(lex: &mut Lexer<'a>) -> &'a str {
    &lex.slice()[..lex.slice().len() - 1]
}

fn lex_tail<'a>(lex: &mut Lexer<'a>) -> &'a str {
    &lex.slice()[1..]
}

/// Unquotes a string where only single-character escape codes are allowed
fn unquote_simple<'a>(lex: &mut Lexer<'a>) -> Cow<'a, str> {
    let slice = &lex.slice()[1..lex.slice().len() - 1];
    if slice.contains("\\") {
        Cow::Owned(slice
            .replace("\\\"", "\"")
            .replace("\\\\", "\\")
            .replace("\\n", "\n")
            .replace("\\r", "\r")
            .replace("\\t", "\t")
            .replace("\\0", "\0")
            .replace("\\'", "'"))

    } else {
        Cow::Borrowed(slice)
    }
}

// region SExpSeq Vec delegate
impl<'a> SExpSeq<'a> {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn push(&mut self, item: SExp<'a>) {
        self.0.push(item);
    }

    pub fn iter(&self) -> impl Iterator<Item=&SExp<'a>> {
        self.0.iter()
    }

    pub fn get(&self, index: usize) -> Option<&SExp<'a>> {
        self.0.get(index)
    }
}

impl<'a> FromIterator<SExp<'a>> for SExpSeq<'a> {
    fn from_iter<T: IntoIterator<Item=SExp<'a>>>(iter: T) -> Self {
        Self(iter.into_iter().collect())
    }
}

impl<'a, 'b> IntoIterator for &'b SExpSeq<'a> {
    type Item = &'b SExp<'a>;
    type IntoIter = std::slice::Iter<'b, SExp<'a>>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl<'a> Index<usize> for SExpSeq<'a> {
    type Output = SExp<'a>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}
// endregion