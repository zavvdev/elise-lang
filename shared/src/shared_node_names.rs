pub struct NodeName;

impl NodeName {
    pub const INT: &'static str = "Int";
    pub const FLOAT: &'static str = "Float";
    pub const STR: &'static str = "String";
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
    pub const CALL_DEFINE: &'static str = "CallDefine";
    pub const TYPE_INT: &'static str = "TypeInt";
    pub const TYPE_FLOAT: &'static str = "TypeFloat";
    pub const TYPE_STR: &'static str = "TypeStr";
    pub const TYPE_BOOL: &'static str = "TypeBool";
    pub const TYPE_LIST: &'static str = "TypeList";
    pub const TYPE_RECORD: &'static str = "TypeRecord";
}
