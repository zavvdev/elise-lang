/**
 * Defines where specific token starts and ends.
 */
#[derive(Debug, PartialEq)]
pub struct TokSpan {
    pub start: usize,
    pub end: usize,
}

/**
 * Primitive values cannot have children.
 */
#[derive(Debug, PartialEq)]
pub struct Primitive {
    pub value: String,
    pub span: TokSpan,
}

/**
 * Any other value that needs to have nested values.
 */
#[derive(Debug, PartialEq)]
pub struct Compound {
    pub span: TokSpan,
    pub children: Vec<Box<AstNode>>,
}

/**
 * We treat DictPair as an AstNode in order to be consistent
 * and always provide ast nodes as children for compound values.
 */
#[derive(Debug, PartialEq)]
pub enum AstNode {
    Call((String, Compound)),
    Number(Primitive),
    String(Primitive),
    Bool(Primitive),
    Null(Primitive),
    List(Compound),
    Dict(Compound),
    DictPair((String, Box<AstNode>)),
    Identifier(Primitive),
}
