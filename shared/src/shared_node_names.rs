pub struct NodeName;
impl NodeName {
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

    pub const TYPEDEF: &'static str = "Type definition";

    pub const SYMBOL: &'static str = "Symbol";
    pub const PRIMITIVE: &'static str = "Primitive";

    pub const FN_LET: &'static str = ".let";
    pub const FN_TYPEDEF: &'static str = ".typedef";
    pub const FN_GET: &'static str = ".get";
    pub const FN_ADD: &'static str = ".add";
}
