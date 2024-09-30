use crate::node_types::rust_names::PrevNodeRustNames;
use crate::{Error, NodeName, NodeRustNames};
use indexmap::IndexMap;
use serde::Deserialize;
use std::borrow::Cow;
use std::collections::HashMap;
use std::fmt::Display;
use std::ops::{BitOrAssign, Index};
use std::path::Path;

use super::deserialize_json_array_as_stream::iter_json_array;

#[derive(Debug)]
pub struct NodeTypeMap { nodes: HashMap<NodeName, NodeType>, prev_rust_names: PrevNodeRustNames }

#[derive(Clone, Debug)]
pub struct NodeType {
    pub name: NodeName,
    pub(crate) rust_names: NodeRustNames,
    pub kind: NodeTypeKind,
}

/// Type needs to be finished by disambiguating.
#[derive(Clone, Debug, Deserialize)]
pub(crate) struct ContextFreeNodeType {
    #[serde(flatten)]
    name: NodeName,
    #[serde(flatten)]
    kind: NodeTypeKind,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(untagged)]
pub enum NodeTypeKind {
    Supertype {
        subtypes: Vec<NodeName>,
    },
    Regular {
        #[serde(default)]
        fields: IndexMap<String, Children>,
        #[serde(default)]
        children: Children,
    },
}

/// Describes a node's named children: their types and quantity.
///
/// "Quantity" here means:
/// - Zero
/// - Zero to one
/// - Zero to many
/// - Exactly one
/// - One to many
///
/// **Unchecked invariant:** if `types` is empty, `multiple` and `required` must be `false`.
#[derive(Clone, Debug, Deserialize)]
pub struct Children {
    /// If `true`, there can be more than one child.
    pub multiple: bool,
    /// If `false`, there can be zero children.
    pub required: bool,
    /// Possible types of children.
    ///
    /// Additionally, if this is empty, that means there are no children.
    pub types: Vec<NodeName>,
}

impl NodeTypeMap {
    pub(crate) fn new(mut node_types: Vec<ContextFreeNodeType>) -> Self {
        // Can't do `sort_by_key` because `K` has an existential lifetime, which Rust can't express.
        node_types.sort_by(|lhs, rhs| lhs.name.cmp(&rhs.name));

        let mut prev_rust_names = PrevNodeRustNames::new();
        let nodes = node_types.into_iter()
            .map(|node_type| (node_type.name.clone(), NodeType::new(node_type, &mut prev_rust_names)))
            .collect();

        Self { nodes, prev_rust_names }
    }

    pub fn get(&self, name: &NodeName) -> Option<&NodeType> {
        self.nodes.get(name)
    }

    pub fn values(&self) -> impl Iterator<Item = &NodeType> {
        self.nodes.values()
    }

    pub fn add_custom_supertype(&mut self, name: &str, subtypes: impl Into<Vec<NodeName>>)
        -> Result<NodeName, NodeName>
    {
        // Supertypes should be hidden nodes, so ensure the leading underscore.
        let name = if !name.starts_with("_") {
            eprintln!("Warning: Custom supertype '{name}' without leading underscore found! Converting name to '_{name}'.");
            format!("_{name}")
        } else {
            name.to_owned()
        };

        let name = NodeName {
            sexp_name: name,
            is_named: true,
        };
        let new_node = ContextFreeNodeType {
            name: name.clone(),
            kind: NodeTypeKind::Supertype { subtypes: subtypes.into() },
        };
        let new_node = NodeType::new(new_node, &mut self.prev_rust_names);

        if !self.nodes.contains_key(&name) {
            self.nodes.insert(name.clone(), new_node);
            Ok(name)
        } else {
            Err(name)
        }
    }
}

impl TryFrom<&Path> for NodeTypeMap {
    type Error = crate::Error;

    fn try_from(path: &Path) -> Result<Self, Self::Error> {
        let text = std::fs::read_to_string(path)?;
        NodeTypeMap::try_from(text.as_str())
    }
}

impl TryFrom<&str> for NodeTypeMap {
    type Error = crate::Error;

    fn try_from(text: &str) -> Result<Self, Self::Error> {
        let elems = iter_json_array::<ContextFreeNodeType, _>(text.as_bytes())
            .collect::<Result<Vec<_>, _>>()?;
        Ok(NodeTypeMap::new(elems))
    }
}

impl ToString for NodeTypeMap {
    fn to_string(&self) -> String {
        todo!()
    }
}

impl<'a> Index<&'a NodeName> for NodeTypeMap {
    type Output = NodeType;

    fn index(&self, name: &'a NodeName) -> &Self::Output {
        &self.nodes[name]
    }
}

impl NodeType {
    fn new(
        ContextFreeNodeType { name, kind }: ContextFreeNodeType,
        prev_rust_names: &mut PrevNodeRustNames,
    ) -> Self {
        let rust_names = NodeRustNames::new(&name, prev_rust_names);

        Self {
            name,
            rust_names,
            kind,
        }
    }

    pub fn rust_type_path(&self) -> Cow<'_, str> {
        self.rust_names.type_path()
    }

    pub fn anon_union_type_name<'a>(
        types: impl IntoIterator<Item = &'a NodeType, IntoIter: 'a>,
    ) -> impl Display + 'a {
        NodeRustNames::anon_union_type_name(types.into_iter().map(|t| &t.rust_names))
    }
}

impl Default for Children {
    fn default() -> Self {
        Self::EMPTY.clone()
    }
}

impl Children {
    /// Create a descriptor for a node with no children.
    ///
    /// Specifically, there are no child types. `required` and `multiple` are at the bottom of the
    /// lattice created by the [`Extend`] impl: that is, both are `false`.
    pub const EMPTY: Self = Self {
        multiple: false,
        required: false,
        types: Vec::new(),
    };

    /// Whether there are no children.
    pub fn is_empty(&self) -> bool {
        self.types.is_empty()
    }
}

impl BitOrAssign for Children {
    fn bitor_assign(&mut self, other: Self) {
        if other.is_empty() {
            return;
        } else if self.is_empty() {
            *self = other;
            return;
        }

        // If either original `Children` has at least 1 element, then this `Children` will.
        self.required |= other.required;

        // If either original `Children` may have multiple children, then this `Children` may.
        self.multiple |= other.multiple;

        // Add other child types, but no duplicates.
        // (We could use `IndexSet` instead of doing this manually, but it probably wouldn't even
        // have amortized performance gain because these are typically small, and it requires us to
        // either not define `const EMPTY` or add a dependency).
        let len = self.types.len();
        self.types.reserve(other.types.len());
        for child_type in other.types {
            if self
                .types
                .iter()
                .take(len)
                .any(|existing_type| *existing_type == child_type)
            {
                continue;
            }
            self.types.push(child_type);
        }
    }
}
