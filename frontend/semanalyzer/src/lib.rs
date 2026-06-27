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

// .define (PI 3.1415)
//
// .let ([x 12, y 38]
//    .mul (PI, .add(x y)))

pub mod aast;
pub mod data_types;
pub mod scope_stack;
pub mod symbol_table;

use elise_ast::AstNode;
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
        Self {
            ast,
            data_binding_table,
            scope_stack: ScopeStack::new(),
        }
    }

    pub fn analyze(&self) -> Result<HIR, SemanticAnalyzerErr> {
        Ok(HIR {
            symbol_table: SymbolTable::new(),
            aast: vec![],
        })
    }
}
