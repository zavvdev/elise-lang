use elise_ast::{AstNode, AstNodeExpr, AstNodeExprPrim, AstNodeTypeAnnot};
use elise_shared_errors::errors_semanalyzer::SemanalyzerErr;

pub fn expect_type_annot_ast_node(ast_node: &AstNode) -> Result<&AstNodeTypeAnnot, SemanalyzerErr> {
    let AstNode::TypeAnnot(annot) = ast_node else {
        return Err(SemanalyzerErr::ExpectedTypeAnnot {
            span: ast_node.span(),
        });
    };
    Ok(annot)
}

pub fn expect_identifier_ast_node(ast_node: &AstNode) -> Result<&AstNodeExprPrim, SemanalyzerErr> {
    let AstNode::Expr(AstNodeExpr::Ident(identifier)) = ast_node else {
        return Err(SemanalyzerErr::ExpectedIdentifier {
            span: ast_node.span(),
        });
    };
    Ok(identifier)
}
