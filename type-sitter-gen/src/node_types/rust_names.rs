use crate::NodeName;
use check_keyword::CheckKeyword;
use convert_case::{Case, Casing};
use enum_map::{Enum, EnumMap};
use join_lazy_fmt::Join;
use std::borrow::Cow;
use std::collections::HashSet;
use std::fmt::{Display, Write};

/// Context to disambiguate a node's type and method accessor.
///
/// tree-sitter is way more permissive with it's node names than Rust. So we have a "pick two of
/// three" scenario:
/// 1. Good Rust name (i.e. no extra characters that only make sense if there are other nodes
///    that said characters would disambiguate).
/// 2. Name follows Rust conventions.
/// 3. Name is context-free.
///
/// We pick 1) and 2). We generate good names like `string` -> `String` and `+` -> `Add`, but that
/// means we must disambiguate `+` and `add`, `string` and `String`, etc. by appending an underscore
/// to the latter.
#[derive(Debug)]
pub(super) struct PrevNodeRustNames {
    types: EnumMap<NodeModule, HashSet<String>>,
    methods: HashSet<String>,
}

/// Describes a node's rust type, method accessor name, and the module the type is located in.
#[derive(Clone, Debug)]
pub(crate) struct NodeRustNames {
    pub(crate) type_name: String,
    pub(crate) method_name: String,
    pub(crate) module: NodeModule,
}

/// Where a node type is located.
///
/// We have three modules to organize things and remove *some* name conflicts. Specifically, we have
/// `unnamed` and `symbols` are separate from the top-level so node named-ness is apparent and we
/// may have nodes like `_if` (becomes `If`) and `"if"`, and `unnamed` is separate from `symbols` so
/// unnamed nodes derived from symbols are apparent *and* because we may have nodes like `"+"` and
/// `"add"`.
#[derive(Clone, Copy, Debug, Enum, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum NodeModule {
    Toplevel,
    Unnamed,
    Symbols
}

/// Common punctuation, which we replace with something descriptive instead of unicode.
const PUNCTUATION_TABLE: [(char, &'static str); 35] = [
    ('&', "And"), ('|', "Or"), ('!', "Not"), ('=', "Eq"), ('<', "Lt"),
    ('>', "Gt"), ('+', "Add"), ('-', "Sub"), ('*', "Mul"), ('/', "Div"),
    ('~', "BitNot"), ('%', "Mod"), ('^', "BitXor"), ('?', "Question"), (':', "Colon"),
    ('.', "Dot"), (',', "Comma"), (';', "Semicolon"), ('(', "LParen"), (')', "RParen"),
    ('[', "LBracket"), (']', "RBracket"), ('{', "LBrace"), ('}', "RBrace"), ('\\', "Backslash"),
    ('\'', "Quote"), ('"', "DoubleQuote"), ('#', "Hash"), ('@', "At"), ('$', "Dollar"),
    ('`', "Backtick"), (' ', "Space"), ('\t', "Tab"), ('\n', "Newline"), ('\r', "CarriageReturn")
];

impl NodeRustNames {
    pub(super) fn new(node_name: &NodeName, prev: &mut PrevNodeRustNames) -> Self {
        let mut this = Self::context_free(node_name);
        this.disambiguate_rust_names(prev);
        this
    }

    fn context_free(NodeName { sexp_name, is_named }: &NodeName) -> Self {
        if sexp_name == "_" {
            // Special-cased
            return Self {
                type_name: "__".to_owned(),
                method_name: "__".to_owned(),
                module: match *is_named {
                    false => NodeModule::Symbols,
                    true => NodeModule::Toplevel
                },
            };
        }

        let raw_sexp_name = if sexp_name.starts_with('_') {
            &sexp_name[1..]
        } else {
            sexp_name
        };
        let (rust_type_name, rust_method_name) = sexp_name_to_rust_names(raw_sexp_name);

        let module = match *is_named {
            false if sexp_name.contains(|c| PUNCTUATION_TABLE.iter().any(|(c2, _)| c == *c2)) => NodeModule::Symbols,
            false => NodeModule::Unnamed,
            true => NodeModule::Toplevel
        };

        Self { type_name: rust_type_name, method_name: rust_method_name, module }

    }

    /// If this has the same `rust_type_name` as another [`NodeName`] as indicated by the set,
    /// appends underscores until it's unique. Does the same to `rust_method_name`. Then adds both
    /// to the sets to ensure the next names are also unique.
    fn disambiguate_rust_names(&mut self, prev: &mut PrevNodeRustNames) {
        while prev.types[self.module].contains(&self.type_name) {
            self.type_name.push('_');
        }
        while prev.methods.contains(&self.method_name) {
            self.method_name.push('_');
        }

        prev.types[self.module].insert(self.type_name.clone());
        prev.methods.insert(self.method_name.clone());
    }

    pub(super) fn type_path(&self) -> Cow<'_, str> {
        match self.module {
            NodeModule::Toplevel => Cow::Borrowed(&self.type_name),
            NodeModule::Unnamed => Cow::Owned(format!("unnamed::{}", self.type_name)),
            NodeModule::Symbols => Cow::Owned(format!("symbols::{}", self.type_name))
        }
    }

    pub(super) fn anon_union_type_name<'a>(
        names: impl IntoIterator<Item=&'a NodeRustNames, IntoIter: 'a>
    ) -> impl Display + 'a {
        "_".join(names.into_iter().map(|name| &name.type_name))
    }
}

impl PrevNodeRustNames {
    pub(super) fn new() -> Self {
        Self { types: EnumMap::default(), methods: HashSet::new() }
    }
}

/// Convert an s-expression to rust type and method identifier names
pub(crate) fn sexp_name_to_rust_names(sexp_name: &str) -> (String, String) {
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