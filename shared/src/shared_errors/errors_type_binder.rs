use crate::shared_types::Span;

#[derive(Debug, PartialEq)]
pub enum TypeBinderErr {
    UnresolvablePath { path: String },
    Unresolvable { span: Span },
    UnableToMerge { span: Span },
}
