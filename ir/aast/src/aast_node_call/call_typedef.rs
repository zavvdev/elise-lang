use elise_shared_types::Span;
use getset::CopyGetters;

use crate::aast_node_call::type_annot::AAstNodeTypeAnnot;

#[derive(Debug, PartialEq, CopyGetters)]
pub struct AAstNodeCallTypedef {
    // Raw alias for a new type created with
    // .typedef function.
    pub alias: String,
    pub annot: AAstNodeTypeAnnot,

    #[getset(get_copy = "pub")]
    pub span: Span,
}
