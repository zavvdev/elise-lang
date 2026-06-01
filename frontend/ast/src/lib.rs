use elise_types::Span;

/// Represents a primitive value that does not
/// have any nested values inside. Numbers, strings, bools etc.
#[derive(Debug, PartialEq)]
pub struct Primitive {
    // Interpreted runtime string. Encoding aware.
    pub value: String,
    // Pointer to the original source of bytes.
    // Does not aware of any encoding.
    pub span: Span,
}

/// Represents a value that consists of other values like
/// lists, dictionaries or functions.
#[derive(Debug, PartialEq)]
pub struct Compound {
    pub span: Span,
    pub children: Vec<Box<AstNode>>,
}

/// Different kinds of functions.
/// We support named and anonymous for now.
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
    pub key_span: Span,
    // Value has its own span since it's AstNode.
    pub value: Box<AstNode>,
    // Span from the start of the key and
    // to the end of the value.
    pub span: Span,
}

#[derive(Debug, PartialEq)]
pub enum AstNode {
    Call((CallKind, Compound)),
    Number(Primitive),
    String(Primitive),
    Bool(Primitive),
    Null(Primitive),
    List(Compound),
    Dict(Compound),
    // We treat DictPair as an AstNode in order to be consistent
    // and always provide ast nodes as children for compound values.
    DictPair(KeyValuePair),
    Identifier(Primitive),
    Slot(Primitive),
}

impl AstNode {
    pub fn span(&self) -> &Span {
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
