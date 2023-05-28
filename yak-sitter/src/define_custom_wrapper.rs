/// Macro which lets you generate your own `tree-sitter` wrappers, which will be versions of the
/// [yak_sitter] ones with a defined `Custom`.
///
/// ## Usage
/// ```rust
/// use std::collections::HashMap;
/// use yak_sitter::{define_custom_wrapper, NodeId};
///
/// pub struct MyCustom {
///    pub my_data: HashMap<NodeId, usize>,
/// }
///
/// define_custom_wrapper!(MyCustom);
/// ```
#[macro_export]
macro_rules! define_custom_wrapper {
    ($Custom:ty) => {
        pub type Tree = $crate::Tree<$Custom>;
        pub type Node<'tree> = $crate::Node<'tree, $Custom>;
        pub type NodePtr = $crate::NodePtr<$Custom>;
        pub type TreeCursor<'tree> = $crate::TreeCursor<'tree, $Custom>;
        pub type QueryMatches<'query, 'tree> = $crate::QueryMatches<'query, 'tree, $Custom>
        pub type QueryMatch<'query, 'tree> = $crate::QueryMatch<'query, 'tree, $Custom>
        pub type QueryCaptures<'query, 'tree> = $crate::QueryCaptures<'query, 'tree, $Custom>
        pub type QueryCapture<'query, 'tree> = $crate::QueryCapture<'query, 'tree, $Custom>
        pub type SubTree<'tree> = $crate::SubTree<'tree, $Custom>
    };
}