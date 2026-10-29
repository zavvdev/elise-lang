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

#[derive(Debug, PartialEq)]
pub struct AAstNodeDataDictKey {
    pub span: Span,
    pub lexeme: String,
}
