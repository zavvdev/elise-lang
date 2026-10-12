use elise_bindings::{TypeBinding, binding_path::BindingPath};
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
    Get {
        span: Span,
        binding_path: BindingPath,

        // This is basically a return type of this function.
        // TODO: We need to have an ability to get a slice
        // from a TypeBinding from the beginning of the node
        // this function's binding_path resolves to, to the
        // end of the binding.
        // For example:
        // If we have this TypeBinding that is used as the first
        // argument of the .get function
        // [Root, Field("user")] => Dict
        // [Root, Field("user"), Field("id")] => Int
        // and we do .get(@data, "user"),
        // then the return type binding must be like:
        // [Root] => Dict
        // [Root, Field("id")] => Int
        return_type_binding: TypeBinding,
    },
}

impl AAstNodeCall {
    pub fn span(&self) -> Span {
        match self {
            AAstNodeCall::Typedef { typedef, .. } => typedef.span(),
            AAstNodeCall::Get { span, .. } => span.clone(),
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
