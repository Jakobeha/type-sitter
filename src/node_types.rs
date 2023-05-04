use std::collections::HashMap;
use serde::Deserialize;
use convert_case::{Casing, Case};

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
    pub rust_method_name: String,
    pub is_implicit: bool
}

impl From<String> for NodeName {
    fn from(sexp_name: String) -> Self {
        let (is_implicit, snake_name) = match value.starts_with("_") {
            false => (false, &value[..]),
            true => (true, &value[1..])
        };
        let rust_method_name = replace_punctuation(snake_name);
        let rust_type_name = rust_method_name.to_case(Case::UpperCamel);
        Self { sexp_name, rust_type_name, rust_method_name, is_implicit }
    }
}

/// Replace punctuation with names (e.g. '&' -> 'And')
pub fn replace_punctuation(str: &str) -> String {

}