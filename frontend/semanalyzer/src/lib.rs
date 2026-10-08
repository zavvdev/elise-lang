pub mod builtins;

use elise_aast::{AAstNode, AAstNodeCall, AAstNodeTypedef, AAstNodeTypedefRecordEntries};
use elise_ast::{
    AstNode, AstNodeExpr, AstNodeExprCall, AstNodeExprPrim, AstNodeTypedef, AstNodeTypedefGeneric,
};
use elise_binder::type_binder::TypeBinder;
use elise_bindings::TypeBindingsMap;
use elise_shared::{
    shared_errors::{errors_semanalyzer::SemanalyzerErr, errors_type_binder::TypeBinderErr},
    shared_types::ArityMismatchKind,
};

use crate::builtins::{FnTypedef, SlotLexeme, TypedefLexeme};

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
            AstNode::Expr(AstNodeExpr::Call(call)) => self.analyze_call(call),
            AstNode::Expr(AstNodeExpr::Slot(prim)) => self.analyze_slot(prim),
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
    // CALL START
    // ==================================================================

    fn analyze_call(&mut self, call: &AstNodeExprCall) -> Result<AAstNode, SemanalyzerErr> {
        match call.lexeme.as_str() {
            FnTypedef::LEXEME => self.analyze_call_typedef(call),
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

    /// Entry point for .typedef function analysis.
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
    // SLOT START
    // ==================================================================

    fn analyze_slot(&mut self, prim: &AstNodeExprPrim) -> Result<AAstNode, SemanalyzerErr> {
        match prim.lexeme.as_str() {
            SlotLexeme::DATA => Ok(AAstNode::SlotData {
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
}
