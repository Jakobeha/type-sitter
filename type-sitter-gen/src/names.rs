use std::borrow::Cow;
use std::collections::HashSet;
use convert_case::{Case, Casing};
use std::fmt::{Display, Write};
use check_keyword::CheckKeyword;
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

pub struct PrevNodeRustNames {
    types: HashSet<String>,
    methods: HashSet<String>,
}

#[derive(Deserialize)]
pub struct _NodeName {
    #[serde(rename = "type")]
    pub sexp_name: String,
    pub named: bool,
}

const PUNCTUATION_TABLE: [(char, &'static str); 35] = [
    ('&', "And"), ('|', "Or"), ('!', "Not"), ('=', "Eq"), ('<', "Lt"),
    ('>', "Gt"), ('+', "Add"), ('-', "Sub"), ('*', "Mul"), ('/', "Div"),
    ('~', "BitNot"), ('%', "Mod"), ('^', "BitXor"), ('?', "Question"), (':', "Colon"),
    ('.', "Dot"), (',', "Comma"), (';', "Semicolon"), ('(', "LParen"), (')', "RParen"),
    ('[', "LBracket"), (']', "RBracket"), ('{', "LBrace"), ('}', "RBrace"), ('\\', "Backslash"),
    ('\'', "Quote"), ('"', "DoubleQuote"), ('#', "Hash"), ('@', "At"), ('$', "Dollar"),
    ('`', "Backtick"), (' ', "Space"), ('\t', "Tab"), ('\n', "Newline"), ('\r', "CarriageReturn")
];

impl NodeName {
    /// Create a node name from the sexp name.
    fn new(sexp_name: String, is_named: bool) -> Self {
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

    /// If this has the same `rust_type_name` as another [`NodeName`] as indicated by the set,
    /// appends underscores until it's unique. Does the same to `rust_method_name`. Then adds both
    /// to the sets to ensure the next names are also unique.
    pub(crate) fn disambiguate_rust_names(&mut self, prev: &mut PrevNodeRustNames) {
        while prev.types.contains(&self.rust_type_name) {
            self.rust_type_name.push('_');
        }
        while prev.methods.contains(&self.rust_method_name) {
            self.rust_method_name.push('_');
        }

        prev.types.insert(self.rust_type_name.clone());
        prev.methods.insert(self.rust_method_name.clone());
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

    pub(super) fn anon_union_type_name(names: &[NodeName]) -> impl Display + '_ {
        "_".join(names.iter().map(|name| &name.rust_type_name))
    }
}

impl From<_NodeName> for NodeName {
    fn from(_NodeName { sexp_name, named }: _NodeName) -> Self {
        NodeName::new(sexp_name, named)
    }
}

impl PrevNodeRustNames {
    pub fn new() -> Self {
        Self { types: HashSet::new(), methods: HashSet::new() }
    }
}

/// Convert an s-expression to rust type and method identifier names
pub fn sexp_name_to_rust_names(sexp_name: &str) -> (String, String) {
    let rust_type_name = make_valid(&sexp_name.from_case(Case::Snake).to_case(Case::Pascal));

    let mut rust_method_name = rust_type_name.from_case(Case::Pascal).to_case(Case::Snake);
    make_not_reserved(&mut rust_method_name);

    (rust_type_name, rust_method_name)
}

/// Replace punctuation with names (e.g. '&' -> 'And'), prepend `_` if the string starts with a
/// number, and prepend 'r#' or append `_` if the string is a reserved keyword, to make the string a
/// valid identifier.
pub(crate) fn make_valid(str: &str) -> String {
    if str.is_empty() {
        return "__".to_owned();
    } else if str.is_keyword() {
        return str.into_safe();
    }

    let mut result = String::with_capacity(str.len());
    if str.starts_with(|c: char| c.is_numeric()) {
        result.insert(0, '_');
    }
    for char in str.chars() {
        match PUNCTUATION_TABLE.iter().find(|(c, _)| *c == char).map(|(_, s)| *s) {
            Some(name) => result.push_str(name),
            None if !char.is_ascii_alphanumeric() && char != '_' => write!(result, "U{:X}", char as u32).unwrap(),
            None => result.push(char)
        }
    }
    result
}

/// Only prepend `r#` or append `_` if reserved, since we did the rest in [`make_valid`].
fn make_not_reserved(name: &mut String) {
    if name.is_keyword() {
        *name = name.into_safe();
    }
}

/// If the string starts with `r#`, remove it.
pub(crate) fn unmake_reserved_if_raw(name: &mut String) {
    if name.starts_with("r#") {
        name.drain(..2);
    }
}

/// If the string starts with `r#` or is `self_` or `Self_`, remove it.
pub(crate) fn unmake_reserved(name: &mut String) {
    match name.as_str() {
        "crate_" | "self_" | "Self_" | "super_" => {
            name.pop().unwrap();
        },
        _ => unmake_reserved_if_raw(name),
    }

}
