use elise_shared_proc_macros::EnumDelegate;
use elise_shared_types::Span;
use getset::{CloneGetters, CopyGetters};

#[derive(Debug, PartialEq, Clone, CopyGetters, CloneGetters)]
pub struct AstNodeTypeAnnot {
    #[getset(get_copy = "pub")]
    pub span: Span,

    #[getset(get_clone = "pub")]
    pub lexeme: String,

    // From the perspective of AST, any type can have a generic.
    // Since AST is only focused on code structure/grammar,
    // we don't care here what kind of type we're currently in,
    // although semantically :Int cannot have a generic type.
    pub generic: Option<AstNodeTypeAnnotGeneric>,
}

#[derive(Debug, PartialEq, Clone, EnumDelegate)]
#[enum_delegate(
    fn span(&self) -> Span;
)]
pub enum AstNodeTypeAnnotGeneric {
    Single(AstNodeTypeAnnotGenericSingle),
    Record(AstNodeTypeAnnotGenericRecord),
}

#[derive(Debug, PartialEq, Clone, CopyGetters)]
pub struct AstNodeTypeAnnotGenericSingle {
    pub annot: Box<AstNodeTypeAnnot>,

    #[getset(get_copy = "pub")]
    pub span: Span,
}

#[derive(Debug, PartialEq, Clone, CopyGetters)]
pub struct AstNodeTypeAnnotGenericRecord {
    pub entries: Vec<(AstNodeTypeAnnotRecordKey, Box<AstNodeTypeAnnot>)>,

    #[getset(get_copy = "pub")]
    pub span: Span,
}

#[derive(Debug, PartialEq, Clone, CloneGetters, CopyGetters)]
pub struct AstNodeTypeAnnotRecordKey {
    #[getset(get_clone = "pub")]
    pub lexeme: String,

    #[getset(get_copy = "pub")]
    pub span: Span,
}
