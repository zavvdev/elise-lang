use elise_types::Span;

#[derive(Debug, PartialEq)]
pub enum SemanalyzerErr {
    SymbolUndefined { span: Span },
    SymbolDuplicate { span: Span },
    Unknown,
}
