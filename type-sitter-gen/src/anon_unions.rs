use std::fmt::{Display, Formatter};
use indexmap::IndexMap;
use proc_macro2::TokenStream;
use crate::names::NodeName;

/// Holds the tokens to define anonymous unions, so that there aren't duplicates
pub type AnonUnions = IndexMap<AnonUnionId, TokenStream>;

/// Id for an anonymous union to reference it and prevent duplicates
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AnonUnionId {
    pub name: String
}

impl AnonUnionId {
    /// Create an id for the anonymous union of `names`
    pub fn new(names: &[NodeName]) -> Self {
        Self {
            name: NodeName::anon_union_type_name(names).to_string()
        }
    }

    /// Create an id for the anonymous union for a query capture variant named `capture_variant_name`
    pub fn query_capture(capture_variant_name: &str) -> Self {
        Self {
            name: capture_variant_name.to_string()
        }
    }
}

impl Display for AnonUnionId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}