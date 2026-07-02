use elise_types::Span;

#[derive(Debug, PartialEq)]
pub enum SemanticAnalyzerErr {
    SymbolUndefined { span: Span },
    SymbolDuplicate { span: Span },
}
