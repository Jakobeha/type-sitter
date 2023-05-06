use std::collections::HashMap;
use serde::Deserialize;
use convert_case::{Casing, Case};
use crate::make_valid::make_valid;

#[derive(Deserialize)]
pub struct NodeType {
    #[serde(rename = "type")]
    pub name: NodeName,
    pub named: bool,
    #[serde(flatten)]
    pub kind: NodeTypeKind
}

#[derive(Deserialize)]
#[serde(untagged)]
pub enum NodeTypeKind {
    Supertype { subtypes: Vec<Type> },
    Regular {
        #[serde(default)]
        fields: HashMap<String, Children>,
        #[serde(default)]
        children: Option<Children>,
    }
}

#[derive(Deserialize)]
pub struct Type {
    #[serde(rename = "type")]
    pub name: NodeName,
    pub named: bool,
}

#[derive(Deserialize)]
pub struct Children {
    pub multiple: bool,
    pub required: bool,
    pub types: Vec<Type>,
}

#[derive(Deserialize)]
#[serde(from = "String")]
pub struct NodeName {
    pub sexp_name: String,
    pub rust_type_name: String,
    pub is_implicit: bool
}

impl From<String> for NodeName {
    fn from(sexp_name: String) -> Self {
        let (is_implicit, snake_case) = match sexp_name.starts_with("_") && sexp_name != "_" {
            false => (false, &sexp_name[..]),
            true => (true, &sexp_name[1..])
        };
        let rust_type_name = make_valid(&snake_case.to_case(Case::UpperCamel));
        Self { sexp_name, rust_type_name, is_implicit }
    }
}