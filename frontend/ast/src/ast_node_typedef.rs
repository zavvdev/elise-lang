use elise_shared::shared_types::Span;

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
