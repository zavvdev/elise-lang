use crate::shared_types::{ArityMismatchKind, Span};

// TODO: Add more precise errors.
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
    UnknownSlot {
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
    TypedefMode {
        span: Span,
    },
    ListInconsistentItemTypes {
        span: Span,
    },
    UnsupportedListItem {
        span: Span,
    },
    UnsupportedDictItem {
        span: Span,
    },
    CannotInferType {
        span: Span,
    },
    NoTypeDefinition {
        span: Span,
    },
    InvalDataAccessArg {
        span: Span,
    },
    UnresolvablePath {
        span: Span,
    },
}
