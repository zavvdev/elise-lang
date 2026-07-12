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
use elise_errors::errors_semanalyzer::SemanalyzerErr;

use crate::{
    aast::{AAstNode, AAstPrimitive},
    scope_stack::ScopeStack,
    symbol_table::SymbolTable,
};

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

    pub fn analyze(&self) -> Result<HIR, SemanalyzerErr> {
        let mut symbol_table = SymbolTable::new();
        let mut aast: Vec<AAstNode> = vec![];

        for ast_node in self.ast {
            let aast_node = self.get_aast_node(ast_node, &mut symbol_table)?;
            aast.push(aast_node);
        }

        Ok(HIR { symbol_table, aast })
    }

    fn get_aast_node(
        &self,
        ast_node: &AstNode,
        symbol_table: &mut SymbolTable,
    ) -> Result<AAstNode, SemanalyzerErr> {
        match ast_node {
            AstNode::Number(primitive) => Self::annotate_number(primitive),
            AstNode::Identifier(primitive) => self.annotate_identifier_reference(primitive),
            AstNode::Call((call_kind, compound)) => {
                Self::annotate_call(call_kind, compound, symbol_table)
            }
            _ => Err(SemanalyzerErr::Unknown),
        }
    }

    fn annotate_call(
        _call_kind: &AstCallKind,
        compound: &AstCompound,
        _symbol_table: &mut SymbolTable,
    ) -> Result<AAstNode, SemanalyzerErr> {
        Err(SemanalyzerErr::SymbolUndefined {
            span: compound.span.clone(),
        })
    }

    /// This function annotates identifier references only.
    /// It means that it captures only identifiers that are
    /// already in scope and just referenced. For example:
    ///
    /// .define (PI 3.1415)
    /// .let ([distance 43]
    ///    .add (PI distance))
    ///
    /// This function takes care of `PI` and `distance` in .add
    /// function call only. Resolution for identifier definition
    /// has to be done in respective functions for handling
    /// semantics for expressions that can define identifiers
    /// line `.let` and `.define`.
    fn annotate_identifier_reference(
        &self,
        primitive: &AstPrimitive,
    ) -> Result<AAstNode, SemanalyzerErr> {
        if let Some((symbol_id, depth)) = self.scope_stack.resolve(&primitive.value) {
            return Ok(AAstNode::SymbolRef {
                symbol_id,
                depth,
                span: primitive.span.clone(),
            });
        }

        Err(SemanalyzerErr::SymbolUndefined {
            span: primitive.span.clone(),
        })
    }

    fn annotate_number(primitive: &AstPrimitive) -> Result<AAstNode, SemanalyzerErr> {
        let aast_prim = AAstPrimitive {
            value: primitive.value.clone(),
            span: primitive.span.clone(),
        };
        if primitive.value.contains(".") {
            return Ok(AAstNode::Float(aast_prim));
        }
        Ok(AAstNode::Int(aast_prim))
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

    use std::collections::HashMap;

    use elise_ast::{AstNode, AstPrimitive};
    use elise_binder::DataBindingTable;
    use elise_types::Span;

    use crate::{
        Harmony,
        aast::{AAstNode, AAstPrimitive},
    };

    #[test]
    fn number_should_annotate_integers() {
        let ast = vec![
            AstNode::Number(AstPrimitive {
                value: "32".to_string(),
                span: Span { start: 1, end: 1 },
            }),
            AstNode::Number(AstPrimitive {
                value: "32e-2".to_string(),
                span: Span { start: 2, end: 2 },
            }),
        ];

        let bindings = DataBindingTable {
            table: HashMap::new(),
        };

        let aast = Harmony::new(&ast, &bindings).analyze();

        assert_eq!(
            aast.unwrap().aast,
            vec![
                AAstNode::Int(AAstPrimitive {
                    value: "32".to_string(),
                    span: Span { start: 1, end: 1 }
                }),
                AAstNode::Int(AAstPrimitive {
                    value: "32e-2".to_string(),
                    span: Span { start: 2, end: 2 }
                })
            ]
        );
    }

    #[test]
    fn number_should_annotate_floats() {
        let ast = vec![
            AstNode::Number(AstPrimitive {
                value: "3.2".to_string(),
                span: Span { start: 1, end: 1 },
            }),
            AstNode::Number(AstPrimitive {
                value: "3.2E-2".to_string(),
                span: Span { start: 2, end: 2 },
            }),
        ];

        let bindings = DataBindingTable {
            table: HashMap::new(),
        };

        let aast = Harmony::new(&ast, &bindings).analyze();

        assert_eq!(
            aast.unwrap().aast,
            vec![
                AAstNode::Float(AAstPrimitive {
                    value: "3.2".to_string(),
                    span: Span { start: 1, end: 1 }
                }),
                AAstNode::Float(AAstPrimitive {
                    value: "3.2E-2".to_string(),
                    span: Span { start: 2, end: 2 }
                })
            ]
        )
    }

    // ==================================================================
    //  NUMBER TESTS END
    // ==================================================================

    // ==================================================================
    //  IDENTIFIER REFERENCE TESTS START
    // ==================================================================

    #[test]
    fn identifier_reference_should_annotate_correctly() {}

    // ==================================================================
    //  IDENTIFIER REFERENCE TESTS END
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
