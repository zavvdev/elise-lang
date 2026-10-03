use crate::shared_types::Span;

#[derive(Debug, PartialEq)]
pub enum TypeBinderErr {
    UnresolvablePath { path: String },
    UnknownTypedef { span: Span },
}
