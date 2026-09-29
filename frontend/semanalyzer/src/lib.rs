pub mod builtins;

use elise_aast::{AAstNode, AAstNodeCall};
use elise_ast::{AstNode, AstNodeExpr, AstNodeExprCall, AstNodeExprPrim, AstNodeTypedef};
use elise_shared::{
    shared_errors::errors_semanalyzer::SemanalyzerErr, shared_types::ArityMismatchKind,
};

use crate::builtins::FnTypedef;

#[derive(Debug)]
pub struct SemanticModel {
    pub aast: Vec<AAstNode>,
}

pub struct Harmony<'a> {
    pub ast: &'a Vec<AstNode>,
}

impl<'a> Harmony<'a> {
    pub fn new(ast: &'a Vec<AstNode>) -> Self {
        Self { ast }
    }

    pub fn analyze(&mut self) -> Result<SemanticModel, SemanalyzerErr> {
        let mut aast: Vec<AAstNode> = vec![];

        for ast_node in self.ast {
            let aast_node = self.analyze_node(ast_node)?;
            aast.push(aast_node);
        }

        Ok(SemanticModel { aast })
    }

    fn analyze_node(&mut self, ast_node: &AstNode) -> Result<AAstNode, SemanalyzerErr> {
        match ast_node {
            AstNode::Expr(AstNodeExpr::Call(call)) => self.analyze_call(call),
            _ => Err(SemanalyzerErr::UnsupportedNode {
                span: ast_node.span().clone(),
            }),
        }
    }

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
        let typedef = Self::expect_typedef(second_arg)?;

        // TODO: analyze known type definitions and other type alias references.

        Ok(AAstNode::Call(AAstNodeCall::Typedef {
            alias: identifier.lexeme.clone(),
            ast_node: typedef.clone(),
        }))
    }
}
