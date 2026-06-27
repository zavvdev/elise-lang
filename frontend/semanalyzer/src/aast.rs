use elise_types::Span;

use crate::symbol_table::SymbolId;

// Captured symbols by closure.
type Captures = Vec<SymbolId>;

#[derive(Debug)]
pub enum AAstNode {
    Define {
        symbol_id: SymbolId,
        value: String,
        span: Span,
    },
    Let {
        bindings: Vec<(SymbolId, Box<AAstNode>)>,
        body: Vec<Box<AAstNode>>,
        span: Span,
        captures: Captures,
    },
    Mul {
        operands: Vec<AAstNode>,
        span: Span,
        captures: Captures,
    },
    Add {
        operands: Vec<AAstNode>,
        span: Span,
        captures: Captures,
    },
}
