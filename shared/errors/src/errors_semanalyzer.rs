use elise_shared_types::{ArityMismatchKind, Span};

// TODO: Add more precise errors.
#[derive(Debug, PartialEq)]
pub enum SemanalyzerErr {
    ArityMismatch {
        fn_name: &'static str,
        kind: ArityMismatchKind,
        found: usize,
        span: Span,
    },

    UnknFn {
        span: Span,
    },
    UnknSlot {
        span: Span,
    },
    UnknType {
        span: Span,
    },

    UnexpNode {
        span: Span,
    },
    UnexpGeneric {
        span: Span,
    },
    UnexpType {
        span: Span,
        expected: Vec<String>,
        found: String,
    },

    // TODO: Maybe we can collapse them into one?
    ExpTypeAnnot {
        span: Span,
    },
    ExpIdentifier {
        span: Span,
    },
    ExpGeneric {
        span: Span,
    },
    ExpExpression {
        span: Span,
    },

    UnsupListItem {
        span: Span,
    },
    UnsupDictItem {
        span: Span,
    },

    InvalGeneric {
        span: Span,
    },
    InvalDataAccessArg {
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
    CannotInferType {
        span: Span,
    },
    NoTypeDefinition {
        span: Span,
    },
    UnresolvablePath {
        span: Span,
    },
}
