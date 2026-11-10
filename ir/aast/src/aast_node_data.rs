//! Notes that represent literal static data such as:
//! Int, Str, Dict and List.
//!
//! Nodes inside AAstNodeData are intended to be static
//! literals, not computed values.

use elise_shared_proc_macros::EnumDelegate;
use elise_shared_types::Span;
use getset::{CloneGetters, CopyGetters};

#[derive(Debug, PartialEq, EnumDelegate)]
#[enum_delegate(
    fn span(&self) -> Span;
)]
pub enum AAstNodeData {
    Int(AAstNodeDataPrim),
    Str(AAstNodeDataPrim),
    List(AAstNodeDataList),
    Dict(AAstNodeDataDict),
}

#[derive(Debug, PartialEq, CopyGetters, CloneGetters)]
pub struct AAstNodeDataPrim {
    #[getset(get_clone = "pub")]
    pub lexeme: String,

    #[getset(get_copy = "pub")]
    pub span: Span,
}

#[derive(Debug, PartialEq, CopyGetters)]
pub struct AAstNodeDataList {
    #[getset(get_copy = "pub")]
    pub span: Span,

    pub items: Vec<Box<AAstNodeData>>,
}

#[derive(Debug, PartialEq, CopyGetters)]
pub struct AAstNodeDataDict {
    #[getset(get_copy = "pub")]
    pub span: Span,

    pub entries: Vec<(AAstNodeDataDictKey, Box<AAstNodeData>)>,
}

#[derive(Debug, PartialEq, CloneGetters)]
pub struct AAstNodeDataDictKey {
    pub span: Span,

    #[getset(get_clone = "pub")]
    pub lexeme: String,
}
