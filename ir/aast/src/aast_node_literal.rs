//! Notes that represent literal static data such as:
//! Int, Str, Dict and List.
//!
//! Nodes inside AAstNodeLiteral are intended to be static
//! literals, not computed values.

use elise_shared_proc_macros::EnumDelegate;
use elise_shared_types::Span;
use getset::{CloneGetters, CopyGetters};

#[derive(Debug, PartialEq, EnumDelegate)]
#[enum_delegate(
    fn span(&self) -> Span;
)]
pub enum AAstNodeLiteral {
    Int(AAstNodeLiteralPrim),
    Str(AAstNodeLiteralPrim),
    List(AAstNodeLiteralList),
    Dict(AAstNodeLiteralDict),
}

#[derive(Debug, PartialEq, CopyGetters, CloneGetters)]
pub struct AAstNodeLiteralPrim {
    #[getset(get_clone = "pub")]
    pub lexeme: String,

    #[getset(get_copy = "pub")]
    pub span: Span,
}

#[derive(Debug, PartialEq, CopyGetters)]
pub struct AAstNodeLiteralList {
    #[getset(get_copy = "pub")]
    pub span: Span,

    pub items: Vec<Box<AAstNodeLiteral>>,
}

#[derive(Debug, PartialEq, CopyGetters)]
pub struct AAstNodeLiteralDict {
    #[getset(get_copy = "pub")]
    pub span: Span,

    pub entries: Vec<(AAstNodeLiteralDictKey, Box<AAstNodeLiteral>)>,
}

#[derive(Debug, PartialEq, CloneGetters)]
pub struct AAstNodeLiteralDictKey {
    pub span: Span,

    #[getset(get_clone = "pub")]
    pub lexeme: String,
}
