use crate::shared_types::{ArityMismatchKind, Span};

#[derive(Debug, PartialEq)]
pub enum SchemaResolverErr {
    Empty,
    Unexp {
        span: Span,
    },
    ArityMismatch {
        fn_name: &'static str,
        expected: usize,
        kind: ArityMismatchKind,
        found: usize,
        span: Span,
    },
    UnresolvablePath {
        path: String,
    },
    InvalTypeDef {
        span: Span,
    },
    InvalDict {
        span: Span,
    },
    InvalUseOfModifier {
        span: Span,
    },
}
