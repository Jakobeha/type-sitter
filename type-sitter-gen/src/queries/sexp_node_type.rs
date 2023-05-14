use std::collections::VecDeque;
use crate::names::NodeName;
use crate::node_types::types::NodeModule;
use crate::queries::sexp::{Atom, GroupType, SExp};

pub(super) enum SExpNodeType {
    Single { name: NodeName },
    Union { names: VecDeque<NodeName>, are_all_variants_named: bool },
    Untyped { is_named: bool },
}

impl SExpNodeType {
    pub(super) fn union_with(self, other: Self) -> Self {
        match (self, other) {
            (this, Self::Untyped { is_named }) => {
                Self::Untyped { is_named: this.is_named() && is_named }
            }
            (Self::Untyped { is_named: self_is_named }, other) => {
                Self::Untyped { is_named: self_is_named && other.is_named() }
            },
            (Self::Single { name: self_name }, Self::Single { name }) => {
                let self_is_named = matches!(self_name.module, NodeModule::Toplevel);
                let is_named = matches!(name.module, NodeModule::Toplevel);
                Self::Union {
                    names: VecDeque::from([self_name, name]),
                    are_all_variants_named: self_is_named && is_named
                }
            }
            (Self::Union { mut names, are_all_variants_named }, Self::Single { name }) => {
                let is_named = matches!(name.module, NodeModule::Toplevel);
                names.push_back(name);
                Self::Union {
                    names,
                    are_all_variants_named: are_all_variants_named && is_named
                }
            }
            (Self::Single { name: self_name }, Self::Union { mut names, are_all_variants_named }) => {
                let self_is_named = matches!(self_name.module, NodeModule::Toplevel);
                names.push_front(self_name);
                Self::Union {
                    names,
                    are_all_variants_named: self_is_named && are_all_variants_named
                }
            }
            (Self::Union { names: mut self_names, are_all_variants_named: self_are_all_variants_named }, Self::Union { mut names, are_all_variants_named }) => {
                self_names.append(&mut names);
                Self::Union {
                    names: self_names,
                    are_all_variants_named: self_are_all_variants_named && are_all_variants_named
                }
            }
        }
    }

    pub(super) fn is_named(&self) -> bool {
        match self {
            Self::Untyped { is_named } => *is_named,
            Self::Single { name } => matches!(name.module, NodeModule::Toplevel),
            Self::Union { are_all_variants_named, .. } => *are_all_variants_named
        }
    }
}

impl From<NodeName> for SExpNodeType {
    fn from(name: NodeName) -> Self {
        Self::Single { name }
    }
}

impl FromIterator<SExpNodeType> for SExpNodeType {
    fn from_iter<T: IntoIterator<Item=SExpNodeType>>(iter: T) -> Self {
        let mut iter = iter.into_iter();
        let mut this = iter.next()
            .expect("can't create s-expression node type from empty iterator (shouldn't have tree-sitter query with empty union)");
        for next in iter {
            this = this.union_with(next);
        }
        this
    }
}

impl<'tree> SExp<'tree> {
    pub(super) fn node_type(&self, is_head: bool) -> SExpNodeType {
        match self {
            SExp::Atom { atom, .. } => atom.node_type(is_head),
            SExp::Group { group_type, items, .. } => match group_type {
                GroupType::Paren => match items.get(0) {
                    None => panic!("empty paren group is not in a valid tree-sitter query"),
                    Some(item) => item.node_type(true)
                },
                GroupType::Bracket => items.iter().map(|item| item.node_type(is_head)).collect()
            }
        }
    }
}

impl<'tree> Atom<'tree> {
    pub(super) fn node_type(&self, is_head: bool) -> SExpNodeType {
        match self {
            Atom::Wildcard => SExpNodeType::Untyped { is_named: is_head },
            Atom::Anchor => panic!("capturing an anchor is not in a valid tree-sitter query"),
            Atom::Field { .. } => panic!("capturing a field is not in a valid tree-sitter query"),
            Atom::Ident { name } => SExpNodeType::from(NodeName::new(name.to_string(), true)),
            Atom::String { content } => SExpNodeType::from(NodeName::new(content.to_string(), false)),
            Atom::Negation { .. } => SExpNodeType::Untyped { is_named: true },
            Atom::Capture { name } => panic!("capturing a capture is not in a valid tree-sitter query (captured capture name = {})", name),
            Atom::Predicate { name } => panic!("capturing a predicate is not in a valid tree-sitter query (captured predicate name = {})", name)
        }
    }
}
