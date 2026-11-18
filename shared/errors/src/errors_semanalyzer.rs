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
    ArityMismatch {
        fn_name: &'static str,
        kind: ArityMismatchKind,
        found: usize,
        span: Span,
    },

    // =================================
    // UNKNOWNS START
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
    // UNKNOWNS END
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
    MissingGeneric {
        span: Span,
        type_annot: String,
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
