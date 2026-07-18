use elise_types::Span;

// TODO: Improve
#[derive(Debug, PartialEq)]
pub enum SemanalyzerErr {
    SymbolUndefined { span: Span },
    SymbolDuplicate { span: Span },

    DefineFnArgsLen { span: Span },
    DefineFnFirstArgIdentifier { span: Span },
    DefineFnSecondArgType { span: Span },

    UnknownFunction { span: Span },
    Unknown,
}
