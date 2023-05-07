use serde::Deserialize;
use indexmap::IndexMap;

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
    pub is_implicit: bool,
    pub is_named: bool
}

#[derive(Deserialize)]
pub struct _NodeName {
    #[serde(rename = "type")]
    pub sexp_name: String,
    pub named: bool,
}

#[derive(PartialEq, Eq, Hash)]
pub struct AnonUnionId {
    pub name: String
}

impl From<_NodeName> for NodeName {
    fn from(_NodeName { sexp_name, named }: _NodeName) -> Self {
        NodeName::new(sexp_name, named)
    }
}