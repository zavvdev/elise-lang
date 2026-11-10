#[derive(Debug, PartialEq)]
pub enum DataBinderErr {
    UnresolvablePath { path: String },
    UnableToBind,
}
