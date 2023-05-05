use std::fmt::Write;

const PUNCTUATION_TABLE: [(char, &'static str); 35] = [
    ('&', "And"), ('|', "Or"), ('!', "Not"), ('=', "Eq"), ('<', "Lt"),
    ('>', "Gt"), ('+', "Add"), ('-', "_"), ('*', "Mul"), ('/', "Div"),
    ('~', "BitNot"), ('%', "Mod"), ('^', "BitXor"), ('?', "Question"), (':', "Colon"),
    ('.', "Dot"), (',', "Comma"), (';', "Semicolon"), ('(', "LParen"), (')', "RParen"),
    ('[', "LBracket"), (']', "RBracket"), ('{', "LBrace"), ('}', "RBrace"), ('\\', "Backslash"),
    ('\'', "Quote"), ('"', "DoubleQuote"), ('#', "Hash"), ('@', "At"), ('$', "Dollar"),
    ('`', "Backtick"), (' ', "Space"), ('\t', "Tab"), ('\n', "Newline"), ('\r', "CarriageReturn")
];

/// Replace punctuation with names (e.g. '&' -> 'And'), and prepend '_' if necessary, to make this
/// a valid identifier.
pub fn make_valid(str: &str) -> String {
    let mut result = String::with_capacity(str.len());
    if str.is_empty() || str.starts_with(|c: char| c.is_numeric()) {
        result.push('_');
    }
    for char in str.chars() {
        match PUNCTUATION_TABLE.iter().find(|(c, _)| *c == char).map(|(_, s)| *s) {
            Some(name) => result.push_str(name),
            None if !char.is_ascii_alphanumeric() => write!(result, "U{:X}", char as u32).unwrap(),
            None => result.push(char)
        }
    }
    result
}