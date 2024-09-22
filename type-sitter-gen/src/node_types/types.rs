use crate::node_types::rust_names::PrevNodeRustNames;
use crate::{NodeName, NodeRustNames};
use indexmap::IndexMap;
use serde::Deserialize;
use std::borrow::Cow;
use std::collections::HashMap;
use std::fmt::Display;
use std::ops::Index;

#[derive(Debug)]
pub(crate) struct NodeTypeMap(HashMap<NodeName, NodeType>);

#[derive(Clone, Debug)]
pub(crate) struct NodeType {
    pub(crate) name: NodeName,
    pub(crate) rust_names: NodeRustNames,
    pub(crate) kind: NodeTypeKind
}

/// Type needs to be finished by disambiguating.
#[derive(Clone, Debug, Deserialize)]
pub(crate) struct ContextFreeNodeType {
    #[serde(flatten)]
    name: NodeName,
    #[serde(flatten)]
    kind: NodeTypeKind
}

#[derive(Clone, Debug, Deserialize)]
#[serde(untagged)]
pub(crate) enum NodeTypeKind {
    Supertype { subtypes: Vec<NodeName> },
    Regular {
        #[serde(default)]
        fields: IndexMap<String, Children>,
        #[serde(default)]
        children: Option<Children>,
    }
}

/// Describes a node's named children: their types and whether there are zero, one, zero to many, or
/// one to many.
#[derive(Clone, Debug, Deserialize)]
pub(crate) struct Children {
    pub(crate) multiple: bool,
    pub(crate) required: bool,
    pub types: Vec<NodeName>,
}

impl NodeTypeMap {
    pub(crate) fn new(mut node_types: Vec<ContextFreeNodeType>) -> Self {
        // Can't do `sort_by_key` because `K` has an existential lifetime, which Rust can't express.
        node_types.sort_by(|lhs, rhs| lhs.name.cmp(&rhs.name));

        let mut prev_rust_names = PrevNodeRustNames::new();
        let map = node_types.into_iter()
            .map(|node_type| (node_type.name.clone(), NodeType::new(node_type, &mut prev_rust_names)))
            .collect();
        Self(map)
    }

    pub fn get(&self, name: &NodeName) -> Option<&NodeType> {
        self.0.get(name)
    }

    pub fn values(&self) -> impl Iterator<Item=&NodeType> {
        self.0.values()
    }
}

impl<'a> Index<&'a NodeName> for NodeTypeMap {
    type Output = NodeType;

    fn index(&self, name: &'a NodeName) -> &Self::Output {
        &self.0[name]
    }
}

impl NodeType {
    fn new(ContextFreeNodeType { name, kind }: ContextFreeNodeType, prev_rust_names: &mut PrevNodeRustNames) -> Self {
        let rust_names = NodeRustNames::new(&name, prev_rust_names);

        Self { name, rust_names, kind }
    }

    pub fn rust_type_path(&self) -> Cow<'_, str> {
        self.rust_names.type_path()
    }

    pub fn anon_union_type_name<'a>(
        types: impl IntoIterator<Item=&'a NodeType, IntoIter: 'a>
    ) -> impl Display + 'a {
        NodeRustNames::anon_union_type_name(types.into_iter().map(|t| &t.rust_names))
    }

    pub fn kind<'a>(
        types: impl IntoIterator<Item=&'a NodeType, IntoIter: 'a>
    ) -> Cow<'a, str> {
        NodeName::kind(types.into_iter().map(|t| &t.name))
    }
}

impl Extend<Children> for Children {
    /// Create a descriptor for a node with the children from this or any of `iter`.
    fn extend<T: IntoIterator<Item=Children>>(&mut self, iter: T) {
        for child in iter {
            // If either of the original Children have at least 1 element, than this Children will.
            self.required |= child.required;

            // Even if both original Children only have up to 1 element, this means this Children
            // will now have up to 2.
            self.multiple = true;

            // Add other child types, but no duplicates
            let len = self.types.len();
            self.types.reserve(child.types.len());
            for child_type in child.types {
                if self.types.iter().take(len).any(|existing_type| *existing_type == child_type) {
                    continue;
                }
                self.types.push(child_type);
            }
        }
    }
}