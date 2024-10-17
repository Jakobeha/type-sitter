use crate::node_types::rust_names::PrevNodeRustNames;
use crate::{NodeName, NodeRustNames};
use serde::Deserialize;
use std::borrow::Cow;
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::fmt::Display;
use std::ops::{BitOrAssign, Index};
use std::path::{Path, PathBuf};
use crate::vec_set::VecSet;

use super::deserialize_json_array_as_stream::iter_json_array;

/// Represents the contents of the `node-types.json` in a form that can be converted to Rust code.
///
/// Use [`crate::generate_nodes()`] the Rust code after inspecting and
#[derive(Debug)]
pub struct NodeTypeMap { nodes: HashMap<NodeName, NodeType>, prev_rust_names: PrevNodeRustNames }

/// Describes a grammar node; corresponds to an entry in `node-types.json`
#[derive(Clone, Debug)]
pub struct NodeType {
    pub name: NodeName,
    pub kind: NodeTypeKind,
    pub(crate) rust_names: NodeRustNames,
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
    Supertype { subtypes: BTreeSet<NodeName> },
    Regular {
        #[serde(default)]
        fields: BTreeMap<String, Children>,
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
    pub types: VecSet<NodeName>,
}

impl NodeTypeMap {
    pub(crate) fn new(node_types: Vec<ContextFreeNodeType>) -> Self {
        let mut prev_rust_names = PrevNodeRustNames::new();
        let nodes = node_types.into_iter()
            .map(|node_type| (node_type.name.clone(), NodeType::new(node_type, &mut prev_rust_names)))
            .collect();

        Self { nodes, prev_rust_names }
    }

    pub(crate) fn get(&self, name: &NodeName) -> Option<&NodeType> {
        self.nodes.get(name)
    }

    /// Get all `NodeType`s
    ///
    /// # Example
    ///
    /// Checking that the grammar contains a node with a certain name:
    ///
    /// ```rust
    /// # use type_sitter_gen::*;
    /// # fn main() {
    /// let node_type_map = NodeTypeMap::try_from(tree_sitter_rust::NODE_TYPES)
    ///         .unwrap();
    /// assert!(node_type_map.values()
    ///     .any(|node| node.name.sexp_name == "trait_item"));
    /// # }
    /// ```
    pub fn values(&self) -> impl Iterator<Item = &NodeType> {
        self.nodes.values()
    }

    /// Add a supertype that isn't yet defined in the grammar.
    ///
    /// This is meant for cases where the original grammar doesn't include a supertype
    /// that you would like to have in your use case AST. The most common case would
    /// be to add an `AllNodes` supertype (see example below).
    ///
    /// # Example
    ///
    /// Adding supertypes for all named and unnamed nodes, and another one for all nodes:
    ///
    /// ```rust
    /// # fn main() {
    /// # use type_sitter_gen::*;
    /// let mut node_type_map = NodeTypeMap::try_from(tree_sitter_rust::NODE_TYPES).unwrap();
    ///
    /// let (named, unnamed): (Vec<_>, Vec<_>) = node_type_map
    ///     .values()
    ///     .map(|node| node.name.clone())
    ///     .partition(|name| name.is_named);
    ///
    /// let named = node_type_map
    ///     .add_custom_supertype("_all_named", named)
    ///     .expect("this shouldn't already exist");
    ///
    /// let unnamed = node_type_map
    ///     .add_custom_supertype("_all_unnamed", unnamed)
    ///     .expect("this shouldn't already exist");
    ///
    /// node_type_map
    ///     .add_custom_supertype("_all_nodes", vec![named, unnamed])
    ///     .expect("this shouldn't already exist");
    ///
    /// let code = generate_nodes(node_type_map).unwrap().into_string();
    ///
    /// assert!(code.contains("pub enum AllNamed"));
    /// assert!(code.contains("pub enum AllUnnamed"));
    /// assert!(code.contains("pub enum AllNodes"));
    /// # }
    /// ```
    ///
    /// # Panics
    ///
    /// Supertype names must be hidden (i.e. start with an underscore).
    /// Therefore this function panics when `name` doesn't have a leading `_`.
    ///
    /// ```rust should_panic
    /// # fn main() {
    /// # use type_sitter_gen::*;
    /// # let mut node_type_map = NodeTypeMap::try_from(tree_sitter_rust::NODE_TYPES).unwrap();
    /// # let _ =
    /// node_type_map.add_custom_supertype("my_supertype", vec![]);  // Panic!
    /// # }
    /// ```
    ///
    /// # Return Value
    ///
    /// Returns `Ok(new_node_name)` if the new supertype has been add ed,
    /// or `Err(existing_node_name)` if a node with that name already exists:
    ///
    /// ```rust
    /// # fn main() {
    /// # use type_sitter_gen::*;
    /// let mut node_type_map = NodeTypeMap::try_from(tree_sitter_rust::NODE_TYPES).unwrap();
    ///
    /// let declarations: Vec<NodeName> = node_type_map.values()
    /// #   .filter(|node| node.name.sexp_name == "struct_item" || node.name.sexp_name == "enum_item")
    ///     // select some declarations
    ///     .map(|node| node.name.clone())
    ///     .collect();
    ///
    /// let result = node_type_map.add_custom_supertype("_declaration_statement", declarations.clone());
    /// assert!(result.is_err());  // the Rust grammar already defines this name
    ///
    /// let result = node_type_map.add_custom_supertype("_my_declaration_statement", declarations);
    /// assert!(result.is_ok());
    /// # }
    /// ```
    pub fn add_custom_supertype(&mut self, name: &str, subtypes: impl IntoIterator<Item=NodeName>)
        -> Result<NodeName, NodeName>
    {
        // Supertypes should be hidden nodes, so ensure the leading underscore.
        if !name.starts_with("_") {
            panic!("Illegal supertype name '{name}'. Supertypes must start with an underscore, i.e. '_{name}'.");
        }

        let name = NodeName {
            sexp_name: name.to_owned(),
            is_named: true,
        };
        let new_node = ContextFreeNodeType {
            name: name.clone(),
            kind: NodeTypeKind::Supertype { subtypes: BTreeSet::from_iter(subtypes) },
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

    /// Read and parse the file at the path, which should be a `node-types.json`.
    fn try_from(path: &Path) -> Result<Self, Self::Error> {
        let text = std::fs::read_to_string(path)?;
        NodeTypeMap::try_from(text.as_str())
    }
}

impl TryFrom<PathBuf> for NodeTypeMap {
    type Error = crate::Error;

    /// Read and parse the file at the path, which should be a `node-types.json`.
    fn try_from(value: PathBuf) -> Result<Self, Self::Error> {
        Self::try_from(value.as_path())
    }
}

impl TryFrom<&str> for NodeTypeMap {
    type Error = crate::Error;

    /// Parse the given text, which should be the content of a `node-types.json` or
    /// a tree-sitter `NODE_TYPES` constant.
    fn try_from(text: &str) -> Result<Self, Self::Error> {
        let elems = iter_json_array::<ContextFreeNodeType, _>(text.as_bytes())
            .collect::<Result<Vec<_>, _>>()?;
        Ok(NodeTypeMap::new(elems))
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

impl Children {
    /// Create a descriptor for a node with no children.
    ///
    /// Specifically, there are no child types. `required` and `multiple` are at the bottom of the
    /// lattice created by the [`Extend`] impl: that is, both are `false`.
    pub const EMPTY: Self = Self { multiple: false, required: false, types: VecSet::new() };

    /// Whether there are no children.
    pub fn is_empty(&self) -> bool {
        self.types.is_empty()
    }
}

impl Default for Children {
    fn default() -> Self {
        Self::EMPTY.clone()
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
        self.types.extend(other.types);
    }
}
