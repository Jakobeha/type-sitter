use std::iter::zip;
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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Atom<'a> {
    /// `_`
    Wildcard,
    /// `.`
    Anchor,
    /// `foo_bar`
    Ident { name: &'a str },
    /// `"foo bar"`
    String { content: &'a str },
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
    #[regex(r#"[a-zA-Z_][a-zA-Z0-9_\.]*"#, Lexer::slice)]
    Ident(&'a str),
    #[regex(r#""([^"\\]|\.)*""#, Lexer::slice)]
    String(&'a str),
    #[regex(r#"![a-zA-Z_][a-zA-Z0-9_\.]*"#, lex_tail)]
    Negation(&'a str),
    #[regex(r#"@[a-zA-Z0-9_\-+\.]*"#, lex_tail)]
    Capture(&'a str),
    /// `#foo_bar`
    #[regex(r#"#[a-zA-Z0-9_\-+\.]*"#, lex_tail)]
    Predicate(&'a str),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParseError {
    Eof { span: Span },
    BadCharacter { span: Span },
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
    pub fn captured_pattern(&self, name: &'a str) -> Option<SExp<'a>> {
        zip(self, self.iter().skip(1))
            .find(|(_, capture)| capture.is_capture(name))
            .map(|(pattern, _)| pattern.clone())
    }
}

impl<'a> TryFrom<&'a str> for SExpSeq<'a> {
    type Error = ParseError;

    fn try_from(source: &'a str) -> Result<Self, Self::Error> {
        Parser::new(source).collect()
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

impl<'a> TryFrom<Token<'a>> for Atom<'a> {
    type Error = Token<'a>;

    fn try_from(value: Token<'a>) -> Result<Self, Self::Error> {
        match value {
            Token::Wildcard => Ok(Self::Wildcard),
            Token::Anchor => Ok(Self::Anchor),
            Token::Ident(name) => Ok(Self::Ident { name }),
            Token::String(content) => Ok(Self::String { content }),
            Token::Negation(name) => Ok(Self::Negation { name }),
            Token::Capture(name) => Ok(Self::Capture { name }),
            Token::Predicate(name) => Ok(Self::Predicate { name }),
            token => Err(token)
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
            Some(Err(())) => Err(ParseError::BadCharacter { span }),
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

fn lex_tail<'a>(lex: &mut Lexer<'a>) -> &'a str {
    &lex.slice()[1..]
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

impl<'a, 'b> Iterator for &'b SExpSeq<'a> {
    type Item = &'b SExp<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.iter().next()
    }
}

impl<'a> Index<usize> for SExpSeq<'a> {
    type Output = SExp<'a>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}
// endregion