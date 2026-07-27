//! # Annotated AST (AAST)
//!
//! The AAST is the output of semantic analysis and the input to the bytecode
//! emitter. It differs from the AST in two ways:
//!   - Identifiers are replaced with SymbolIds resolved against the SymbolTable.
//!   - Nodes are type-annotated and constant-folded where possible.
//!
//! The AAST is a compile-time only structure, discarded after bytecode emission.

use elise_shared::{shared_node_names::NodeName, shared_types::Span};

use crate::semanalyzer_symbol_table::SymbolId;

/// AAstNode must store primitive values as String type instead of
/// parsed values since emitter only needs to know the type in order
/// to emit a correct opcode. Parsing to correct value must be done
/// only during VM bytecode execution.
#[derive(Debug, PartialEq)]
pub enum AAstNode {
    CallDefine {
        symbol_id: SymbolId,
        value: Box<AAstNode>,
        span: Span,
    },
    CallLet {
        bindings: Vec<(SymbolId, Box<AAstNode>)>,
        body: Vec<Box<AAstNode>>,
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
    pub fn span(&self) -> &Span {
        match self {
            AAstNode::CallDefine { span, .. }
            | AAstNode::CallLet { span, .. }
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
            AAstNode::CallDefine { .. } => NodeName::CALL_DEFINE,
            AAstNode::CallLet { .. } => NodeName::CALL_LET,
            AAstNode::SymbolRef { .. } => NodeName::SYMBOL,
            AAstNode::Int { .. } => NodeName::INT,
            AAstNode::Float { .. } => NodeName::FLOAT,
            AAstNode::String { .. } => NodeName::STRING,
            AAstNode::Bool { .. } => NodeName::BOOL,
            AAstNode::Null { .. } => NodeName::NULL,
        }
    }
}
