use std::collections::BTreeMap;
use crate::NodeType;
use proc_macro2::TokenStream;
use std::fmt::{Display, Formatter};

/// Holds the tokens to define anonymous unions, so that there aren't duplicates
pub type AnonUnions = BTreeMap<AnonUnionId, TokenStream>;

/// Id for an anonymous union to reference it and prevent duplicates
#[derive(Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct AnonUnionId {
    pub(super) name: String
}

impl AnonUnionId {
    /// Create an id for the anonymous union of `names`
    pub(crate) fn new(types: &[&NodeType]) -> Self {
        Self { name: NodeType::anon_union_type_name(types.iter().copied()).to_string() }
    }

    /// Create an id for the anonymous union for a query capture variant named `capture_variant_name`
    pub(crate) fn query_capture(capture_variant_name: &str) -> Self {
        Self { name: capture_variant_name.to_string() }
    }
}

impl Display for AnonUnionId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}
