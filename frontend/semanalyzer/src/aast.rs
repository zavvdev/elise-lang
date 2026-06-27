//! # Annotated AST (AAST)
//!
//! The AAST is the output of semantic analysis and the input to the bytecode
//! emitter. It differs from the AST in two ways:
//!   - Identifiers are replaced with SymbolIds resolved against the SymbolTable.
//!   - Nodes are type-annotated and constant-folded where possible.
//!
//! The AAST is a compile-time only structure, discarded after bytecode emission.

use elise_types::Span;

use crate::symbol_table::SymbolId;

/// AAstNode must store primitive values as String type instead of
/// parsed values since emitter only needs to know the type in order
/// to emit a correct opcode. Parsing to correct value must be done
/// only during VM bytecode execution.
#[derive(Debug)]
pub enum AAstNode {
    Define {
        symbol_id: SymbolId,
        // Storing value as a String directly instead of Box<AstNode>
        // since .define can create references to primitive types only
        // by design.
        value: String,
        span: Span,
    },
    Let {
        bindings: Vec<(SymbolId, Box<AAstNode>)>,
        body: Vec<Box<AAstNode>>,
        span: Span,
    },
    Mul {
        operands: Vec<Box<AAstNode>>,
        span: Span,
    },
    Add {
        operands: Vec<Box<AAstNode>>,
        span: Span,
    },
}
