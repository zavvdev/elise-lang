use crate::shared_types::{ArityMismatchKind, Span};

#[derive(Debug, PartialEq)]
pub enum TypeBinderErr {
    //Empty,
    //Unexp {
    //    span: Span,
    //},
    //ArityMismatch {
    //    fn_name: &'static str,
    //    kind: ArityMismatchKind,
    //    found: usize,
    //    span: Span,
    //},
    UnresolvablePath {
        path: String,
    },
    UnknownTypedef {
        span: Span,
    },
    //InvalTypeDef {
    //    span: Span,
    //},
    //InvalDict {
    //    span: Span,
    //},
    //InvalUseOfModifier {
    //    span: Span,
    //},
    //UndexpType {
    //    expected: String,
    //    found: String,
    //    span: Span,
    //},
    //NoUnionOfUnion {
    //    span: Span,
    //},
}
