use elise_shared_proc_macros::EnumDelegate;
use elise_shared_types::Span;
use getset::CopyGetters;

#[derive(Debug, PartialEq, Clone, EnumDelegate)]
#[enum_delegate(
    fn span(&self) -> Span;
)]
pub enum AAstNodeTypeAnnot {
    // Represents a reference to a custom data type.
    // For example:
    // .typedef (User :UserShape)
    // where :UserShape is a reference to the custom
    // data type.
    Custom(AAstNodeTypeAnnotCustom),
    Record(AAstNodeTypeAnnotRecord),
    List(AAstNodeTypeAnnotList),
    Int(AAstNodeTypeAnnotPrim),
}

// ==================================================================
//
// CUSTOM START
//
// ==================================================================

#[derive(Debug, PartialEq, Clone, CopyGetters)]
pub struct AAstNodeTypeAnnotCustom {
    #[getset(get_copy = "pub")]
    pub span: Span,

    pub alias: String,
}

// ==================================================================
//
// CUSTOM END
//
// ==================================================================

// ==================================================================
//
// RECORD START
//
// ==================================================================

pub type AAstNodeTypeAnnotRecordEntries = Vec<(String, Box<AAstNodeTypeAnnot>)>;

#[derive(Debug, PartialEq, Clone, CopyGetters)]
pub struct AAstNodeTypeAnnotRecord {
    #[getset(get_copy = "pub")]
    pub span: Span,

    pub entries: AAstNodeTypeAnnotRecordEntries,
}

// ==================================================================
//
// RECORD END
//
// ==================================================================

// ==================================================================
//
// LIST START
//
// ==================================================================

#[derive(Debug, PartialEq, Clone, CopyGetters)]
pub struct AAstNodeTypeAnnotList {
    #[getset(get_copy = "pub")]
    pub span: Span,

    pub item_type: Box<AAstNodeTypeAnnot>,
}

// ==================================================================
//
// LIST END
//
// ==================================================================

// ==================================================================
//
// PRIM START
//
// ==================================================================

#[derive(Debug, PartialEq, Clone, CopyGetters)]
pub struct AAstNodeTypeAnnotPrim {
    #[getset(get_copy = "pub")]
    pub span: Span,
}

// ==================================================================
//
// PRIM END
//
// ==================================================================
