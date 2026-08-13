#[derive(Debug, Eq, Hash, PartialEq, Clone)]
pub enum ResolutionPathSegment {
    // The beginning of the path.
    Root,
    // Represents any index. For example, when we want to
    // build a schema resolution path, we don't need to describe
    // what type each list element has, we can just say that if
    // our list is a list of integers, then any index points to
    // some data with type Int.
    AbstractIndex,
    // Any field like dict key.
    Field(String),
}
impl ResolutionPathSegment {
    // For anything that requires string representation, like error reports.
    pub fn as_str(&self) -> String {
        match self {
            ResolutionPathSegment::Root => "Root".to_string(),
            ResolutionPathSegment::AbstractIndex => "AbstractIndex".to_string(),
            ResolutionPathSegment::Field(name) => format!("Field(\"{}\")", name),
        }
    }
}

/// Data structure that allows us to represent a path to follow
/// in order to get some data. In our case we can use it to
/// describe a path to type descriptors or data itself.
///
/// Internal representation uses a Vector of PathSegment's
/// where the first segment must always be Root segment
/// which cannot be removed.
///
/// This data structure was created specifically for cases
/// when we use expressions that extract some data, for example:
/// .get(@data, "name")
/// In this case we can say that path is [Root, Field("name")].
///
/// This data structure is intended to be used for schema resolution
/// and data binding, where former is used at compilation stage,
/// and latter is used at runtime stage.
#[derive(Debug, Eq, Hash, PartialEq, Clone)]
pub struct ResolutionPath(Vec<ResolutionPathSegment>);
impl Default for ResolutionPath {
    fn default() -> Self {
        Self::new()
    }
}
impl ResolutionPath {
    // Root must always be the first segment.
    pub fn new() -> Self {
        Self(vec![ResolutionPathSegment::Root])
    }

    // It's better to map over segments and push them in order
    // to use logic inside push function.
    pub fn with_segments(segments: Vec<ResolutionPathSegment>) -> Self {
        let mut new = Self::new();
        for segment in segments {
            new.push(segment);
        }
        new
    }

    // Do not allow to push Root segment since it's there by default.
    pub fn push(&mut self, segment: ResolutionPathSegment) {
        if segment != ResolutionPathSegment::Root {
            self.0.push(segment);
        }
    }

    // Don't allow to remove the last element.
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

// ==================================================================
//
// TESTS START
//
// ==================================================================

#[cfg(test)]
mod tests {
    use crate::resolution_path::{ResolutionPath, ResolutionPathSegment};

    #[test]
    fn should_create_default_path() {
        let path = ResolutionPath::default();
        assert_eq!(path, ResolutionPath(vec![ResolutionPathSegment::Root]));
    }

    #[test]
    fn should_create_with_initial_segments() {
        let path = ResolutionPath::with_segments(vec![ResolutionPathSegment::AbstractIndex]);
        assert_eq!(
            path,
            ResolutionPath(vec![
                ResolutionPathSegment::Root,
                ResolutionPathSegment::AbstractIndex,
            ])
        );
    }

    #[test]
    fn should_skip_root_segment_if_creating_with_segments() {
        let path = ResolutionPath::with_segments(vec![
            ResolutionPathSegment::AbstractIndex,
            ResolutionPathSegment::Root,
        ]);
        assert_eq!(
            path,
            ResolutionPath(vec![
                ResolutionPathSegment::Root,
                ResolutionPathSegment::AbstractIndex,
            ])
        );
    }

    #[test]
    fn should_push_new_path_segment() {
        let mut path = ResolutionPath::new();
        assert_eq!(path, ResolutionPath(vec![ResolutionPathSegment::Root]));
        path.push(ResolutionPathSegment::Field("test".to_string()));
        assert_eq!(
            path,
            ResolutionPath(vec![
                ResolutionPathSegment::Root,
                ResolutionPathSegment::Field("test".to_string())
            ])
        );
    }

    #[test]
    fn should_not_allow_to_push_root_segment() {
        let mut path = ResolutionPath::new();
        assert_eq!(path, ResolutionPath(vec![ResolutionPathSegment::Root]));
        path.push(ResolutionPathSegment::Root);
        assert_eq!(path, ResolutionPath(vec![ResolutionPathSegment::Root,]));
    }

    #[test]
    fn should_pop_last_segment() {
        let mut path = ResolutionPath::new();
        path.push(ResolutionPathSegment::Field("test".to_string()));
        assert_eq!(
            path,
            ResolutionPath(vec![
                ResolutionPathSegment::Root,
                ResolutionPathSegment::Field("test".to_string())
            ])
        );
        path.pop();
        assert_eq!(path, ResolutionPath(vec![ResolutionPathSegment::Root,]));
    }

    #[test]
    fn should_not_pop_if_len_is_1() {
        let mut path = ResolutionPath::new();
        assert_eq!(path, ResolutionPath(vec![ResolutionPathSegment::Root]));
        path.pop();
        assert_eq!(path, ResolutionPath(vec![ResolutionPathSegment::Root]));
    }

    #[test]
    fn should_return_segments_as_str() {
        let mut path = ResolutionPath::new();
        path.push(ResolutionPathSegment::Field("test".to_string()));
        path.push(ResolutionPathSegment::AbstractIndex);
        assert_eq!(path.as_str(), "[Root, Field(\"test\"), AbstractIndex]");
    }
}

// ==================================================================
//
// TESTS END
//
// ==================================================================
