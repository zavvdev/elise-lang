//! # Harmony — Semantic Analyzer
//!
//! Transforms an AST into a HIR (High-level Intermediate Representation)
//! by walking the AST and performing semantic validation and annotation.
//!
//! ## Input
//!   - AST produced by the parser
//!
//! ## Output
//!   - HIR { SymbolTable, AAST }
//!
//! ## What Harmony does
//!   - Resolves identifiers into SymbolIds and registers them in the SymbolTable
//!   - Validates language rules (arity, type constraints, redefinition etc.)
//!   - Annotates AST nodes with type information derived from schema and literals
//!   - Folds constants where all operands are known at compile time
//!
//! ## What Harmony does NOT do
//!   - Store runtime values in the SymbolTable (type only, value lives in AAST)
//!   - Emit bytecode (that is the emitter's responsibility)
//!   - Interpret values beyond what is necessary for constant folding and
//!     compile-time optimizations (full interpretation is the VM's responsibility)
//!
//! By the time HIR reaches the emitter, all semantic guarantees are established
//! and the emitter can trust the AAST without re-validation.

pub mod builtins;
pub mod scope_stack;

use elise_aast::{
    AAstNode, AAstNodeData,
    data_types::{AAstDataType, AAstPrimDataType},
    symbol_table::SymbolTable,
};
use elise_ast::{AstNode, AstNodeExpr, AstNodeExprCall, AstNodeExprPrim};
use elise_bindings::type_bindings::TypeAliasTable;
use elise_shared::{
    shared_errors::errors_semanalyzer::SemanalyzerErr, shared_node_names::NodeName,
    shared_types::ArityMismatchKind,
};

use crate::{
    builtins::{FnLet, TypedefLexeme},
    scope_stack::ScopeStack,
};

// ==================================================================
//
// SEMANALYZER START
//
// ==================================================================

#[derive(Debug)]
pub struct HIR {
    pub symbol_table: SymbolTable,
    pub aast: Vec<AAstNode>,
}

pub struct Harmony<'a> {
    pub ast: &'a Vec<AstNode>,
    pub scope_stack: ScopeStack,
    pub type_alias_table: Option<TypeAliasTable>,
}

impl<'a> Harmony<'a> {
    pub fn new(ast: &'a Vec<AstNode>, type_alias_table: Option<TypeAliasTable>) -> Self {
        let mut scope_stack = ScopeStack::new();

        // In order to have a global scope we push a new one
        // before analyzing AST, so the first stack frame is
        // our genesis scope. We need to do this because things
        // like .define function does not create its own stack
        // frame, so if it defines an identifier in the global
        // scope, the stack frame must be already there.
        scope_stack.push();
        Self { ast, scope_stack, type_alias_table }
    }

    /// Analyzer entry point. Creates symbol table and aast vector
    /// that are both mutable and passed down to every other function
    /// that needs them.
    pub fn analyze(&mut self) -> Result<HIR, SemanalyzerErr> {
        let mut symbol_table = SymbolTable::new();
        let mut aast: Vec<AAstNode> = vec![];

        for ast_node in self.ast {
            let aast_node = self.annotate_ast_node(ast_node, &mut symbol_table)?;
            aast.push(aast_node);
        }

        Ok(HIR { symbol_table, aast })
    }

    fn annotate_ast_node(
        &mut self,
        ast_node: &AstNode,
        symbol_table: &mut SymbolTable,
    ) -> Result<AAstNode, SemanalyzerErr> {
        match ast_node {
            AstNode::Expr(expr) => match expr {
                AstNodeExpr::Ident(primitive) => self.annotate_identifier_reference(primitive),
                AstNodeExpr::Int(primitive) => Self::annotate_int(primitive),
                AstNodeExpr::Call(call) => self.annotate_call(call, symbol_table),
                _ => Err(SemanalyzerErr::UnsupportedNode {
                    span: ast_node.span().clone(),
                }),
            },
            _ => Err(SemanalyzerErr::UnsupportedNode {
                span: ast_node.span().clone(),
            }),
        }
    }

    fn expect_typedef(ast_node: &AstNode) -> Result<AAstDataType, SemanalyzerErr> {
        let AstNode::Typedef(typedef) = ast_node else {
            return Err(SemanalyzerErr::ExpectedTypedef {
                span: ast_node.span().clone(),
            });
        };

        match typedef.lexeme.as_ref() {
            TypedefLexeme::INT => Ok(AAstDataType::Prim(AAstPrimDataType::Int)),
            _ => Err(SemanalyzerErr::UnknownTypedef {
                span: ast_node.span().clone(),
            }),
        }
    }

    fn expect_expr(ast_node: &AstNode) -> Result<&AstNodeExpr, SemanalyzerErr> {
        let AstNode::Expr(expr) = ast_node else {
            return Err(SemanalyzerErr::ExpectedExpr {
                span: ast_node.span().clone(),
            });
        };
        Ok(expr)
    }

    fn expect_identifier(ast_node: &AstNode) -> Result<&AstNodeExprPrim, SemanalyzerErr> {
        let AstNode::Expr(AstNodeExpr::Ident(identifier)) = ast_node else {
            return Err(SemanalyzerErr::ExpectedIdentifier {
                span: ast_node.span().clone(),
            });
        };
        Ok(identifier)
    }

    // ==================================================================
    // PRIMITIVE ANNOTATIONS START
    //
    // Annotations for primitive values such as Number, String, Bool,
    // Null, Identifier which we can map almost 1:1 from AstNode
    // to AAstNode.
    // ==================================================================

    // ==================================================================
    // ANNOTATE IDENTIFIER REFERENCE START
    //
    // Annotates identifier references only.
    // It means that it captures only identifiers that are
    // already in scope and just referenced. For example:
    //
    // .let (PI 3.1415)
    // .add (PI 42)
    //
    // This function takes care of `PI` in .add
    // function call only.
    // ==================================================================

    fn annotate_identifier_reference(
        &self,
        primitive: &AstNodeExprPrim,
    ) -> Result<AAstNode, SemanalyzerErr> {
        self.scope_stack
            .resolve(&primitive.lexeme)
            .map(|(symbol_id, depth)| AAstNode::SymbolRef {
                symbol_id,
                depth,
                span: primitive.span.clone(),
            })
            .ok_or_else(|| SemanalyzerErr::SymbolUndefined {
                span: primitive.span.clone(),
            })
    }

    // ==================================================================
    // ANNOTATE IDENTIFIER REFERENCE END
    // ==================================================================

    // ==================================================================
    // ANNOTATE INT START
    // ==================================================================

    fn annotate_int(primitive: &AstNodeExprPrim) -> Result<AAstNode, SemanalyzerErr> {
        Ok(AAstNode::Data(AAstNodeData::Int {
            value: primitive.lexeme.clone(),
            span: primitive.span.clone(),
        }))
    }

    // ==================================================================
    // ANNOTATE INT END
    // ==================================================================

    // ==================================================================
    // ANNOTATE CALL START
    // ==================================================================

    fn annotate_call(
        &mut self,
        call: &AstNodeExprCall,
        symbol_table: &mut SymbolTable,
    ) -> Result<AAstNode, SemanalyzerErr> {
        match call.lexeme.as_str() {
            FnLet::LEXEME => self.annotate_let_call(call, symbol_table),
            _ => Err(SemanalyzerErr::UnknownFunction {
                span: call.span.clone(),
            }),
        }
    }

    // ==================================================================
    // ANNOTATE CALL END
    // ==================================================================

    // ==================================================================
    // ANNOTATE LET CALL START
    //
    // .let (Identifier Typedef DataType)
    //
    // 1. Has only 3 arguments;
    // 2. First argument is always an identifier;
    // 3. Second argument is always a type definition;
    // 4. Third argument is always a data;
    // 5. Never creates a new scope stack record;
    // 6. Defines symbols in the current scope stack;
    // 7. Does not remove any scope stack entries;
    // ==================================================================

    fn annotate_let_call(
        &mut self,
        call: &AstNodeExprCall,
        symbol_table: &mut SymbolTable,
    ) -> Result<AAstNode, SemanalyzerErr> {
        if call.body.len() != FnLet::ARGS_LEN {
            return Err(SemanalyzerErr::ArityMismatch {
                fn_name: FnLet::LEXEME,
                found: call.body.len(),
                span: call.span.clone(),
                kind: ArityMismatchKind::Eq(FnLet::ARGS_LEN),
            });
        }

        let first_arg = &**call.body.get(0).unwrap();
        let second_arg = &**call.body.get(1).unwrap();
        let third_arg = &**call.body.get(2).unwrap();

        let identifier = Self::expect_identifier(first_arg)?;
        let typedef = Self::expect_typedef(second_arg)?;
        let expr = Self::expect_expr(third_arg)?;

        let (ident_type, aast_node) = match third_arg {
            AstNode::Expr(AstNodeExpr::Int(prim)) => (
                AAstDataType::Prim(AAstPrimDataType::Int),
                Self::annotate_int(prim)?,
            ),
            _ => {
                return Err(SemanalyzerErr::ArgTypeMismatch {
                    fn_name: FnLet::LEXEME,
                    position: 2,
                    expected: NodeName::EXPRESSION,
                    found: second_arg.as_str(),
                    span: second_arg.span().clone(),
                });
            }
        };

        if self.scope_stack.resolve(&primitive.value).is_some() {
            return Err(SemanalyzerErr::SymbolDuplicate {
                span: call.span.clone(),
            });
        }

        let symbol_id = symbol_table.fresh(&primitive.value, LangType::Primitive(ident_type));

        self.scope_stack.define(&primitive.value, symbol_id);

        Ok(AAstNode::CallDefine {
            symbol_id,
            value: Box::new(aast_node),
            span: call.span.clone(),
        })
    }

    // ==================================================================
    // ANNOTATE LET CALL END
    // ==================================================================
}

// ==================================================================
//
// SEMANALYZER END
//
// ==================================================================
