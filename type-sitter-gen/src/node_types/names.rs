use std::borrow::Cow;
use std::iter::once;
use join_lazy_fmt::Join;
use serde::Deserialize;

/// Uniquely identifies a node within a grammar.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct NodeName {
    #[serde(rename = "type")]
    pub sexp_name: String,
    #[serde(rename = "named")]
    pub is_named: bool,
}

impl NodeName {
    pub fn is_implicit(&self) -> bool {
        self.sexp_name != "_" && self.sexp_name.starts_with('_')
    }

    pub(super) fn kind<'a>(names: impl IntoIterator<Item=&'a NodeName>) -> Cow<'a, str> {
        let mut names = names.into_iter();

        let Some(first) = names.next() else {
            return Cow::Borrowed("{}");
        };

        let Some(second) = names.next() else {
            return Cow::Borrowed(&first.sexp_name);
        };

        // Effectively add the first and second name back to the iterator.
        let names = once(first).chain(once(second)).chain(names);

        Cow::Owned(format!("{{{}}}", " | ".join(names.map(|n| &n.sexp_name))))
    }
}