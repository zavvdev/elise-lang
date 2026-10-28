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

#[derive(Debug, PartialEq, Clone)]
pub enum AstNodeTypeAnnotGeneric {
    Single(Box<AstNodeTypeAnnot>),
    Record(Vec<(AstNodeTypeAnnotRecordKey, Box<AstNodeTypeAnnot>)>),
}

#[derive(Debug, PartialEq, Clone, CloneGetters)]
pub struct AstNodeTypeAnnotRecordKey {
    #[getset(get_clone = "pub")]
    pub lexeme: String,

    pub span: Span,
}
