pub mod builtins;

use std::collections::HashMap;

use elise_aast::{AAstNode, AAstNodeCall, AAstNodeTypedef, AAstNodeTypedefRecordEntries};
use elise_ast::{
    AstNode, AstNodeExpr, AstNodeExprCall, AstNodeExprPrim, AstNodeTypedef, AstNodeTypedefGeneric,
};
use elise_bindings::TypeBindings;
use elise_shared::{
    shared_errors::errors_semanalyzer::SemanalyzerErr, shared_types::ArityMismatchKind,
};

use crate::builtins::{FnTypedef, TypedefLexeme};

#[derive(Debug)]
pub struct SemanticModel {
    pub aast: Vec<AAstNode>,

    // Collection of all custom type bindings
    // resolved during analysis.
    pub type_bindings: HashMap<String, TypeBindings>,
}

pub struct Harmony<'a> {
    pub ast: &'a Vec<AstNode>,

    // Local resolved type aliases. These are not bindings.
    // We use them for creating bindings by passing to
    // TypeBinder. Every time we resolve a new AAst typedef
    // node (when user calls .typedef function), we insert
    // this new AAst node here, so we can then access it
    // for creating type binding.
    pub type_aliases: HashMap<String, AAstNodeTypedef>,

    // Local resolved type bindings. Flattened AAst type
    // definition nodes for access simplification.
    // Returned back from Harmony so it can accept it back
    // as globals from a different context. For example,
    // we receive type bindings from schema semantic
    // analysis and then pass it as globals into source
    // code semantic analysis stage, so we can access
    // type definitions from schema file inside our source
    // code file.
    pub type_bindings: HashMap<String, TypeBindings>,
}

impl<'a> Harmony<'a> {
    pub fn new(
        ast: &'a Vec<AstNode>,
        global_type_bindings: Option<HashMap<String, TypeBindings>>,
    ) -> Self {
        Self {
            ast,
            type_aliases: HashMap::new(),
            // Inject as globals if available. Passing global type
            // bindings into semanalyzer is not required.
            type_bindings: global_type_bindings.unwrap_or_default(),
        }
    }

    pub fn analyze(&mut self) -> Result<SemanticModel, SemanalyzerErr> {
        let mut aast: Vec<AAstNode> = vec![];

        for ast_node in self.ast {
            let aast_node = self.analyze_node(ast_node)?;
            aast.push(aast_node);
        }

        Ok(SemanticModel {
            aast,
            // TODO: Maybe we can do smth without cloning?
            type_bindings: self.type_bindings.clone(),
        })
    }

    fn analyze_node(&mut self, ast_node: &AstNode) -> Result<AAstNode, SemanalyzerErr> {
        match ast_node {
            AstNode::Expr(AstNodeExpr::Call(call)) => self.analyze_call(call),
            _ => Err(SemanalyzerErr::UnsupportedNode {
                span: ast_node.span().clone(),
            }),
        }
    }

    // ==================================================================
    // NODE MATCHERS START
    // ==================================================================

    fn expect_typedef(ast_node: &AstNode) -> Result<&AstNodeTypedef, SemanalyzerErr> {
        let AstNode::Typedef(typedef) = ast_node else {
            return Err(SemanalyzerErr::ExpectedTypedef {
                span: ast_node.span().clone(),
            });
        };
        Ok(typedef)
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
    // NODE MATCHERS END
    // ==================================================================

    // ==================================================================
    // TYPEDEF START
    // ==================================================================

    fn analyze_typedef_custom(typedef: &AstNodeTypedef) -> Result<AAstNodeTypedef, SemanalyzerErr> {
        if typedef.generic.is_some() {
            return Err(SemanalyzerErr::UnexpectedGeneric {
                span: typedef.span.clone(),
            });
        }
        // TODO: Check if exists in type_aliases.
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
            _ => Self::analyze_typedef_custom(typedef),
        }
    }

    // ==================================================================
    // TYPEDEF END
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
        let second_arg = &**call.body.get(1).unwrap();

        let identifier = Self::expect_identifier(first_arg)?;
        let ast_typedef = Self::expect_typedef(second_arg)?;
        let aast_typedef = self.analyze_typedef(ast_typedef)?;

        // TODO: Insert typedef into aliases, bind types and insert
        // into type_bindings. But we need to do it ONLY for custom
        // type definitions.

        //let bindings = TypeBinder::new(&aast_typedef, &self.local_type_bindings)
        //    .bind()
        //    .unwrap();

        //println!("bindings: {:#?}", bindings);

        //self.local_type_bindings
        //    .insert(lexeme.to_string(), bindings);

        //self.local_type_aliases
        //    .insert(lexeme.to_string(), aast_typedef.clone());

        Ok(AAstNode::Call(AAstNodeCall::Typedef {
            alias: identifier.lexeme.clone(),
            typedef: aast_typedef,
        }))
    }

    // ==================================================================
    // CALL END
    // ==================================================================
}
