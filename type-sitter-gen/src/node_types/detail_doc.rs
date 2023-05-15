use std::fmt::{Display, Formatter};
use crate::names::NodeName;
use crate::node_types::types::{Children, NodeType, NodeTypeKind};

/// Generate detailed documentation about a node type
pub struct DetailDoc<'a> {
    node_type: &'a NodeType
}

/// Display the "kind" of [Children], and provide a link to its inner type
pub struct ChildrenKind<'a> {
    children: &'a Children,
    inline_anon_union_references: bool
}

impl<'a> DetailDoc<'a> {
    pub fn new(node_type: &'a NodeType) -> Self {
        Self { node_type }
    }
}

impl<'a> Display for DetailDoc<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.node_type.kind {
            NodeTypeKind::Supertype { subtypes } => {
                writeln!(f, "This node type is a supertype of:")?;
                for subtype in subtypes {
                    writeln!(f, "- `{}` ([{}])", subtype.sexp_name, subtype.rust_type_path())?;
                }
            }
            NodeTypeKind::Regular { fields, children } => {
                match fields.is_empty() {
                    false => {
                        writeln!(f, "This node has these fields:")?;
                        for (field_name, field) in fields {
                            writeln!(f, "- `{}`: {}", field_name, ChildrenKind::new(field, false))?;
                        }
                        if let Some(children) = children {
                            let an_additional_child_or_children = match (children.required, children.multiple) {
                                (false, false) => "an additional (optional) child",
                                (true, false) => "an additional child",
                                (_, true) => "additional children"
                            };
                            writeln!(f, "\nAnd {}: {}", an_additional_child_or_children, ChildrenKind::new(children, true))?;
                        }
                    }
                    true => {
                        match children.as_ref().filter(|c| !c.types.is_empty()) {
                            None => writeln!(f, "This node has no children")?,
                            Some(children) => {
                                let this_child_or_these_children = match (children.required, children.multiple) {
                                    (false, false) => "an (optional) child",
                                    (true, false) => "a child",
                                    (_, true) => "children"
                                };
                                writeln!(f, "This node has {}: {}", this_child_or_these_children, ChildrenKind::new(children, true))?;
                            }
                        }
                    }
                }
            }
        }
        Ok(())
    }
}

impl<'a> ChildrenKind<'a> {
    pub fn new(children: &'a Children, inline_anon_union_references: bool) -> Self {
        Self { children, inline_anon_union_references }
    }
}

impl<'a> Display for ChildrenKind<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let kind = NodeName::kind(&self.children.types);
        let kind_suffix = match (self.children.required, self.children.multiple) {
            (false, false) => "?",
            (true, false) => "",
            (false, true) => "*",
            (true, true) => "+"
        };
        write!(f, "`{}{}`", kind, kind_suffix)?;
        match !self.inline_anon_union_references || self.children.types.len() < 2 {
            false => {
                writeln!(f, ":")?;
                for child in &self.children.types {
                    writeln!(f, "- [{}]", child.rust_type_path())?;
                }
            },
            true => {
                let rust_type_path = NodeName::rust_type_path_of(&self.children.types);
                write!(f, " ([{}])", rust_type_path)?
            }
        }
        Ok(())
    }
}
