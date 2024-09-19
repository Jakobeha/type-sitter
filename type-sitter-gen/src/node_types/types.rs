use serde::Deserialize;
use indexmap::IndexMap;
use crate::names::NodeName;

#[derive(Clone, Deserialize)]
pub struct NodeType {
    #[serde(flatten)]
    pub name: NodeName,
    #[serde(flatten)]
    pub kind: NodeTypeKind
}

#[derive(Clone, Deserialize)]
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

#[derive(Clone, Deserialize)]
pub struct Children {
    pub multiple: bool,
    pub required: bool,
    pub types: Vec<NodeName>,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum NodeModule {
    Toplevel,
    Unnamed,
    Symbols
}

impl Extend<Children> for Children {
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
                if self.types.iter().take(len).any(|existing_type| &existing_type.rust_type_name == &child_type.rust_type_name) {
                    continue;
                }
                self.types.push(child_type);
            }
        }
    }
}