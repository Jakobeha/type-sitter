use crate::node_types::NodeTypeMap;
use crate::{Children, NodeName, NodeType, NodeTypeKind};
use std::fmt::{Display, Formatter};

/// Generate detailed documentation about a node type
pub(super) struct DetailDoc<'a> {
    node_type: &'a NodeType,
    all_types: &'a NodeTypeMap,
}

/// Display the "kind" of [`Children`], and provide a link to its inner type
pub(super) struct ChildrenKind<'a> {
    children: &'a Children,
    inline_anon_union_references: bool,
    all_types: &'a NodeTypeMap,
}

impl<'a> DetailDoc<'a> {
    pub(super) fn new(node_type: &'a NodeType, all_types: &'a NodeTypeMap) -> Self {
        Self {
            node_type,
            all_types,
        }
    }
}

impl<'a> Display for DetailDoc<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.node_type.kind {
            NodeTypeKind::Supertype {
                subtypes: subtype_names,
            } => {
                writeln!(f, "This node type has subtypes:\n")?;

                for subtype_name in subtype_names {
                    let subtype = &self.all_types[subtype_name];
                    writeln!(
                        f,
                        "- `{}` ([`{}`])",
                        subtype_name.sexp_name,
                        subtype.rust_type_path()
                    )?;
                }
            }
            NodeTypeKind::Regular { fields, children } => {
                if fields.is_empty() {
                    if children.is_empty() {
                        writeln!(f, "This node has no named children")?;
                    } else {
                        let a_child_or_children = match (children.required, children.multiple) {
                            (false, false) => "an optional named child",
                            (true, false) => "a named child",
                            (_, true) => "named children",
                        };
                        writeln!(
                            f,
                            "This node has {} of type {}",
                            a_child_or_children,
                            ChildrenKind::new(children, true, self.all_types)
                        )?;
                    }
                } else {
                    writeln!(f, "This node has these fields:\n")?;

                    for (field_name, field) in fields {
                        writeln!(
                            f,
                            "- `{}`: {}",
                            field_name,
                            ChildrenKind::new(field, false, self.all_types)
                        )?;
                    }

                    if !children.is_empty() {
                        let an_additional_child_or_children =
                            match (children.required, children.multiple) {
                                (false, false) => "an optional additional named child",
                                (true, false) => "an additional named child",
                                (_, true) => "additional named children",
                            };
                        writeln!(
                            f,
                            "\nAnd {} of type {}",
                            an_additional_child_or_children,
                            ChildrenKind::new(children, true, self.all_types)
                        )?;
                    }
                }
            }
        }
        Ok(())
    }
}

impl<'a> ChildrenKind<'a> {
    pub(super) fn new(
        children: &'a Children,
        inline_anon_union_references: bool,
        all_types: &'a NodeTypeMap,
    ) -> Self {
        Self {
            children,
            inline_anon_union_references,
            all_types,
        }
    }
}

impl<'a> Display for ChildrenKind<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let kind = NodeName::kind(&self.children.types);
        let kind_suffix = match (self.children.required, self.children.multiple) {
            (false, false) => "?",
            (true, false) => "",
            (false, true) => "*",
            (true, true) => "+",
        };
        write!(f, "`{}{}`", kind, kind_suffix)?;

        if self.inline_anon_union_references && self.children.types.len() > 1 {
            writeln!(f, ":\n")?;

            for child_name in &self.children.types {
                let child = &self.all_types[child_name];
                writeln!(f, "- [`{}`]", child.rust_type_path())?;
            }
        } else {
            write!(f, " (")?;

            let mut iter = self.children.types.iter();
            if let Some(first_name) = iter.next() {
                let first = &self.all_types[first_name];
                write!(f, "[`{}`]", first.rust_type_path())?;

                for next_name in iter {
                    let next = &self.all_types[next_name];
                    write!(f, " | [`{}`]", next.rust_type_path())?;
                }
            } else {
                write!(
                    f,
                    "[never](https://doc.rust-lang.org/std/primitive.never.html)"
                )?;
            }

            write!(f, ")")?;
        }
        Ok(())
    }
}
