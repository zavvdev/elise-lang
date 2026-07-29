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
    // TODO: Review variants; need to narrow down;
    const TRUE_VARIANTS: [&str; 4] = ["true", "yes", "on", "y"];
    const FALSE_VARIANTS: [&str; 4] = ["false", "no", "off", "n"];
    pub const TRUE_LEXEME: &'static str = "true";
    pub const FALSE_LEXEME: &'static str = "false";

    pub fn is_true(lexeme: &str) -> bool {
        Self::TRUE_VARIANTS.contains(&lexeme.to_lowercase().as_str())
    }

    pub fn is_false(lexeme: &str) -> bool {
        Self::FALSE_VARIANTS.contains(&lexeme.to_lowercase().as_str())
    }

    pub fn is_bool(lexeme: &str) -> bool {
        Self::is_true(lexeme) || Self::is_false(lexeme)
    }
}

pub struct SchemaFnOptional;
impl SchemaFnOptional {
    pub const LEXEME: &'static str = "optional";
    // TODO: Add a predicate for optional value;
}
