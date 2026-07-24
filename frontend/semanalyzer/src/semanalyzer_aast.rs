//! # Annotated AST (AAST)
//!
//! The AAST is the output of semantic analysis and the input to the bytecode
//! emitter. It differs from the AST in two ways:
//!   - Identifiers are replaced with SymbolIds resolved against the SymbolTable.
//!   - Nodes are type-annotated and constant-folded where possible.
//!
//! The AAST is a compile-time only structure, discarded after bytecode emission.

use elise_shared::shared_types::Span;

use crate::semanalyzer_symbol_table::SymbolId;

/// AAstNode must store primitive values as String type instead of
/// parsed values since emitter only needs to know the type in order
/// to emit a correct opcode. Parsing to correct value must be done
/// only during VM bytecode execution.
#[derive(Debug, PartialEq)]
pub enum AAstNode {
    FDefine {
        symbol_id: SymbolId,
        value: Box<AAstNode>,
        span: Span,
    },
    FLet {
        bindings: Vec<(SymbolId, Box<AAstNode>)>,
        body: Vec<Box<AAstNode>>,
        span: Span,
    },
    FMul {
        operands: Vec<Box<AAstNode>>,
        span: Span,
    },
    FAdd {
        operands: Vec<Box<AAstNode>>,
        span: Span,
    },
    SymbolRef {
        symbol_id: SymbolId,
        span: Span,
        depth: usize,
    },
    Int {
        value: String,
        span: Span,
    },
    Float {
        value: String,
        span: Span,
    },
    String {
        value: String,
        span: Span,
    },
    Bool {
        value: bool,
        span: Span,
    },
    Null {
        span: Span,
    },
}

// String representations for AAstNode's in order to be able to
// use them for error reports.
impl AAstNode {
    pub const FDEFINE_STR: &'static str = "FDefine";
    pub const FLET_STR: &'static str = "FLet";
    pub const FMUL_STR: &'static str = "FMul";
    pub const FADD_STR: &'static str = "FAdd";
    pub const SYMBOL_REF_STR: &'static str = "SymbolRef";
    pub const INT_STR: &'static str = "Int";
    pub const FLOAT_STR: &'static str = "Float";
    pub const STRING_STR: &'static str = "String";
    pub const BOOL_STR: &'static str = "Bool";
    pub const NULL_STR: &'static str = "Null";

    pub fn span(&self) -> &Span {
        match self {
            AAstNode::FDefine { span, .. }
            | AAstNode::FLet { span, .. }
            | AAstNode::FMul { span, .. }
            | AAstNode::FAdd { span, .. }
            | AAstNode::SymbolRef { span, .. }
            | AAstNode::Int { span, .. }
            | AAstNode::Float { span, .. }
            | AAstNode::String { span, .. }
            | AAstNode::Bool { span, .. }
            | AAstNode::Null { span, .. } => span,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            AAstNode::FDefine { .. } => Self::FDEFINE_STR,
            AAstNode::FLet { .. } => Self::FLET_STR,
            AAstNode::FMul { .. } => Self::FMUL_STR,
            AAstNode::FAdd { .. } => Self::FADD_STR,
            AAstNode::SymbolRef { .. } => Self::SYMBOL_REF_STR,
            AAstNode::Int { .. } => Self::INT_STR,
            AAstNode::Float { .. } => Self::FLOAT_STR,
            AAstNode::String { .. } => Self::STRING_STR,
            AAstNode::Bool { .. } => Self::BOOL_STR,
            AAstNode::Null { .. } => Self::NULL_STR,
        }
    }
}
