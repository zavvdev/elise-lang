//! # Annotated AST (AAST)
//!
//! The AAST is the output of semantic analysis and the input to the bytecode
//! emitter. It differs from the AST in two ways:
//!   - Identifiers are replaced with SymbolIds resolved against the SymbolTable.
//!   - Nodes are type-annotated and constant-folded where possible.
//!
//! The AAST is a compile-time only structure, discarded after bytecode emission.

mod data_types;
mod symbol_table;

use elise_shared::{shared_node_names::NodeName, shared_types::Span};

use crate::{data_types::AAstDataType, symbol_table::SymbolId};

#[derive(Debug, PartialEq)]
pub enum AAstNodeCall {
    Let {
        symbol_id: SymbolId,
        value: AAstNodeData,
        span: Span,
    },
    Typedef {
        // TODO: Do we need to store it in symbol table or maybe
        // some other record for type definitions only?
        span: Span,
        alias: String,
        dtype: AAstDataType,
    },
    Get {
        span: Span,
    },
    Add {
        span: Span,
    },
}

#[derive(Debug, PartialEq)]
pub enum AAstNodeData {
    Int { value: String, span: Span },
}

/// AAstNode must store primitive values as String type instead of
/// parsed values since emitter only needs to know the type in order
/// to emit a correct opcode. Parsing to correct value must be done
/// only during VM bytecode execution.
#[derive(Debug, PartialEq)]
pub enum AAstNode {
    Call(AAstNodeCall),
    Data(AAstNodeData),
    SymbolRef {
        symbol_id: SymbolId,
        span: Span,
        depth: usize,
    },
}

// String representations for AAstNode's in order to be able to
// use them for error reports.
impl AAstNode {
    pub fn span(&self) -> &Span {
        match self {
            AAstNode::SymbolRef { span, .. } => span,
            AAstNode::Call(call_type) => match call_type {
                AAstNodeCall::Let { span, .. } => span,
                AAstNodeCall::Typedef { span, .. } => span,
                AAstNodeCall::Add { span, .. } => span,
                AAstNodeCall::Get { span, .. } => span,
            },
            AAstNode::Data(data) => match data {
                AAstNodeData::Int { span, .. } => span,
            },
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            AAstNode::SymbolRef { .. } => NodeName::SYMBOL,
            AAstNode::Call(call_type) => match call_type {
                AAstNodeCall::Let { .. } => NodeName::FN_LET,
                AAstNodeCall::Typedef { .. } => NodeName::FN_TYPEDEF,
                AAstNodeCall::Add { .. } => NodeName::FN_ADD,
                AAstNodeCall::Get { .. } => NodeName::FN_GET,
            },
            AAstNode::Data(data) => match data {
                AAstNodeData::Int { .. } => NodeName::INT,
            },
        }
    }
}
