use elise_shared_types::Span;

#[derive(Debug, PartialEq)]
pub enum ArityMismatchKind {
    Eq(usize),
    MoreEq(usize),
    LessEq(usize),
    More(usize),
    Less(usize),
    Range((usize, usize)),
}
impl ArityMismatchKind {
    pub fn as_string(&self) -> String {
        match self {
            ArityMismatchKind::Eq(n) => format!("{}", n),
            ArityMismatchKind::MoreEq(n) => format!(">={n}"),
            ArityMismatchKind::LessEq(n) => format!("<={n}"),
            ArityMismatchKind::More(n) => format!(">{n}"),
            ArityMismatchKind::Less(n) => format!("<{n}"),
            ArityMismatchKind::Range((min, max)) => format!(">={min}, <={max}"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum SemanalyzerErr {
    // =================================
    // UNKNOWN START
    // =================================
    UnknFnCall {
        span: Span,
        name: String,
    },
    UnknSlot {
        span: Span,
        name: String,
    },
    UnknType {
        span: Span,
        name: String,
    },
    // =================================
    // UNKNOWN END
    // =================================

    // =================================
    // UNEXPECTED START
    // =================================
    UnexpNode {
        span: Span,
        expected: Vec<String>,
        found: String,
    },
    UnexpExpr {
        span: Span,
        found: String,
        expected: Vec<String>,
    },
    UnexpType {
        span: Span,
        expected: Vec<String>,
        found: String,
    },
    // =================================
    // UNEXPECTED END
    // =================================

    // =================================
    // MISSING START
    // =================================
    MissingGeneric {
        span: Span,
        type_annot: String,
    },

    MissingTypeDefinition {
        span: Span,
        for_name: String,
    },
    // =================================
    // MISSING END
    // =================================
    ArityMismatch {
        fn_name: &'static str,
        kind: ArityMismatchKind,
        found: usize,
        span: Span,
    },

    InvalGeneric {
        span: Span,
        type_annot: String,
    },

    UnresolvableTypePath {
        path: String,
        span: Span,
    },

    UnableToCoalesceType {
        name: String,
        span: Span,
    },

    TypedefContextViolation {
        span: Span,
    },

    InconsistentListItems {
        span: Span,
    },

    CannotInferType {
        span: Span,
        name: String,
    },

    UndefinedDataAccess {
        span: Span,
    },
}
