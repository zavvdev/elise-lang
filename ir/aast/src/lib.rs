use elise_ast::AstNodeTypedef;

#[derive(Debug, PartialEq)]
pub enum AAstNodeCall {
    Typedef {
        alias: String,
        // TODO: should we keep ast node or create aast native descriptor?
        ast_node: AstNodeTypedef,
    },
}

#[derive(Debug, PartialEq)]
pub enum AAstNode {
    Call(AAstNodeCall),
}
