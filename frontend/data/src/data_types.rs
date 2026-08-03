#[derive(Debug, Eq, Hash, PartialEq, Clone)]
pub enum ResolutionPathSegment {
    Root,
    Index(usize),
    AbstractIndex,
    Field(String),
}

pub type ResolutionPath = Vec<ResolutionPathSegment>;
