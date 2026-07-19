use elise_types::Span;

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
    UnsupportedCallKind {
        span: Span,
    },
}
