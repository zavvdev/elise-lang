//! # Harmony — Semantic Analyzer
//!
//! Transforms an AST into a HIR (High-level Intermediate Representation)
//! by walking the AST and performing semantic validation and annotation.
//!
//! ## Input
//!   - AST produced by the parser
//!   - DataBindingTable produced by the Binder (validated data + schema)
//!
//! ## Output
//!   - HIR { SymbolTable, AAST }
//!
//! ## What Harmony does
//!   - Resolves identifiers into SymbolIds and registers them in the SymbolTable
//!   - Validates language rules (arity, type constraints, redefinition etc.)
//!   - Annotates AST nodes with type information derived from schema and literals
//!   - Folds constants where all operands are known at compile time
//!   - Resolves data references against DataBindingTable to derive types
//!
//! ## What Harmony does NOT do
//!   - Store runtime values in the SymbolTable (type only, value lives in AAST)
//!   - Emit bytecode (that is the emitter's responsibility)
//!   - Interpret values beyond what is necessary for constant folding and
//!     compile-time optimizations (full interpretation is the VM's responsibility)
//!
//! By the time HIR reaches the emitter, all semantic guarantees are established
//! and the emitter can trust the AAST without re-validation.

pub mod aast;
pub mod data_types;
pub mod scope_stack;
pub mod symbol_table;

use elise_ast::{AstCallKind, AstCompound, AstNode, AstPrimitive};
use elise_binder::DataBindingTable;
use elise_errors::errors_semantic_analyzer::SemanticAnalyzerErr;

use crate::{aast::AAstNode, scope_stack::ScopeStack, symbol_table::SymbolTable};

#[derive(Debug)]
pub struct HIR {
    pub symbol_table: SymbolTable,
    pub aast: Vec<AAstNode>,
}

pub struct Harmony<'a> {
    pub ast: &'a Vec<AstNode>,
    pub data_binding_table: &'a DataBindingTable,
    pub scope_stack: ScopeStack,
}

impl<'a> Harmony<'a> {
    pub fn new(ast: &'a Vec<AstNode>, data_binding_table: &'a DataBindingTable) -> Self {
        // In order to have a global scope we push a new one
        // before analyzing AST, so the first stack frame is
        // our genesis scope.
        let mut scope_stack = ScopeStack::new();
        scope_stack.push();
        Self {
            ast,
            data_binding_table,
            scope_stack,
        }
    }

    pub fn analyze(&self) -> Result<HIR, SemanticAnalyzerErr> {
        let mut symbol_table = SymbolTable::new();
        let mut aast: Vec<AAstNode> = vec![];

        for ast_node in self.ast {
            if let Some(aast_node) = Self::get_aast_node(ast_node, &mut symbol_table) {
                aast.push(aast_node);
            }
        }

        Ok(HIR { symbol_table, aast })
    }

    fn get_aast_node(ast_node: &AstNode, symbol_table: &mut SymbolTable) -> Option<AAstNode> {
        match ast_node {
            AstNode::Call((call_kind, compound)) => {
                Self::annotate_call(call_kind, compound, symbol_table)
            }
            AstNode::Identifier(primitive) => Self::annotate_identifier(primitive, symbol_table),
            AstNode::Number(primitive) => Self::annotate_number(primitive, symbol_table),
            _ => None,
        }
    }

    fn annotate_call(
        _call_kind: &AstCallKind,
        _compound: &AstCompound,
        _symbol_table: &mut SymbolTable,
    ) -> Option<AAstNode> {
        None
    }

    fn annotate_identifier(
        _primitive: &AstPrimitive,
        _symbol_table: &mut SymbolTable,
    ) -> Option<AAstNode> {
        None
    }

    fn annotate_number(
        _primitive: &AstPrimitive,
        _symbol_table: &mut SymbolTable,
    ) -> Option<AAstNode> {
        None
    }
}

// ==================================================================
//
//  TESTS START
//
// ==================================================================

#[cfg(test)]
mod tests {
    // ==================================================================
    //  NUMBER TESTS START
    // ==================================================================

    #[test]
    fn number_should_annotate_integers() {}

    #[test]
    fn number_should_annotate_floats() {}

    // ==================================================================
    //  NUMBER TESTS END
    // ==================================================================

    // ==================================================================
    //  IDENTIFIER TESTS START
    // ==================================================================

    #[test]
    fn identifier_should_annotate_correctly() {}

    // ==================================================================
    //  IDENTIFIER TESTS END
    // ==================================================================

    // ==================================================================
    //  DEFINE FN TESTS START
    // ==================================================================

    #[test]
    fn fndefine_should_annotate_correctly() {}

    // ==================================================================
    //  DEFINE FN TESTS END
    // ==================================================================
}

// ==================================================================
//
//  TESTS END
//
// ==================================================================
