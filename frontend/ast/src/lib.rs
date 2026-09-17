mod ast_node_expr;
mod ast_node_typedef;

use elise_shared::shared_types::Span;

use crate::{ast_node_expr::AstNodeExpr, ast_node_typedef::AstNodeTypedef};

#[derive(Debug, PartialEq)]
pub enum AstNode {
    Expr(AstNodeExpr),
    Typedef(AstNodeTypedef),
}

impl AstNode {
    pub fn span(&self) -> &Span {
        match self {
            AstNode::Expr(expr) => expr.span(),
            AstNode::Typedef(typedef) => typedef.span(),
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            AstNode::Expr(expr) => expr.as_str(),
            AstNode::Typedef(typedef) => typedef.as_str(),
        }
    }
}
