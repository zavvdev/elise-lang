use std::ops::Deref;

#[derive(Debug, Eq, Hash, PartialEq, Clone)]
pub enum BindingPathSegment {
    // The beginning of the path represented as a string
    // associated with the entity being bind. For example,
    // it can be type definition name, or variable name
    // if we bind data.
    Alias(String),

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
            BindingPathSegment::Alias(alias) => format!("Alias(\"{}\")", alias),
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
/// where the first segment must always be Alias segment
/// which cannot be removed.
///
/// This data structure was created specifically for cases
/// when we use expressions that extract some data, for example:
/// .get(@data, "name")
/// In this case we can say that path is [Alias("Data"), Field("name")].
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

impl BindingPath {
    // Alias must always be the first segment.
    pub fn new(alias: String) -> Self {
        Self(vec![BindingPathSegment::Alias(alias)])
    }

    // It's better to map over segments and push them in order
    // to use logic inside push function.
    pub fn with_segments(alias: String, segments: Vec<BindingPathSegment>) -> Self {
        let mut new = Self::new(alias);
        for segment in segments {
            new.push(segment);
        }
        new
    }

    // Do not allow to push Alias segment since it's there by default.
    pub fn push(&mut self, segment: BindingPathSegment) {
        if !matches!(segment, BindingPathSegment::Alias(..)) {
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
