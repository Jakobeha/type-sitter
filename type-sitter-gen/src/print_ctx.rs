use crate::NodeTypeMap;
use syn::Path;

/// General arguments to both node and query printers.
#[derive(Clone, Copy)]
pub struct PrintCtx<'a> {
    pub all_types: &'a NodeTypeMap,
    pub tree_sitter: &'a Path,
    pub type_sitter_lib: &'a Path,
}
