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

//pub mod semanalyzer_aast;
//pub mod semanalyzer_config;
//pub mod semanalyzer_data_types;
//pub mod semanalyzer_scope_stack;
//pub mod semanalyzer_symbol_table;
//
//use elise_ast::{AstCall, AstNode, AstPrimitive};
//use elise_data::data_binder::DataBindingTable;
//use elise_shared::{
//    shared_errors::errors_semanalyzer::{ArityMismatchKind, SemanalyzerErr},
//    shared_node_names::NodeName,
//    shared_types::{Keyword, Span},
//};
//
//use crate::{
//    semanalyzer_aast::AAstNode,
//    semanalyzer_config::{FnDefine, FnLet},
//    semanalyzer_data_types::{LangPrimitiveType, LangType},
//    semanalyzer_scope_stack::ScopeStack,
//    semanalyzer_symbol_table::SymbolTable,
//};
//
//// ==================================================================
////
////  SEMANALYZER START
////
//// ==================================================================
//
//#[derive(Debug)]
//pub struct HIR {
//    pub symbol_table: SymbolTable,
//    pub aast: Vec<AAstNode>,
//}
//
//pub struct Harmony<'a> {
//    pub ast: &'a Vec<AstNode>,
//    pub data_binding_table: &'a DataBindingTable,
//    pub scope_stack: ScopeStack,
//}
//
//impl<'a> Harmony<'a> {
//    pub fn new(ast: &'a Vec<AstNode>, data_binding_table: &'a DataBindingTable) -> Self {
//        // In order to have a global scope we push a new one
//        // before analyzing AST, so the first stack frame is
//        // our genesis scope.
//        let mut scope_stack = ScopeStack::new();
//        scope_stack.push();
//        Self {
//            ast,
//            data_binding_table,
//            scope_stack,
//        }
//    }
//
//    pub fn analyze(&mut self) -> Result<HIR, SemanalyzerErr> {
//        let mut symbol_table = SymbolTable::new();
//        let mut aast: Vec<AAstNode> = vec![];
//
//        for ast_node in self.ast {
//            let aast_node = self.annotate_ast_node(ast_node, &mut symbol_table)?;
//            aast.push(aast_node);
//        }
//
//        Ok(HIR { symbol_table, aast })
//    }
//
//    fn annotate_ast_node(
//        &mut self,
//        ast_node: &AstNode,
//        symbol_table: &mut SymbolTable,
//    ) -> Result<AAstNode, SemanalyzerErr> {
//        match ast_node {
//            AstNode::Int(primitive) => Self::annotate_int(primitive),
//            AstNode::Float(primitive) => Self::annotate_float(primitive),
//            AstNode::String(primitive) => Self::annotate_string(primitive),
//            AstNode::Bool(primitive) => Self::annotate_bool(primitive),
//            AstNode::Null(primitive) => Self::annotate_null(primitive),
//            AstNode::Identifier(primitive) => self.annotate_identifier_reference(primitive),
//            AstNode::Call(call) => self.annotate_call(call, symbol_table),
//            _ => Err(SemanalyzerErr::UnsupportedNode {
//                span: ast_node.span().clone(),
//            }),
//        }
//    }
//
//    // ==================================================================
//    // ANNOTATE DEFINE CALL START
//    //
//    // .define (Identifier LangPrimitiveType)
//    //
//    // 1. Has only 2 arguments;
//    // 2. First argument is always an identifier;
//    // 3. Second argument is always primitive type;
//    // 4. Never creates a new scope stack record;
//    // 5. Defines symbols in the current scope stack;
//    // 6. Does not remove any scope stack entries;
//    // ==================================================================
//
//    fn annotate_define_call(
//        &mut self,
//        call: &AstCall,
//        symbol_table: &mut SymbolTable,
//    ) -> Result<AAstNode, SemanalyzerErr> {
//        if call.children.len() != FnDefine::ARGS_LEN {
//            return Err(SemanalyzerErr::ArityMismatch {
//                fn_name: FnDefine::LEXEME,
//                expected: FnDefine::ARGS_LEN,
//                found: call.children.len(),
//                span: call.span.clone(),
//                kind: ArityMismatchKind::Eq,
//            });
//        }
//
//        let first_arg = &**call.children.first().unwrap();
//        let second_arg = &**call.children.last().unwrap();
//
//        let (ident_type, aast_node) = match second_arg {
//            AstNode::Int(prim) => (LangPrimitiveType::Int, Self::annotate_int(prim)?),
//            AstNode::Float(prim) => (LangPrimitiveType::Float, Self::annotate_float(prim)?),
//            AstNode::String(prim) => (LangPrimitiveType::String, Self::annotate_string(prim)?),
//            AstNode::Bool(prim) => (LangPrimitiveType::Bool, Self::annotate_bool(prim)?),
//            AstNode::Null(prim) => (LangPrimitiveType::Null, Self::annotate_null(prim)?),
//            _ => {
//                return Err(SemanalyzerErr::ArgTypeMismatch {
//                    fn_name: FnDefine::LEXEME,
//                    position: 1,
//                    expected: NodeName::PRIMITIVE,
//                    found: second_arg.as_str(),
//                    span: second_arg.span().clone(),
//                });
//            }
//        };
//
//        let AstNode::Identifier(primitive) = first_arg else {
//            return Err(SemanalyzerErr::ArgKindMismatch {
//                fn_name: FnDefine::LEXEME,
//                position: 0,
//                expected: NodeName::IDENTIFIER,
//                found: first_arg.as_str(),
//                span: first_arg.span().clone(),
//            });
//        };
//
//        if self.scope_stack.resolve(&primitive.value).is_some() {
//            return Err(SemanalyzerErr::SymbolDuplicate {
//                span: call.span.clone(),
//            });
//        }
//
//        let symbol_id =
//            symbol_table.fresh(primitive.value.clone(), LangType::Primitive(ident_type));
//
//        self.scope_stack.define(primitive.value.clone(), symbol_id);
//
//        Ok(AAstNode::CallDefine {
//            symbol_id,
//            value: Box::new(aast_node),
//            span: call.span.clone(),
//        })
//    }
//
//    // ==================================================================
//    // ANNOTATE DEFINE CALL END
//    // ==================================================================
//
//    // ==================================================================
//    // ANNOTATE LET CALL START
//    //
//    // .let ([(Identifier Expression)+] Expression+)
//    //
//    // 1. Min 2 arguments;
//    // 2. First argument is always a list;
//    // 3. Odd items in the list are always identifiers;
//    // 4. Even items in the list are always expressions
//    //    that must be evaluated first;
//    // 5. The result of evaluation is always a result of
//    //    the last evaluated expression;
//    // 6. Creates its own scope stack when enters;
//    // 7. Removes its own scope stack when evaluation finishes;
//    // 8. Does not allow symbol re-bindings;
//    // 9. Can access outer scope;
//    // ==================================================================
//
//    fn annotate_let_call(
//        &mut self,
//        call: &AstCall,
//        _symbol_table: &mut SymbolTable,
//    ) -> Result<AAstNode, SemanalyzerErr> {
//        if call.children.len() < FnLet::MIN_ARGS_LEN {
//            return Err(SemanalyzerErr::ArityMismatch {
//                fn_name: FnLet::LEXEME,
//                expected: FnLet::MIN_ARGS_LEN,
//                found: call.children.len(),
//                span: call.span.clone(),
//                kind: ArityMismatchKind::MoreEq,
//            });
//        }
//
//        // TODO
//
//        Err(SemanalyzerErr::UnknownFunction {
//            span: Span { start: 0, end: 0 },
//        })
//    }
//
//    // ==================================================================
//    // ANNOTATE LET CALL END
//    // ==================================================================
//
//    // ==================================================================
//    // ANNOTATE CALL START
//    // ==================================================================
//
//    fn annotate_call(
//        &mut self,
//        call: &AstCall,
//        symbol_table: &mut SymbolTable,
//    ) -> Result<AAstNode, SemanalyzerErr> {
//        match call.lexeme.as_str() {
//            FnDefine::LEXEME => self.annotate_define_call(call, symbol_table),
//            FnLet::LEXEME => self.annotate_let_call(call, symbol_table),
//            _ => Err(SemanalyzerErr::UnknownFunction {
//                span: call.span.clone(),
//            }),
//        }
//    }
//
//    // ==================================================================
//    // ANNOTATE CALL END
//    // ==================================================================
//
//    // ==================================================================
//    // PRIMITIVE ANNOTATIONS START
//    //
//    // Annotations for primitive values Number, String, Bool, Null,
//    // Identifier which we can map almost 1:1 from AstNode to AAstNode.
//    // ==================================================================
//
//    // ==================================================================
//    // ANNOTATE IDENTIFIER REFERENCE START
//    //
//    // Annotates identifier references only.
//    // It means that it captures only identifiers that are
//    // already in scope and just referenced. For example:
//    //
//    // .define (PI 3.1415)
//    // .let ([distance 43]
//    //    .add (PI distance))
//    //
//    // This function takes care of `PI` and `distance` in .add
//    // function call only. Resolution for identifier definition
//    // has to be done in respective functions for handling
//    // semantics for expressions that can define identifiers
//    // like `.let` and `.define`.
//    // ==================================================================
//
//    fn annotate_identifier_reference(
//        &self,
//        primitive: &AstPrimitive,
//    ) -> Result<AAstNode, SemanalyzerErr> {
//        self.scope_stack
//            .resolve(&primitive.value)
//            .map(|(symbol_id, depth)| AAstNode::SymbolRef {
//                symbol_id,
//                depth,
//                span: primitive.span.clone(),
//            })
//            .ok_or_else(|| SemanalyzerErr::SymbolUndefined {
//                span: primitive.span.clone(),
//            })
//    }
//
//    // ==================================================================
//    // ANNOTATE IDENTIFIER REFERENCE END
//    // ==================================================================
//
//    // ==================================================================
//    // ANNOTATE INT START
//    // ==================================================================
//
//    fn annotate_int(primitive: &AstPrimitive) -> Result<AAstNode, SemanalyzerErr> {
//        Ok(AAstNode::Int {
//            value: primitive.value.clone(),
//            span: primitive.span.clone(),
//        })
//    }
//
//    // ==================================================================
//    // ANNOTATE INT END
//    // ==================================================================
//
//    // ==================================================================
//    // ANNOTATE FLOAT START
//    // ==================================================================
//
//    fn annotate_float(primitive: &AstPrimitive) -> Result<AAstNode, SemanalyzerErr> {
//        Ok(AAstNode::Float {
//            value: primitive.value.clone(),
//            span: primitive.span.clone(),
//        })
//    }
//
//    // ==================================================================
//    // ANNOTATE FLOAT END
//    // ==================================================================
//
//    // ==================================================================
//    // ANNOTATE STRING START
//    // ==================================================================
//
//    fn annotate_string(primitive: &AstPrimitive) -> Result<AAstNode, SemanalyzerErr> {
//        Ok(AAstNode::String {
//            value: primitive.value.clone(),
//            span: primitive.span.clone(),
//        })
//    }
//
//    // ==================================================================
//    // ANNOTATE STRING END
//    // ==================================================================
//
//    // ==================================================================
//    // ANNOTATE BOOL START
//    // ==================================================================
//
//    fn annotate_bool(primitive: &AstPrimitive) -> Result<AAstNode, SemanalyzerErr> {
//        Ok(AAstNode::Bool {
//            value: primitive.value == Keyword::TRUE,
//            span: primitive.span.clone(),
//        })
//    }
//
//    // ==================================================================
//    // ANNOTATE BOOL END
//    // ==================================================================
//
//    // ==================================================================
//    // ANNOTATE NULL START
//    // ==================================================================
//
//    fn annotate_null(primitive: &AstPrimitive) -> Result<AAstNode, SemanalyzerErr> {
//        Ok(AAstNode::Null {
//            span: primitive.span.clone(),
//        })
//    }
//
//    // ==================================================================
//    // ANNOTATE BOOL END
//    // ==================================================================
//
//    // ==================================================================
//    // PRIMITIVE ANNOTATIONS END
//    // ==================================================================
//}
//
//// ==================================================================
////
////  SEMANALYZER END
////
//// ==================================================================
