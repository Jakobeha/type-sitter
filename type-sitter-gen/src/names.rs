use convert_case::{Casing, Case};
use std::fmt::Write;
use join_lazy_fmt::Join;
use crate::node_types::types::{AnonUnionId, NodeModule, NodeName};

const PUNCTUATION_TABLE: [(char, &'static str); 35] = [
    ('&', "And"), ('|', "Or"), ('!', "Not"), ('=', "Eq"), ('<', "Lt"),
    ('>', "Gt"), ('+', "Add"), ('-', "Sub"), ('*', "Mul"), ('/', "Div"),
    ('~', "BitNot"), ('%', "Mod"), ('^', "BitXor"), ('?', "Question"), (':', "Colon"),
    ('.', "Dot"), (',', "Comma"), (';', "Semicolon"), ('(', "LParen"), (')', "RParen"),
    ('[', "LBracket"), (']', "RBracket"), ('{', "LBrace"), ('}', "RBrace"), ('\\', "Backslash"),
    ('\'', "Quote"), ('"', "DoubleQuote"), ('#', "Hash"), ('@', "At"), ('$', "Dollar"),
    ('`', "Backtick"), (' ', "Space"), ('\t', "Tab"), ('\n', "Newline"), ('\r', "CarriageReturn")
];

const RESERVED_IDENTS: [&'static str; 4] = [
    "Self",
    "self",
    "super",
    "crate"
];

impl AnonUnionId {
    pub fn new(names: &[NodeName]) -> Self {
        Self {
            name: "_".join(names.iter().map(|name| name.rust_type_name.clone())).to_string()
        }
    }
}

impl NodeName {
    /// Create a node name from the sexp name.
    pub fn new(sexp_name: String, is_named: bool) -> Self {
        if sexp_name == "_" {
            // Special-cased
            return Self {
                sexp_name,
                rust_type_name: "__".to_owned(),
                rust_method_name: "__".to_owned(),
                is_implicit: false,
                module: match is_named {
                    false => NodeModule::Symbols,
                    true => NodeModule::Toplevel
                },
            };
        }

        let (is_implicit, raw_sexp_name) = match sexp_name.starts_with("_") {
            false => (false, &sexp_name[..]),
            true => (true, &sexp_name[1..])
        };
        let rust_type_name = make_valid(&raw_sexp_name.from_case(Case::Snake).to_case(Case::Pascal));
        let rust_method_name = make_valid2(rust_type_name.from_case(Case::Pascal).to_case(Case::Snake));
        let module = match is_named {
            false if sexp_name.contains(|c| PUNCTUATION_TABLE.iter().any(|(c2, _)| c == *c2)) => NodeModule::Symbols,
            false => NodeModule::Unnamed,
            true => NodeModule::Toplevel
        };
        Self { sexp_name, rust_type_name, rust_method_name, is_implicit, module }
    }
}

/// Replace punctuation with names (e.g. '&' -> 'And'), and prepend '_' if necessary, to make this
/// a valid identifier.
fn make_valid(str: &str) -> String {
    let mut result = String::with_capacity(str.len());
    if str.is_empty() {
        result.push_str("__");
    } else if str.starts_with(|c: char| c.is_numeric()) || RESERVED_IDENTS.contains(&str) {
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

/// Only replace reserved names, since we did the rest in `make_valid`.
fn make_valid2(mut result: String) -> String {
    if RESERVED_IDENTS.contains(&result.as_str()) {
        result.push('_');
    }
    result
}