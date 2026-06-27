#[derive(Debug, PartialEq)]
pub enum LangPrimitiveType {
    Int,
    Float,
    // TODO: Add more
}

#[derive(Debug, PartialEq)]
pub enum LangType {
    Primitive(LangPrimitiveType),
    // TODO: Add compounds
}
