pub mod builtins;

use std::mem::discriminant;

use elise_aast::{
    AAstNode, AAstNodeCall, AAstNodeData, AAstNodeDataDictKey, AAstNodeDataPrim, AAstNodeTypedef,
    AAstNodeTypedefRecordEntries,
};
use elise_ast::{
    AstNode, AstNodeExpr, AstNodeExprCall, AstNodeExprDict, AstNodeExprList, AstNodeExprPrim,
    AstNodeTypedef, AstNodeTypedefGeneric,
};
use elise_binder::type_binder::TypeBinder;
use elise_bindings::{TypeBinding, TypeBindingsMap};
use elise_shared::{
    shared_errors::{errors_semanalyzer::SemanalyzerErr, errors_type_binder::TypeBinderErr},
    shared_types::{ArityMismatchKind, Span},
};

use crate::builtins::{FnGet, FnTypedef, SlotData, TypedefLexeme};

pub struct Harmony<'a> {
    pub ast: &'a Vec<AstNode>,

    // Local resolved type bindings. Flattened AAst type
    // definition nodes for access simplification.
    // Also returned from Harmony so it can accept it back
    // as globals from a different context. For example,
    // we receive type bindings from schema semantic
    // analysis and then pass it as globals into a source
    // code semantic analysis stage, so we can access
    // type definitions from schema file inside our source
    // code file.
    pub type_bindings: &'a mut TypeBindingsMap,

    // Instruct semanalyzer to limit analysis to
    // type definitions only. Set to `true` for
    // the case of .elt files analysis.
    pub typedef_mode: bool,
}

impl<'a> Harmony<'a> {
    pub fn new(
        ast: &'a Vec<AstNode>,
        global_type_bindings: &'a mut TypeBindingsMap,
        typedef_mode: bool,
    ) -> Self {
        Self {
            ast,
            // Inject as globals if available. Passing global type
            // bindings into semanalyzer is not required.
            type_bindings: global_type_bindings,
            typedef_mode,
        }
    }

    fn verify_node(&self, aast_node: &AAstNode) -> Result<(), SemanalyzerErr> {
        match aast_node {
            AAstNode::Call(AAstNodeCall::Typedef { .. }) => Ok(()),
            node => {
                if self.typedef_mode {
                    return Err(SemanalyzerErr::TypedefMode { span: node.span() });
                }
                Ok(())
            }
        }
    }

    pub fn analyze(&mut self) -> Result<Vec<AAstNode>, SemanalyzerErr> {
        let mut aast: Vec<AAstNode> = vec![];

        for ast_node in self.ast {
            let aast_node = self.analyze_node(ast_node)?;
            self.verify_node(&aast_node)?;
            aast.push(aast_node);
        }

        Ok(aast)
    }

    fn analyze_node(&mut self, ast_node: &AstNode) -> Result<AAstNode, SemanalyzerErr> {
        match ast_node {
            AstNode::Expr(expr) => self.analyze_node_expr(expr),
            _ => Err(SemanalyzerErr::UnsupportedNode {
                span: ast_node.span(),
            }),
        }
    }

    fn analyze_node_expr(&mut self, ast_node: &AstNodeExpr) -> Result<AAstNode, SemanalyzerErr> {
        match ast_node {
            AstNodeExpr::Call(call) => self.analyze_call(call),
            AstNodeExpr::Slot(prim) => self.analyze_slot(prim),
            AstNodeExpr::Int(prim) => self.analyze_prim(prim, AAstNodeData::Int),
            AstNodeExpr::Str(prim) => self.analyze_prim(prim, AAstNodeData::Str),
            AstNodeExpr::List(list) => self.analyze_list(list),
            AstNodeExpr::Dict(dict) => self.analyze_dict(dict),
            _ => Err(SemanalyzerErr::UnsupportedNode {
                span: ast_node.span(),
            }),
        }
    }

    // ==================================================================
    // NODE MATCHERS START
    // ==================================================================

    fn expect_typedef(ast_node: &AstNode) -> Result<&AstNodeTypedef, SemanalyzerErr> {
        let AstNode::Typedef(typedef) = ast_node else {
            return Err(SemanalyzerErr::ExpectedTypedef {
                span: ast_node.span(),
            });
        };
        Ok(typedef)
    }

    fn expect_identifier(ast_node: &AstNode) -> Result<&AstNodeExprPrim, SemanalyzerErr> {
        let AstNode::Expr(AstNodeExpr::Ident(identifier)) = ast_node else {
            return Err(SemanalyzerErr::ExpectedIdentifier {
                span: ast_node.span(),
            });
        };
        Ok(identifier)
    }

    // ==================================================================
    // NODE MATCHERS END
    // ==================================================================

    // ==================================================================
    // TYPE INFERENCE START
    // ==================================================================

    /// Takes AAstNode, filters out everything we cannot use for
    /// type inference (any entity that does not resolve into a data)
    /// and returns back TypeBinding.
    fn infer_type(&self, aast_node: &AAstNode) -> Result<TypeBinding, SemanalyzerErr> {
        match aast_node {
            // NOTE: We support only @data slot for now.
            AAstNode::SlotData { span } => {
                let Some(slot_bindings) = self
                    .type_bindings
                    .get(&SlotData::TYPEDEF_LEXEME.to_string())
                else {
                    return Err(SemanalyzerErr::NoTypeDefinition { span: span.clone() });
                };
                Ok(slot_bindings.clone())
            }
            unsupported_node => Err(SemanalyzerErr::CannotInferType {
                span: unsupported_node.span(),
            }),
        }
    }

    // ==================================================================
    // TYPE INFERENCE END
    // ==================================================================

    // ==================================================================
    // CALL START
    // ==================================================================

    fn analyze_call(&mut self, call: &AstNodeExprCall) -> Result<AAstNode, SemanalyzerErr> {
        match call.lexeme.as_str() {
            FnTypedef::LEXEME => self.analyze_call_typedef(call),
            FnGet::LEXEME => self.analyze_call_get(call),
            _ => Err(SemanalyzerErr::UnknownFunction {
                span: call.span.clone(),
            }),
        }
    }

    // ==================================================================
    // CALL END
    // ==================================================================

    // ==================================================================
    // TYPEDEF CALL START
    // ==================================================================

    fn analyze_typedef_custom(
        &self,
        typedef: &AstNodeTypedef,
    ) -> Result<AAstNodeTypedef, SemanalyzerErr> {
        if typedef.generic.is_some() {
            return Err(SemanalyzerErr::UnexpectedGeneric {
                span: typedef.span.clone(),
            });
        }

        // If we encounter a type definition with an unknown
        // type alias, return an error. It covers the case
        // with referencing itself, because at this point
        // we don't have a record of a type definition
        // being resolved, so resolving alias to itself will
        // fail here.
        if !self.type_bindings.contains_key(&typedef.lexeme) {
            return Err(SemanalyzerErr::UnknownTypedef {
                span: typedef.span.clone(),
            });
        }

        Ok(AAstNodeTypedef::Custom {
            alias: typedef.lexeme.clone(),
            span: typedef.span.clone(),
        })
    }

    fn analyze_typedef_int(typedef: &AstNodeTypedef) -> Result<AAstNodeTypedef, SemanalyzerErr> {
        if typedef.generic.is_some() {
            return Err(SemanalyzerErr::UnexpectedGeneric {
                span: typedef.span.clone(),
            });
        }
        Ok(AAstNodeTypedef::Int {
            span: typedef.span.clone(),
        })
    }

    fn analyze_typedef_list(
        &mut self,
        typedef: &AstNodeTypedef,
    ) -> Result<AAstNodeTypedef, SemanalyzerErr> {
        let Some(generic) = typedef.generic.as_ref() else {
            return Err(SemanalyzerErr::ExpectedGeneric {
                span: typedef.span.clone(),
            });
        };

        let AstNodeTypedefGeneric::Single(single_generic) = generic else {
            return Err(SemanalyzerErr::InvalidGeneric {
                span: typedef.span.clone(),
            });
        };

        Ok(AAstNodeTypedef::List {
            span: typedef.span.clone(),
            item_type: Box::new(self.analyze_typedef(single_generic)?),
        })
    }

    fn analyze_typedef_record(
        &mut self,
        typedef: &AstNodeTypedef,
    ) -> Result<AAstNodeTypedef, SemanalyzerErr> {
        let Some(generic) = typedef.generic.as_ref() else {
            return Err(SemanalyzerErr::ExpectedGeneric {
                span: typedef.span.clone(),
            });
        };

        let AstNodeTypedefGeneric::Record(ast_entries) = generic else {
            return Err(SemanalyzerErr::InvalidGeneric {
                span: typedef.span.clone(),
            });
        };

        let entries = ast_entries
            .iter()
            .map(|(key, typedef)| {
                Ok((key.lexeme.clone(), Box::new(self.analyze_typedef(typedef)?)))
            })
            .collect::<Result<AAstNodeTypedefRecordEntries, SemanalyzerErr>>()?;

        Ok(AAstNodeTypedef::Record {
            span: typedef.span.clone(),
            entries,
        })
    }

    fn analyze_typedef(
        &mut self,
        typedef: &AstNodeTypedef,
    ) -> Result<AAstNodeTypedef, SemanalyzerErr> {
        match typedef.lexeme.as_str() {
            TypedefLexeme::INT => Self::analyze_typedef_int(typedef),
            TypedefLexeme::LIST => self.analyze_typedef_list(typedef),
            TypedefLexeme::RECORD => self.analyze_typedef_record(typedef),
            _ => self.analyze_typedef_custom(typedef),
        }
    }

    fn record_typedef(
        &mut self,
        alias: &str,
        aast_typedef: &AAstNodeTypedef,
    ) -> Result<(), SemanalyzerErr> {
        match TypeBinder::new(aast_typedef, self.type_bindings).bind() {
            Ok(bindings) => self.type_bindings.insert(alias.to_string(), bindings),
            Err(bind_err) => match bind_err {
                TypeBinderErr::UnknownTypedef { span } => {
                    return Err(SemanalyzerErr::UnknownTypedef { span });
                }
                _ => {
                    return Err(SemanalyzerErr::UnresolvableTypedef {
                        span: aast_typedef.span(),
                    });
                }
            },
        };
        Ok(())
    }

    fn analyze_call_typedef(&mut self, call: &AstNodeExprCall) -> Result<AAstNode, SemanalyzerErr> {
        if call.body.len() != FnTypedef::ARGS_LEN {
            return Err(SemanalyzerErr::ArityMismatch {
                fn_name: FnTypedef::LEXEME,
                found: call.body.len(),
                span: call.span.clone(),
                kind: ArityMismatchKind::Eq(FnTypedef::ARGS_LEN),
            });
        }

        let first_arg = &**call.body.first().unwrap();
        let second_arg = &**call.body.last().unwrap();

        let identifier = Self::expect_identifier(first_arg)?;

        // Ensure that the second argument is a type definition
        // and extract it.
        let ast_typedef = Self::expect_typedef(second_arg)?;

        // Analyze the whole type definition.
        let aast_typedef = self.analyze_typedef(ast_typedef)?;

        // Save newly resolved type into a global scope.
        self.record_typedef(&identifier.lexeme, &aast_typedef)?;

        Ok(AAstNode::Call(AAstNodeCall::Typedef {
            alias: identifier.lexeme.clone(),
            typedef: aast_typedef,
        }))
    }

    // ==================================================================
    // TYPEDEF CALL END
    // ==================================================================

    // ==================================================================
    // GET CALL START
    // ==================================================================

    fn analyze_call_get(&mut self, call: &AstNodeExprCall) -> Result<AAstNode, SemanalyzerErr> {
        if call.body.len() < FnGet::ARGS_LEN_MIN {
            return Err(SemanalyzerErr::ArityMismatch {
                fn_name: FnGet::LEXEME,
                found: call.body.len(),
                span: call.span.clone(),
                kind: ArityMismatchKind::MoreEq(FnGet::ARGS_LEN_MIN),
            });
        }

        let first_arg = self.analyze_node(call.body.first().unwrap())?;
        let _data_binding = self.infer_type(&first_arg)?;

        // TODO: Collect the rest of the arguments, validate that
        // there are either number nodes or string nodes, and
        // build BindingPath from them. Then try to resolve a
        // type definition by this path from data_binding.
        // If not found, emit error.

        Err(SemanalyzerErr::UnknownFunction {
            span: Span { start: 0, end: 0 },
        })
    }

    // ==================================================================
    // GET CALL END
    // ==================================================================

    // ==================================================================
    // SLOT START
    // ==================================================================

    fn analyze_slot(&mut self, prim: &AstNodeExprPrim) -> Result<AAstNode, SemanalyzerErr> {
        match prim.lexeme.as_str() {
            SlotData::LEXEME => Ok(AAstNode::SlotData {
                span: prim.span.clone(),
            }),
            _ => Err(SemanalyzerErr::UnknownSlot {
                span: prim.span.clone(),
            }),
        }
    }

    // ==================================================================
    // SLOT END
    // ==================================================================

    // ==================================================================
    // PRIM START
    //
    // Primitive Ast nodes map 1:1 to AAst primitive nodes
    // so we one a single function for them.
    // ==================================================================

    fn analyze_prim(
        &mut self,
        prim: &AstNodeExprPrim,
        variant: fn(AAstNodeDataPrim) -> AAstNodeData,
    ) -> Result<AAstNode, SemanalyzerErr> {
        Ok(AAstNode::Data(variant(AAstNodeDataPrim {
            span: prim.span.clone(),
            lexeme: prim.lexeme.clone(),
        })))
    }

    // ==================================================================
    // PRIM END
    // ==================================================================

    // ==================================================================
    // LIST START
    // ==================================================================

    fn analyze_list(&mut self, list: &AstNodeExprList) -> Result<AAstNode, SemanalyzerErr> {
        // Each list item must be of the same type.
        // This validation guarantees only a shallow type comparison,
        // so technically, this code won't fail: [[1, 2], ["3", "4"]]
        // which is fine, because on the type validation level this code
        // will be rejected since we have no possibility to express
        // list item as a union type.
        let mut item_type: Option<&Box<AstNodeExpr>> = None;
        for item in list.items.iter() {
            if let Some(ty) = item_type
                && discriminant(&**item) != discriminant(ty)
            {
                return Err(SemanalyzerErr::ListInconsistentItemTypes { span: item.span() });
            } else {
                item_type = Some(item);
            }
        }

        // We allow only data nodes inside list items for now in order
        // to avoid complexity related to expression evaluation at runtime.
        // Grammatically, this code can be accepted:
        // [.add(1, 2)] because .add call is an expression that produced a
        // value, but this will require us to emit an additional type of
        // AAst nodes that must instruct the Compiler or VM to perform
        // evaluation of this expression first in order to derive its value
        // into the list. This will be covered in the future.
        let mut aast_items = vec![];
        for item in list.items.iter() {
            let AAstNode::Data(data_node) = self.analyze_node_expr(item)? else {
                return Err(SemanalyzerErr::UnsupportedListItem { span: item.span() });
            };
            aast_items.push(Box::new(data_node));
        }

        Ok(AAstNode::Data(AAstNodeData::List {
            span: list.span.clone(),
            items: aast_items,
        }))
    }

    // ==================================================================
    // LIST END
    // ==================================================================

    // ==================================================================
    // DICT START
    // ==================================================================

    fn analyze_dict(&mut self, dict: &AstNodeExprDict) -> Result<AAstNode, SemanalyzerErr> {
        let mut entries = vec![];

        for entry in dict.entries.iter() {
            let AAstNode::Data(data_node) = self.analyze_node_expr(&entry.1)? else {
                return Err(SemanalyzerErr::UnsupportedDictItem {
                    span: entry.1.span(),
                });
            };
            entries.push((
                AAstNodeDataDictKey {
                    span: entry.0.span.clone(),
                    lexeme: entry.0.lexeme.clone(),
                },
                Box::new(data_node),
            ));
        }

        Ok(AAstNode::Data(AAstNodeData::Dict {
            span: dict.span.clone(),
            entries,
        }))
    }

    // ==================================================================
    // DICT END
    // ==================================================================
}
