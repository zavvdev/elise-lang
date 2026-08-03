#[derive(Debug, Eq, Hash, PartialEq, Clone)]
pub enum ResolutionPathSegment {
    Root,
    Index(usize),
    AbstractIndex,
    Field(String),
}
impl ResolutionPathSegment {
    pub fn as_str(&self) -> String {
        match self {
            ResolutionPathSegment::Root => "Root".to_string(),
            ResolutionPathSegment::Index(i) => format!("Index({})", i),
            ResolutionPathSegment::AbstractIndex => "AbstractIndex".to_string(),
            ResolutionPathSegment::Field(name) => format!("Field({})", name),
        }
    }
}

#[derive(Debug, Eq, Hash, PartialEq, Clone)]
pub struct ResolutionPath(Vec<ResolutionPathSegment>);
impl Default for ResolutionPath {
    fn default() -> Self {
        Self::new()
    }
}

impl ResolutionPath {
    pub fn new() -> Self {
        Self(vec![ResolutionPathSegment::Root])
    }

    pub fn push(&mut self, segment: ResolutionPathSegment) {
        self.0.push(segment);
    }

    pub fn pop(&mut self) -> Option<ResolutionPathSegment> {
        if self.0.len() > 1 {
            return self.0.pop();
        }
        None
    }

    pub fn as_str(&self) -> String {
        format!(
            "[{}]",
            self.0
                .iter()
                .map(|seg| seg.as_str())
                .collect::<Vec<_>>()
                .join(", ")
        )
    }
}
