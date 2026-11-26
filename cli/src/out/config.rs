pub struct ErrCategory;
impl ErrCategory {
    // Errors related to the file system actions like Read or Write.
    pub const FS: &'static str = "File System Error";

    // Grammar malformations.
    pub const SYNTAX: &'static str = "Syntax Error";

    // Something is named or pathed, and it doesn't exist.
    pub const REFERENCE: &'static str = "Reference Error";

    // Something exists, but its type is wrong, incomplete, inconsistent or unknown.
    pub const TYPE: &'static str = "Type Error";

    // Something's well-formed per the grammar, but not allowed in this position or shape.
    pub const SEMANTIC: &'static str = "Semantic Error";
}
