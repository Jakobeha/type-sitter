use std::fmt::{Display, Formatter};
use crate::names::NodeName;
use crate::node_types::types::{Children, NodeType, NodeTypeKind};

/// Generate detailed documentation about a node type
pub struct DetailDoc<'a> {
    node_type: &'a NodeType
}

/// Display the "kind" of [`Children`], and provide a link to its inner type
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
                writeln!(f, "This node type has subtypes:\n")?;
                for subtype in subtypes {
                    writeln!(f, "- `{}` ([`{}`])", subtype.sexp_name, subtype.rust_type_path())?;
                }
            }
            NodeTypeKind::Regular { fields, children } => {
                if fields.is_empty() {
                    if let Some(children) = children.as_ref().filter(|c| !c.types.is_empty()) {
                        let a_child_or_children = match (children.required, children.multiple) {
                            (false, false) => "an optional named child",
                            (true, false) => "a named child",
                            (_, true) => "named children"
                        };
                        writeln!(f, "This node has {} of type {}", a_child_or_children, ChildrenKind::new(children, true))?;
                    } else {
                        writeln!(f, "This node has no named children")?;
                    }
                } else {
                    writeln!(f, "This node has these fields:\n")?;
                    for (field_name, field) in fields {
                        writeln!(f, "- `{}`: {}", field_name, ChildrenKind::new(field, false))?;
                    }
                    if let Some(children) = children {
                        let an_additional_child_or_children = match (children.required, children.multiple) {
                            (false, false) => "an optional additional named child",
                            (true, false) => "an additional named child",
                            (_, true) => "additional named children"
                        };
                        writeln!(f, "\nAnd {} of type {}", an_additional_child_or_children, ChildrenKind::new(children, true))?;
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
        if self.inline_anon_union_references && self.children.types.len() > 1 {
            writeln!(f, ":\n")?;
            for child in &self.children.types {
                writeln!(f, "- [`{}`]", child.rust_type_path())?;
            }
        } else {
            write!(f, " (")?;
            let mut iter = self.children.types.iter();
            if let Some(first) = iter.next() {
                write!(f, "[`{}`]", first.rust_type_path())?;
                for next in iter {
                    write!(f, " | [`{}`]", next.rust_type_path())?;
                }
            } else {
                write!(f, "[never](https://doc.rust-lang.org/std/primitive.never.html)")?;
            }
            write!(f, ")")?;
        }
        Ok(())
    }
}
