pub struct SchemaFnRoot;
impl SchemaFnRoot {
    pub const LEXEME: &'static str = "schema";
}

pub struct SchemaFnRow;
impl SchemaFnRow {
    pub const LEXEME: &'static str = "row";
}

pub struct SchemaFnInt;
impl SchemaFnInt {
    pub const LEXEME: &'static str = "int";
}

pub struct SchemaFnFloat;
impl SchemaFnFloat {
    pub const LEXEME: &'static str = "float";
}

pub struct SchemaFnString;
impl SchemaFnString {
    pub const LEXEME: &'static str = "string";
}

pub struct SchemaFnBool;
impl SchemaFnBool {
    pub const LEXEME: &'static str = "bool";
}

pub struct SchemaFnOptional;
impl SchemaFnOptional {
    pub const LEXEME: &'static str = "optional";
    // TODO: Add a predicate for optional value;
}
