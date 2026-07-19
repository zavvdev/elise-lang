//! # Annotated AST (AAST)
//!
//! The AAST is the output of semantic analysis and the input to the bytecode
//! emitter. It differs from the AST in two ways:
//!   - Identifiers are replaced with SymbolIds resolved against the SymbolTable.
//!   - Nodes are type-annotated and constant-folded where possible.
//!
//! The AAST is a compile-time only structure, discarded after bytecode emission.

use elise_shared_types::Span;

use crate::symbol_table::SymbolId;

#[derive(Debug, PartialEq)]
pub struct AAstPrimitive {
    pub value: String,
    pub span: Span,
}

/// AAstNode must store primitive values as String type instead of
/// parsed values since emitter only needs to know the type in order
/// to emit a correct opcode. Parsing to correct value must be done
/// only during VM bytecode execution.
#[derive(Debug, PartialEq)]
pub enum AAstNode {
    FDefine {
        symbol_id: SymbolId,
        // Storing value as a String directly instead of Box<AstNode>
        // since .define can create references to primitive types only
        // by design.
        value: String,
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
    Int(AAstPrimitive),
    Float(AAstPrimitive),
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

    pub fn span(&self) -> &Span {
        match self {
            AAstNode::FDefine { span, .. }
            | AAstNode::FLet { span, .. }
            | AAstNode::FMul { span, .. }
            | AAstNode::FAdd { span, .. }
            | AAstNode::SymbolRef { span, .. } => span,
            AAstNode::Int(c) | AAstNode::Float(c) => &c.span,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            AAstNode::FDefine { .. } => Self::FDEFINE_STR,
            AAstNode::FLet { .. } => Self::FLET_STR,
            AAstNode::FMul { .. } => Self::FMUL_STR,
            AAstNode::FAdd { .. } => Self::FADD_STR,
            AAstNode::SymbolRef { .. } => Self::SYMBOL_REF_STR,
            AAstNode::Int(_) => Self::INT_STR,
            AAstNode::Float(_) => Self::FLOAT_STR,
        }
    }
}
