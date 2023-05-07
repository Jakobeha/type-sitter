use serde::Deserialize;
use convert_case::{Casing, Case};
use indexmap::IndexMap;
use crate::make_valid::make_valid;

#[derive(Deserialize)]
pub struct NodeType {
    #[serde(flatten)]
    pub name: NodeName,
    #[serde(flatten)]
    pub kind: NodeTypeKind
}

#[derive(Deserialize)]
#[serde(untagged)]
pub enum NodeTypeKind {
    Supertype { subtypes: Vec<NodeName> },
    Regular {
        #[serde(default)]
        fields: IndexMap<String, Children>,
        #[serde(default)]
        children: Option<Children>,
    }
}

#[derive(Deserialize)]
pub struct Children {
    pub multiple: bool,
    pub required: bool,
    pub types: Vec<NodeName>,
}

#[derive(Deserialize)]
#[serde(from = "_NodeName")]
pub struct NodeName {
    pub sexp_name: String,
    pub rust_type_name: String,
    pub rust_method_name: String,
    pub is_implicit: bool
}

#[derive(Deserialize)]
struct _NodeName {
    #[serde(rename = "type")]
    pub sexp_name: String,
    pub named: bool,
}

impl From<_NodeName> for NodeName {
    fn from(_NodeName { sexp_name, named }: _NodeName) -> Self {
        let (is_implicit, snake_case) = match sexp_name.starts_with("_") && sexp_name != "_" {
            false => (false, &sexp_name[..]),
            true => (true, &sexp_name[1..])
        };
        let case = match named {
            false => Case::UpperSnake,
            true => Case::Pascal,
        };
        let rust_type_name = make_valid(&snake_case.from_case(Case::Snake).to_case(case));
        let mut rust_method_name = rust_type_name.from_case(case).to_case(Case::Snake);
        // Hacky workaround because cases are hard. TODO: Fix names in general
        if rust_method_name.is_empty() {
            rust_method_name.push_str("__");
        }
        Self { sexp_name, rust_type_name, rust_method_name, is_implicit }
    }
}