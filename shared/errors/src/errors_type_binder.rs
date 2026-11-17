use elise_shared_types::Span;

#[derive(Debug, PartialEq)]
pub enum TypeBinderErr {
    UnresolvablePath { path: String },
    UnresolvableType { span: Span, name: String },
    UnableToMerge { span: Span },
}
