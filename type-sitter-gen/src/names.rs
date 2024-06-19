use std::borrow::Cow;
use convert_case::{Case, Casing};
use std::fmt::{Display, Write};
use join_lazy_fmt::Join;
use serde::Deserialize;
use crate::node_types::types::NodeModule;

#[derive(Clone, Deserialize)]
#[serde(from = "_NodeName")]
pub struct NodeName {
    pub sexp_name: String,
    pub rust_type_name: String,
    pub rust_method_name: String,
    pub is_implicit: bool,
    pub module: NodeModule,
}

#[derive(Deserialize)]
pub struct _NodeName {
    #[serde(rename = "type")]
    pub sexp_name: String,
    pub named: bool,
}

const PUNCTUATION_TABLE: [(char, &'static str); 35] = [
    ('&', "And_"), ('|', "Or_"), ('!', "Not_"), ('=', "Eq_"), ('<', "Lt_"),
    ('>', "Gt_"), ('+', "Add_"), ('-', "Sub_"), ('*', "Mul_"), ('/', "Div_"),
    ('~', "BitNot_"), ('%', "Mod_"), ('^', "BitXor_"), ('?', "Question_"), (':', "Colon_"),
    ('.', "Dot_"), (',', "Comma_"), (';', "Semicolon_"), ('(', "LParen_"), (')', "RParen_"),
    ('[', "LBracket_"), (']', "RBracket_"), ('{', "LBrace_"), ('}', "RBrace_"), ('\\', "Backslash_"),
    ('\'', "Quote_"), ('"', "DoubleQuote_"), ('#', "Hash_"), ('@', "At_"), ('$', "Dollar_"),
    ('`', "Backtick_"), (' ', "Space_"), ('\t', "Tab_"), ('\n', "Newline_"), ('\r', "CarriageReturn_")
];

const RESERVED_IDENTS: [&'static str; 4] = [
    "Self",
    "self",
    "super",
    "crate"
];

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
        let (rust_type_name, rust_method_name) = sexp_name_to_rust_names(raw_sexp_name);
        let module = match is_named {
            false if sexp_name.contains(|c| PUNCTUATION_TABLE.iter().any(|(c2, _)| c == *c2)) => NodeModule::Symbols,
            false => NodeModule::Unnamed,
            true => NodeModule::Toplevel
        };
        Self { sexp_name, rust_type_name, rust_method_name, is_implicit, module }
    }

    pub fn kind(names: &[NodeName]) -> Cow<'_, str> {
        match names.len() {
            0 => Cow::Borrowed("{}"),
            1 => Cow::Borrowed(&names[0].sexp_name),
            _ => Cow::Owned(format!("{{{}}}", " | ".join(names.iter().map(|n| &n.sexp_name))))
        }
    }

    pub fn rust_type_path(&self) -> Cow<'_, str> {
        match self.module {
            NodeModule::Toplevel => Cow::Borrowed(&self.rust_type_name),
            NodeModule::Unnamed => Cow::Owned(format!("unnamed::{}", self.rust_type_name)),
            NodeModule::Symbols => Cow::Owned(format!("symbols::{}", self.rust_type_name))
        }
    }

    pub fn rust_type_path_of(names: &[NodeName]) -> Cow<'_, str> {
        match names.len() {
            0 => Cow::Borrowed("type_sitter_lib::Never"),
            1 => names[0].rust_type_path(),
            _ => Cow::Owned(format!("anon_unions::{}", Self::anon_union_type_name(names)))
        }
    }

    pub(super) fn anon_union_type_name(names: &[NodeName]) -> impl Display + '_ {
        "_".join(names.iter().map(|name| &name.rust_type_name))
    }
}

impl From<_NodeName> for NodeName {
    fn from(_NodeName { sexp_name, named }: _NodeName) -> Self {
        NodeName::new(sexp_name, named)
    }
}

/// Convert an s-expression to rust type and method identifier names
pub fn sexp_name_to_rust_names(sexp_name: &str) -> (String, String) {
    let rust_type_name = make_valid(&sexp_name.from_case(Case::Snake).to_case(Case::Pascal));
    let rust_method_name = make_valid2(rust_type_name.from_case(Case::Pascal).to_case(Case::Snake));
    (rust_type_name, rust_method_name)
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
