use elise_shared::shared_types::Span;

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
    // TODO: Return an owned Span?
    pub fn span(&self) -> &Span {
        match self {
            AAstNodeTypedef::Custom { span, .. } => span,
            AAstNodeTypedef::Record { span, .. } => span,
            AAstNodeTypedef::List { span, .. } => span,
            AAstNodeTypedef::Int { span, .. } => span,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum AAstNodeCall {
    Typedef {
        alias: String,
        typedef: AAstNodeTypedef,
    },
}

impl AAstNodeCall {
    pub fn span(&self) -> &Span {
        match self {
            AAstNodeCall::Typedef { typedef, .. } => typedef.span(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum AAstNode {
    Call(AAstNodeCall),
}

impl AAstNode {
    pub fn span(&self) -> &Span {
        match self {
            AAstNode::Call(call) => call.span(),
        }
    }
}
