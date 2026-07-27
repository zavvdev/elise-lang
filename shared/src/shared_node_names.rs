pub struct NodeName;

impl NodeName {
    pub const INT: &'static str = "Int";
    pub const FLOAT: &'static str = "Float";
    pub const STRING: &'static str = "String";
    pub const BOOL: &'static str = "Bool";
    pub const NULL: &'static str = "Null";
    pub const LIST: &'static str = "List";
    pub const DICT: &'static str = "Dict";
    pub const DICT_PAIR: &'static str = "DictPair";
    pub const IDENTIFIER: &'static str = "Identifier";
    pub const SLOT: &'static str = "Slot";
    pub const CALL: &'static str = "Call";
    pub const SYMBOL: &'static str = "Symbol";
    pub const PRIMITIVE: &'static str = "Primitive";
    pub const EMPTY: &'static str = "Empty";
    pub const CALL_DEFINE: &'static str = "CallDefine";
    pub const CALL_LET: &'static str = "CallLet";
}
