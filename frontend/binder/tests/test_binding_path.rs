use elise_binder::binding_path::{BindingPath, BindingPathSegment};

mod common;

#[test]
fn should_create_default_path() {
    let path = BindingPath::new("Data".to_string());
    assert_eq!(
        path,
        BindingPath(vec![BindingPathSegment::Alias("Data".to_string())])
    );
}

#[test]
fn should_create_with_initial_segments() {
    let path =
        BindingPath::with_segments("Data".to_string(), vec![BindingPathSegment::AbstractIndex]);
    assert_eq!(
        path,
        BindingPath(vec![
            BindingPathSegment::Alias("Data".to_string()),
            BindingPathSegment::AbstractIndex,
        ])
    );
}

#[test]
fn should_skip_alias_segment_if_creating_with_segments() {
    let path = BindingPath::with_segments(
        "Data".to_string(),
        vec![
            BindingPathSegment::AbstractIndex,
            BindingPathSegment::Alias("Some".to_string()),
        ],
    );
    assert_eq!(
        path,
        BindingPath(vec![
            BindingPathSegment::Alias("Data".to_string()),
            BindingPathSegment::AbstractIndex,
        ])
    );
}

#[test]
fn should_push_new_path_segment() {
    let mut path = BindingPath::new("Data".to_string());
    assert_eq!(
        path,
        BindingPath(vec![BindingPathSegment::Alias("Data".to_string())])
    );
    path.push(BindingPathSegment::Field("test".to_string()));
    assert_eq!(
        path,
        BindingPath(vec![
            BindingPathSegment::Alias("Data".to_string()),
            BindingPathSegment::Field("test".to_string())
        ])
    );
}

#[test]
fn should_not_allow_to_push_alias_segment() {
    let mut path = BindingPath::new("Data".to_string());
    assert_eq!(
        path,
        BindingPath(vec![BindingPathSegment::Alias("Data".to_string())])
    );
    path.push(BindingPathSegment::Alias("Some".to_string()));
    assert_eq!(
        path,
        BindingPath(vec![BindingPathSegment::Alias("Data".to_string())])
    );
}

#[test]
fn should_pop_last_segment() {
    let mut path = BindingPath::new("Data".to_string());
    path.push(BindingPathSegment::Field("test".to_string()));
    assert_eq!(
        path,
        BindingPath(vec![
            BindingPathSegment::Alias("Data".to_string()),
            BindingPathSegment::Field("test".to_string())
        ])
    );
    path.pop();
    assert_eq!(
        path,
        BindingPath(vec![BindingPathSegment::Alias("Data".to_string())])
    );
}

#[test]
fn should_not_pop_if_len_is_1() {
    let mut path = BindingPath::new("Data".to_string());
    assert_eq!(
        path,
        BindingPath(vec![BindingPathSegment::Alias("Data".to_string())])
    );
    path.pop();
    assert_eq!(
        path,
        BindingPath(vec![BindingPathSegment::Alias("Data".to_string())])
    );
}

#[test]
fn should_return_segments_as_str() {
    let mut path = BindingPath::new("Data".to_string());
    path.push(BindingPathSegment::Field("test".to_string()));
    path.push(BindingPathSegment::AbstractIndex);
    assert_eq!(
        path.as_str(),
        "[Alias(\"Data\"), Field(\"test\"), AbstractIndex]"
    );
}
