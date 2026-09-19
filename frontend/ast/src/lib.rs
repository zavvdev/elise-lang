use elise_shared::shared_types::Span;

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

// ==================================================================
//
//  EXPR START
//
// ==================================================================

#[derive(Debug, PartialEq)]
pub struct AstNodeExprCall {
    pub lexeme: String,
    pub span: Span,
    pub body: Vec<Box<AstNode>>,
}

#[derive(Debug, PartialEq)]
pub struct AstNodeExprPrim {
    pub lexeme: String,
    pub span: Span,
}

#[derive(Debug, PartialEq)]
pub struct AstNodeExprList {
    pub span: Span,
    pub items: Vec<Box<AstNodeExpr>>,
}

#[derive(Debug, PartialEq)]
pub struct AstNodeExprDictKey {
    pub span: Span,
    pub lexeme: String,
}

#[derive(Debug, PartialEq)]
pub struct AstNodeExprDict {
    pub span: Span,
    pub entries: Vec<(AstNodeExprDictKey, Box<AstNodeExpr>)>,
}

#[derive(Debug, PartialEq)]
pub enum AstNodeExpr {
    Call(AstNodeExprCall),
    Int(AstNodeExprPrim),
    Float(AstNodeExprPrim),
    Str(AstNodeExprPrim),
    Bool(AstNodeExprPrim),
    Null(AstNodeExprPrim),
    List(AstNodeExprList),
    Dict(AstNodeExprDict),
    Ident(AstNodeExprPrim),
    Slot(AstNodeExprPrim),
}

impl AstNodeExpr {
    pub fn span(&self) -> &Span {
        match self {
            AstNodeExpr::Int(p)
            | AstNodeExpr::Float(p)
            | AstNodeExpr::Str(p)
            | AstNodeExpr::Bool(p)
            | AstNodeExpr::Ident(p)
            | AstNodeExpr::Null(p)
            | AstNodeExpr::Slot(p) => &p.span,
            AstNodeExpr::List(l) => &l.span,
            AstNodeExpr::Dict(d) => &d.span,
            AstNodeExpr::Call(c) => &c.span,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            AstNodeExpr::Int(_) => AstNodeExprName::INT,
            AstNodeExpr::Float(_) => AstNodeExprName::FLOAT,
            AstNodeExpr::Str(_) => AstNodeExprName::STR,
            AstNodeExpr::Bool(_) => AstNodeExprName::BOOL,
            AstNodeExpr::Ident(_) => AstNodeExprName::IDENT,
            AstNodeExpr::Null(_) => AstNodeExprName::NULL,
            AstNodeExpr::Slot(_) => AstNodeExprName::SLOT,
            AstNodeExpr::Dict(_) => AstNodeExprName::DICT,
            AstNodeExpr::List(_) => AstNodeExprName::LIST,
            AstNodeExpr::Call(_) => AstNodeExprName::CALL,
        }
    }
}

pub struct AstNodeExprName;
impl AstNodeExprName {
    pub const INT: &'static str = "Int";
    pub const FLOAT: &'static str = "Float";
    pub const STR: &'static str = "String";
    pub const BOOL: &'static str = "Bool";
    pub const IDENT: &'static str = "Identifier";
    pub const NULL: &'static str = "Null";
    pub const SLOT: &'static str = "Slot";
    pub const LIST: &'static str = "List";
    pub const DICT: &'static str = "Dict";
    pub const CALL: &'static str = "Call";
}

// ==================================================================
//
//  EXPR END
//
// ==================================================================

// ==================================================================
//
//  TYPEDEF START
//
// ==================================================================

#[derive(Debug, PartialEq)]
pub struct AstNodeTypedefDesc {
    pub span: Span,
    pub nullable: bool,
    pub optional: bool,
}

#[derive(Debug, PartialEq)]
pub struct AstNodeTypedefPrim {
    pub desc: AstNodeTypedefDesc,
}

#[derive(Debug, PartialEq)]
pub struct AstNodeTypedefComp {
    pub desc: AstNodeTypedefDesc,
    pub generic: Box<AstNodeTypedef>,
}

#[derive(Debug, PartialEq)]
pub enum AstNodeTypedef {
    TInt(AstNodeTypedefPrim),
    TFloat(AstNodeTypedefPrim),
    TStr(AstNodeTypedefPrim),
    TBool(AstNodeTypedefPrim),
    TList(AstNodeTypedefComp),
    TRecord(AstNodeTypedefComp),
}

impl AstNodeTypedef {
    pub fn span(&self) -> &Span {
        match self {
            AstNodeTypedef::TInt(p)
            | AstNodeTypedef::TFloat(p)
            | AstNodeTypedef::TStr(p)
            | AstNodeTypedef::TBool(p) => &p.desc.span,
            AstNodeTypedef::TList(c) | AstNodeTypedef::TRecord(c) => &c.desc.span,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            AstNodeTypedef::TInt(_) => AstNodeTypedefName::TINT,
            AstNodeTypedef::TFloat(_) => AstNodeTypedefName::TFLOAT,
            AstNodeTypedef::TStr(_) => AstNodeTypedefName::TSTR,
            AstNodeTypedef::TBool(_) => AstNodeTypedefName::TBOOL,
            AstNodeTypedef::TList(_) => AstNodeTypedefName::TLIST,
            AstNodeTypedef::TRecord(_) => AstNodeTypedefName::TRECORD,
        }
    }
}

pub struct AstNodeTypedefName;
impl AstNodeTypedefName {
    pub const TINT: &'static str = "TypeInt";
    pub const TFLOAT: &'static str = "TypeFloat";
    pub const TSTR: &'static str = "TypeString";
    pub const TBOOL: &'static str = "TypeBool";
    pub const TLIST: &'static str = "TypeList";
    pub const TRECORD: &'static str = "TypeRecord";
}

// ==================================================================
//
//  TYPEDEF END
//
// ==================================================================
