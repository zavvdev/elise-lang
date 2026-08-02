use crate::shared_types::{ArityMismatchKind, Span};

#[derive(Debug, PartialEq)]
pub enum SchemaResolverErr {
    ArityMismatch {
        fn_name: &'static str,
        expected: usize,
        kind: ArityMismatchKind,
        found: usize,
        span: Span,
    },
    InvalTypeDef {
        span: Span,
    },
    Todo,
    InvalRoot {
        span: Span,
    },

    ArgsLen {
        span: Span,
    },

    ColInvalName {
        span: Span,
    },
    ColInvalType {
        span: Span,
    },
    ColTypeNoArgs {
        span: Span,
    },
    ColDuplicate {
        span: Span,
    },

    OptArgsLen {
        span: Span,
    },

    OptOpt {
        span: Span,
    },
}
