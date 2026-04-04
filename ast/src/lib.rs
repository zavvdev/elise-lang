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
 * In order to use in pattern matching where we don't care about
 * enum values and only need to match type.
 */
pub enum AstNodeKind {
    Call,
    Number,
    String,
    Bool,
    Null,
    List,
    Dict,
    DictPair,
    Identifier,
    Any,
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

impl AstNode {
    pub fn kind(&self) -> AstNodeKind {
        match self {
            AstNode::Call(_) => AstNodeKind::Call,
            AstNode::Number(_) => AstNodeKind::Number,
            AstNode::String(_) => AstNodeKind::String,
            AstNode::Bool(_) => AstNodeKind::Bool,
            AstNode::Null(_) => AstNodeKind::Null,
            AstNode::List(_) => AstNodeKind::List,
            AstNode::Dict(_) => AstNodeKind::Dict,
            AstNode::DictPair(_) => AstNodeKind::DictPair,
            AstNode::Identifier(_) => AstNodeKind::Identifier,
        }
    }
}
