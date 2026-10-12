use elise_shared::shared_types::Span;

#[derive(Debug, PartialEq)]
pub enum AAstNode {
    Call(AAstNodeCall),
    Data(AAstNodeData),
    SlotData { span: Span },
}

impl AAstNode {
    pub fn span(&self) -> Span {
        match self {
            AAstNode::Call(call) => call.span(),
            AAstNode::Data(data) => data.span(),
            AAstNode::SlotData { span } => span.clone(),
        }
    }
}

// ==================================================================
//
// CALL START
//
// ==================================================================

#[derive(Debug, PartialEq)]
pub enum AAstNodeCall {
    Typedef {
        alias: String,
        typedef: AAstNodeTypedef,
    },
}

impl AAstNodeCall {
    pub fn span(&self) -> Span {
        match self {
            AAstNodeCall::Typedef { typedef, .. } => typedef.span(),
        }
    }
}

// ==================================================================
//
// CALL END
//
// ==================================================================

// ==================================================================
//
// DATA START
//
// ==================================================================

#[derive(Debug, PartialEq)]
pub struct AAstNodeDataPrim {
    pub lexeme: String,
    pub span: Span,
}

#[derive(Debug, PartialEq)]
pub struct AAstNodeDataDictKey {
    pub span: Span,
    pub lexeme: String,
}

#[derive(Debug, PartialEq)]
pub enum AAstNodeData {
    Int(AAstNodeDataPrim),
    Str(AAstNodeDataPrim),
    List {
        span: Span,
        items: Vec<Box<AAstNodeData>>,
    },
    Dict {
        span: Span,
        entries: Vec<(AAstNodeDataDictKey, Box<AAstNodeData>)>,
    },
}

impl AAstNodeData {
    pub fn span(&self) -> Span {
        match self {
            AAstNodeData::Int(prim) | AAstNodeData::Str(prim) => prim.span.clone(),
            AAstNodeData::List { span, .. } => span.clone(),
            AAstNodeData::Dict { span, .. } => span.clone(),
        }
    }
}

// ==================================================================
//
// DATA END
//
// ==================================================================

// ==================================================================
//
// TYPEDEF START
//
// ==================================================================

pub type AAstNodeTypedefRecordEntries = Vec<(String, Box<AAstNodeTypedef>)>;

#[derive(Debug, PartialEq, Clone)]
pub enum AAstNodeTypedef {
    Custom {
        span: Span,
        alias: String,
    },
    Record {
        span: Span,
        entries: AAstNodeTypedefRecordEntries,
    },
    List {
        span: Span,
        item_type: Box<AAstNodeTypedef>,
    },
    Int {
        span: Span,
    },
}

impl AAstNodeTypedef {
    pub fn span(&self) -> Span {
        match self {
            AAstNodeTypedef::Custom { span, .. } => span.clone(),
            AAstNodeTypedef::Record { span, .. } => span.clone(),
            AAstNodeTypedef::List { span, .. } => span.clone(),
            AAstNodeTypedef::Int { span, .. } => span.clone(),
        }
    }
}

// ==================================================================
//
// TYPEDEF END
//
// ==================================================================
