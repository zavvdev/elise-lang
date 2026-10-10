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

    fn new_sized(size: usize) -> Self {
        let mut new = Vec::with_capacity(size);
        new.insert(0, BindingPathSegment::Root);
        Self(new)
    }

    pub fn with_segments(segments: Vec<BindingPathSegment>) -> Self {
        let Some(len) = segments.len().checked_add(1) else {
            let mut new = Self::new();
            for segment in segments {
                new.push(segment);
            }
            return new;
        };

        let mut new = Self::new_sized(len);

        // It's better to map over segments and push them in order
        // to use logic inside our push function.
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

    /// Path is considered valid if it's not empty,
    /// and has only one Root segment at the beginning.
    pub fn is_valid(binding_path: &BindingPath) -> bool {
        let root_indexes: Vec<_> = binding_path
            .iter()
            .enumerate()
            .filter(|(_, k)| **k == BindingPathSegment::Root)
            .map(|(i, _)| i)
            .collect();

        matches!(root_indexes.as_slice(), [0])
    }

    /// Merges two paths into a new one. All segments from prepend_path go
    /// to the very beginning, and the rest of the to_path except the Root
    /// segment goes after.
    pub fn prepend(prepend_path: &BindingPath, to_path: &BindingPath) -> Option<BindingPath> {
        if !Self::is_valid(to_path) || !Self::is_valid(prepend_path) {
            return None;
        }

        // The new length is a prepend_path length + to_path length - 1
        // because the Root path segment of the to_path is dropped.
        // We can safely subtract 1 because zero length path is considered
        // as invalid and it's already checked previously.
        let new_path_len = prepend_path.len().checked_add(to_path.len() - 1)?;

        let mut new_path = Vec::with_capacity(new_path_len);

        // Clones each element from prepend_path into new_path.
        // So our new Root segment is a Root from prepend_path.
        new_path.extend_from_slice(prepend_path);

        // Clones each element from to_path except the first into
        // the new_path.
        new_path.extend_from_slice(to_path.get(1..).unwrap_or_default());

        Some(BindingPath(new_path))
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
