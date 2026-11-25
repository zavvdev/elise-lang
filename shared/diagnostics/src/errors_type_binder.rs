use elise_shared_types::Span;

#[derive(Debug, PartialEq)]
pub enum TypeBinderErr {
    UnresolvablePath { path: String, span: Span },
    UnresolvableType { span: Span, name: String },
    UnableToCoalesce { span: Span, name: String },
}
