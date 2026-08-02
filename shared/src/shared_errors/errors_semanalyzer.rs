use crate::shared_types::{ArityMismatchKind, Span};

#[derive(Debug, PartialEq)]
pub enum SemanalyzerErr {
    SymbolUndefined {
        span: Span,
    },
    SymbolDuplicate {
        span: Span,
    },
    ArityMismatch {
        fn_name: &'static str,
        expected: usize,
        kind: ArityMismatchKind,
        found: usize,
        span: Span,
    },
    ArgTypeMismatch {
        fn_name: &'static str,
        position: usize,
        expected: &'static str,
        found: &'static str,
        span: Span,
    },
    ArgKindMismatch {
        fn_name: &'static str,
        position: usize,
        expected: &'static str,
        found: &'static str,
        span: Span,
    },
    UnknownFunction {
        span: Span,
    },
    UnsupportedNode {
        span: Span,
    },
}
