use crate::shared_types::{ArityMismatchKind, Span};

#[derive(Debug, PartialEq)]
pub enum SemanalyzerErr {
    ArityMismatch {
        fn_name: &'static str,
        kind: ArityMismatchKind,
        found: usize,
        span: Span,
    },
    UnknownFunction {
        span: Span,
    },
    UnsupportedNode {
        span: Span,
    },
    ExpectedTypedef {
        span: Span,
    },
    UnknownTypedef {
        span: Span,
    },
    ExpectedIdentifier {
        span: Span,
    },
    UnexpectedGeneric {
        span: Span,
    },
    ExpectedGeneric {
        span: Span,
    },
    InvalidGeneric {
        span: Span,
    },
    UnresolvableTypedef {
        span: Span,
    },
    TypedefNoReferenceItself {
        span: Span,
    },
    TypedefMode {
        span: Span,
    },
}
