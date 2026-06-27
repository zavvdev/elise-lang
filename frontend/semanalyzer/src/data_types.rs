#[derive(Debug)]
pub enum LangPrimitiveType {
    Int,
    Float,
    // TODO: Add more
}

#[derive(Debug)]
pub enum LangType {
    Primitive(LangPrimitiveType),
    // TODO: Add compounds
}
