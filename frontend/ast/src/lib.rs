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

#[derive(Debug, PartialEq)]
pub enum CallKind {
    Named(String),
    Anon,
}

#[derive(Debug, PartialEq)]
pub struct KeyValuePair {
    pub key: String,
    // Span for key itself since we don't want
    // to keep the whole ast node as key.
    pub key_span: TokSpan,
    // Value has its own span since it's AstNode.
    pub value: Box<AstNode>,
    // Span from the start of the key and
    // to the end of the value.
    pub span: TokSpan,
}

/**
 * We treat DictPair as an AstNode in order to be consistent
 * and always provide ast nodes as children for compound values.
 */
#[derive(Debug, PartialEq)]
pub enum AstNode {
    Call((CallKind, Compound)),
    Number(Primitive),
    String(Primitive),
    Bool(Primitive),
    Null(Primitive),
    List(Compound),
    Dict(Compound),
    DictPair(KeyValuePair),
    Identifier(Primitive),
    Slot(Primitive),
}

impl AstNode {
    pub fn span(&self) -> &TokSpan {
        match self {
            AstNode::Call((_, c)) => &c.span,
            AstNode::Number(p)
            | AstNode::String(p)
            | AstNode::Bool(p)
            | AstNode::Null(p)
            | AstNode::Identifier(p)
            | AstNode::Slot(p) => &p.span,
            AstNode::List(c) | AstNode::Dict(c) => &c.span,
            AstNode::DictPair(p) => &p.span,
        }
    }
}
