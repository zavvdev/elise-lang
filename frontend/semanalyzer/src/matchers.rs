use elise_ir_ast::ast_type_annot::AstNodeTypeAnnot;
use elise_ir_ast::{AstNode, AstNodeExpr, AstNodeExprPrim};
use elise_shared_errors::errors_semanalyzer::SemanalyzerErr;
use elise_shared_types::Name;

pub fn expect_type_annot_ast_node(ast_node: &AstNode) -> Result<&AstNodeTypeAnnot, SemanalyzerErr> {
    let AstNode::TypeAnnot(annot) = ast_node else {
        return Err(SemanalyzerErr::UnexpNode {
            span: ast_node.span(),
            expected: vec![Name::NODE__TYPE_ANNOT.to_string()],
            found: ast_node.name(),
        });
    };
    Ok(annot)
}

pub fn expect_identifier_ast_node(ast_node: &AstNode) -> Result<&AstNodeExprPrim, SemanalyzerErr> {
    let AstNode::Expr(AstNodeExpr::Ident(identifier)) = ast_node else {
        return Err(SemanalyzerErr::UnexpNode {
            span: ast_node.span(),
            found: ast_node.name(),
            expected: vec![Name::EXPR__IDENTIFIER.to_string()],
        });
    };
    Ok(identifier)
}
