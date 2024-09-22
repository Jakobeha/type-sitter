use crate::queries::sexp::{Atom, GroupType, SExp};
use crate::{NodeModule, NodeName, NodeType, NodeTypeMap};
use quote::ToTokens;
use std::borrow::Cow;
use std::collections::VecDeque;
use syn::Path;

pub(super) enum SExpNodeType {
    Single { r#type: NodeType },
    Union { types: VecDeque<NodeType>, are_all_variants_named: bool },
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
            (Self::Single { r#type: self_type }, Self::Single { r#type }) => {
                let self_is_named = matches!(self_type.rust_names.module, NodeModule::Toplevel);
                let is_named = matches!(r#type.rust_names.module, NodeModule::Toplevel);
                Self::Union {
                    types: VecDeque::from([self_type, r#type]),
                    are_all_variants_named: self_is_named && is_named
                }
            }
            (
                Self::Union { mut types, are_all_variants_named },
                Self::Single { r#type }
            ) => {
                let is_named = matches!(r#type.rust_names.module, NodeModule::Toplevel);
                types.push_back(r#type);
                Self::Union {
                    types,
                    are_all_variants_named: are_all_variants_named && is_named
                }
            }
            (
                Self::Single { r#type: self_type },
                Self::Union { mut types, are_all_variants_named }
            ) => {
                let self_is_named = matches!(self_type.rust_names.module, NodeModule::Toplevel);
                types.push_front(self_type);
                Self::Union {
                    types,
                    are_all_variants_named: self_is_named && are_all_variants_named
                }
            }
            (
                Self::Union { types: mut self_types, are_all_variants_named: self_are_all_variants_named },
                Self::Union { mut types, are_all_variants_named }
            ) => {
                self_types.append(&mut types);
                Self::Union {
                    types: self_types,
                    are_all_variants_named: self_are_all_variants_named && are_all_variants_named
                }
            }
        }
    }

    pub(super) fn is_named(&self) -> bool {
        match self {
            Self::Untyped { is_named } => *is_named,
            Self::Single { r#type } => matches!(r#type.rust_names.module, NodeModule::Toplevel),
            Self::Union { are_all_variants_named, .. } => *are_all_variants_named
        }
    }

    // ???: Remove unnecessary allocating in all these path methods, especially this one?
    pub(super) fn rust_type_path(&self, nodes: &Path, capture_variant_name: &str) -> Cow<'_, str> {
        match self {
            Self::Untyped { is_named } => match is_named {
                false => Cow::Borrowed("type_sitter_lib::UntypedNode"),
                true => Cow::Borrowed("type_sitter_lib::UntypedNamedNode"),
            },
            Self::Single { r#type } => Cow::Owned(format!("{}::{}", nodes.to_token_stream().to_string().replace(" ", ""), r#type.rust_type_path())),
            Self::Union { .. } => Cow::Owned(format!("anon_unions::{}", capture_variant_name))
        }
    }
}

impl From<NodeType> for SExpNodeType {
    fn from(r#type: NodeType) -> Self {
        Self::Single { r#type }
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
    pub(super) fn node_type(&self, is_head: bool, all_types: &NodeTypeMap) -> SExpNodeType {
        match self {
            SExp::Atom { atom, .. } => atom.node_type(is_head, all_types),
            SExp::Group { group_type, items, .. } => match group_type {
                GroupType::Paren => match items.get(0) {
                    None => panic!("empty paren group isn't allowed in a tree-sitter query"),
                    Some(item) => item.node_type(true, all_types)
                },
                GroupType::Bracket => items.iter().map(|item| item.node_type(is_head, all_types)).collect()
            }
        }
    }
}

impl<'tree> Atom<'tree> {
    pub(super) fn node_type(&self, is_head: bool, all_types: &NodeTypeMap) -> SExpNodeType {
        match self {
            Atom::Wildcard => SExpNodeType::Untyped { is_named: is_head },
            Atom::Anchor => panic!("anchor doesn't have a node type (note: capturing an anchor isn't allowed in a tree-sitter query)"),
            Atom::Field { .. } => panic!("field doesn't have a node type (note: capturing a field isn't allowed in a tree-sitter query)"),
            Atom::Ident { name } => {
                let node_name = NodeName {
                    sexp_name: name.to_string(),
                    is_named: true,
                };

                let node_type = all_types
                    .get(&node_name)
                    .cloned()
                    .expect("identifier not found in node type map");

                SExpNodeType::from(node_type)
            },
            Atom::String { content } => {
                let node_name = NodeName {
                    sexp_name: content.to_string(),
                    is_named: false,
                };

                let node_type = all_types
                    .get(&node_name)
                    .cloned()
                    .expect("string not found in node type map");

                SExpNodeType::from(node_type)
            },
            Atom::Negation { .. } => SExpNodeType::Untyped { is_named: true },
            Atom::Capture { name } => panic!("capture doesn't have a node type (note: capturing a capture isn't allowed in a tree-sitter query; capture name = {})", name),
            Atom::Predicate { name } => panic!("predicate doesn't have a node type (internal error, a capture after a predicate should never make us try to check the predicate's type; predicate name = {})", name)
        }
    }
}
