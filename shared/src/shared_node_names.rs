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
    pub const TYPEDEF: &'static str = "Typedef";
}
