// .const (PI 3.1415)
//
// .let ([x 12, y 38]
//    .mul (PI, .add(x y)))

pub mod symbol_table;

use elise_ast::AstNode;
use elise_binder::DataBindingTable;
use elise_errors::errors_semantic_analyzer::SemanticAnalyzerErr;

use crate::symbol_table::SymbolTable;

// TODO: Update
#[derive(Debug)]
pub enum AAstNode {
    FConst,
    FVar,
    FAdd,
    FMul,
}

#[derive(Debug)]
pub struct HIR {
    pub symbol_table: SymbolTable,
    pub aast: Vec<AAstNode>,
}

pub struct Harmony<'a> {
    pub ast: &'a Vec<AstNode>,
    pub data_binding_table: &'a DataBindingTable,
    // TODO: Add ScopeStack
}

impl<'a> Harmony<'a> {
    pub fn new(ast: &'a Vec<AstNode>, data_binding_table: &'a DataBindingTable) -> Self {
        Self {
            ast,
            data_binding_table,
        }
    }

    pub fn analyze(&self) -> Result<HIR, SemanticAnalyzerErr> {
        Ok(HIR {
            symbol_table: SymbolTable::new(),
            aast: vec![],
        })
    }
}
