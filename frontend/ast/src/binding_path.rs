use std::ops::Deref;

#[derive(Debug, Eq, Hash, PartialEq, Clone)]
pub enum BindingPathSegment {
    // The beginning of the path.
    Root,

    // Represents any index. For example, when we want to
    // build a schema binding path, we don't need to describe
    // what type each list element has, we can just say that if
    // our list is a list of integers, then any index points to
    // some data with type Int.
    AbstractIndex,

    // Specific index.
    Index(usize),

    // Any field like dict key.
    Field(String),
}
impl BindingPathSegment {
    // For anything that requires string representation, like error reports.
    pub fn as_str(&self) -> String {
        match self {
            BindingPathSegment::Root => "Root".to_string(),
            BindingPathSegment::AbstractIndex => "AbstractIndex".to_string(),
            BindingPathSegment::Field(name) => format!("Field(\"{}\")", name),
            BindingPathSegment::Index(idx) => format!("Index(\"{}\")", idx),
        }
    }
}

/// Data structure that allows us to represent a path to follow
/// in order to get some data. In our case we can use it to
/// describe a path to type descriptors or data itself.
///
/// Internal representation uses a Vector of path segments
/// where the first segment must always be Root segment
/// which cannot be removed.
///
/// This data structure was created specifically for cases
/// when we use expressions that extract some data, for example:
/// .get(@data, "name")
/// In this case we can say that path is [Root, Field("name")].
///
/// This data structure is intended to be used for schema binding
/// and data binding, where former is used at compilation stage,
/// and latter is used at runtime stage.
#[derive(Debug, Eq, Hash, PartialEq, Clone)]
pub struct BindingPath(pub Vec<BindingPathSegment>);

// Implementing Deref gives us an ability to extract
// the underlying vector in order to use .iter(), .len(),
// .first etc without implementing their traits separately
// (like Index trait or IntoIterator). So we have all native
// Vec methods for free.
impl Deref for BindingPath {
    type Target = [BindingPathSegment];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Default for BindingPath {
    fn default() -> Self {
        Self::new()
    }
}

impl BindingPath {
    // Root must always be the first segment.
    pub fn new() -> Self {
        Self(vec![BindingPathSegment::Root])
    }

    // It's better to map over segments and push them in order
    // to use logic inside push function.
    pub fn with_segments(segments: Vec<BindingPathSegment>) -> Self {
        let mut new = Self::new();
        for segment in segments {
            new.push(segment);
        }
        new
    }

    // Do not allow to push Root segment since it's there by default.
    pub fn push(&mut self, segment: BindingPathSegment) {
        if segment != BindingPathSegment::Root {
            self.0.push(segment);
        }
    }

    // Don't allow to remove the last element.
    pub fn pop(&mut self) -> Option<BindingPathSegment> {
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
    use crate::binding_path::{BindingPath, BindingPathSegment};

    #[test]
    fn should_create_default_path() {
        let path = BindingPath::default();
        assert_eq!(path, BindingPath(vec![BindingPathSegment::Root]));
    }

    #[test]
    fn should_create_with_initial_segments() {
        let path = BindingPath::with_segments(vec![BindingPathSegment::AbstractIndex]);
        assert_eq!(
            path,
            BindingPath(vec![
                BindingPathSegment::Root,
                BindingPathSegment::AbstractIndex,
            ])
        );
    }

    #[test]
    fn should_skip_root_segment_if_creating_with_segments() {
        let path = BindingPath::with_segments(vec![
            BindingPathSegment::AbstractIndex,
            BindingPathSegment::Root,
        ]);
        assert_eq!(
            path,
            BindingPath(vec![
                BindingPathSegment::Root,
                BindingPathSegment::AbstractIndex,
            ])
        );
    }

    #[test]
    fn should_push_new_path_segment() {
        let mut path = BindingPath::new();
        assert_eq!(path, BindingPath(vec![BindingPathSegment::Root]));
        path.push(BindingPathSegment::Field("test".to_string()));
        assert_eq!(
            path,
            BindingPath(vec![
                BindingPathSegment::Root,
                BindingPathSegment::Field("test".to_string())
            ])
        );
    }

    #[test]
    fn should_not_allow_to_push_root_segment() {
        let mut path = BindingPath::new();
        assert_eq!(path, BindingPath(vec![BindingPathSegment::Root]));
        path.push(BindingPathSegment::Root);
        assert_eq!(path, BindingPath(vec![BindingPathSegment::Root,]));
    }

    #[test]
    fn should_pop_last_segment() {
        let mut path = BindingPath::new();
        path.push(BindingPathSegment::Field("test".to_string()));
        assert_eq!(
            path,
            BindingPath(vec![
                BindingPathSegment::Root,
                BindingPathSegment::Field("test".to_string())
            ])
        );
        path.pop();
        assert_eq!(path, BindingPath(vec![BindingPathSegment::Root,]));
    }

    #[test]
    fn should_not_pop_if_len_is_1() {
        let mut path = BindingPath::new();
        assert_eq!(path, BindingPath(vec![BindingPathSegment::Root]));
        path.pop();
        assert_eq!(path, BindingPath(vec![BindingPathSegment::Root]));
    }

    #[test]
    fn should_return_segments_as_str() {
        let mut path = BindingPath::new();
        path.push(BindingPathSegment::Field("test".to_string()));
        path.push(BindingPathSegment::AbstractIndex);
        assert_eq!(path.as_str(), "[Root, Field(\"test\"), AbstractIndex]");
    }
}

// ==================================================================
//
// TESTS END
//
// ==================================================================
