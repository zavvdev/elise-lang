use elise_bindings::binding_path::{BindingPath, BindingPathSegment};

mod common;

// ==================================================================
// DEFAULT START
// ==================================================================

#[test]
fn should_create_default_path() {
    let path = BindingPath::default();
    assert_eq!(path, BindingPath(vec![BindingPathSegment::Root]));
}

// ==================================================================
// DEFAULT END
// ==================================================================

// ==================================================================
// WITH SEGMENTS START
// ==================================================================

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

// ==================================================================
// WITH SEGMENTS END
// ==================================================================

// ==================================================================
// PUSH START
// ==================================================================

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

// ==================================================================
// PUSH END
// ==================================================================

// ==================================================================
// POP START
// ==================================================================

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

// ==================================================================
// POP END
// ==================================================================

// ==================================================================
// AS_STR START
// ==================================================================

#[test]
fn should_return_segments_as_str() {
    let mut path = BindingPath::new();
    path.push(BindingPathSegment::Field("test".to_string()));
    path.push(BindingPathSegment::AbstractIndex);
    assert_eq!(path.as_str(), "[Root, Field(\"test\"), AbstractIndex]");
}

// ==================================================================
// AS_STR END
// ==================================================================

// ==================================================================
// IS_VALID START
// ==================================================================

#[test]
fn should_return_true_for_valid_paths() {
    let valid_paths = vec![
        BindingPath::new(),
        BindingPath::with_segments(vec![BindingPathSegment::AbstractIndex]),
        BindingPath::with_segments(vec![
            BindingPathSegment::AbstractIndex,
            BindingPathSegment::AbstractIndex,
        ]),
    ];

    for valid_path in valid_paths {
        assert_eq!(BindingPath::is_valid(&valid_path), true);
    }
}

#[test]
fn should_return_false_for_invalid_paths() {
    let invalid_paths = vec![
        BindingPath(vec![]),
        BindingPath(vec![
            BindingPathSegment::Root,
            BindingPathSegment::AbstractIndex,
            BindingPathSegment::Root,
        ]),
        BindingPath(vec![
            BindingPathSegment::AbstractIndex,
            BindingPathSegment::Root,
        ]),
    ];

    for invalid_path in invalid_paths {
        assert_eq!(BindingPath::is_valid(&invalid_path), false);
    }
}

// ==================================================================
// IS_VALID END
// ==================================================================

// ==================================================================
// PREPEND START
// ==================================================================

#[test]
fn should_not_prepend_if_both_invalid() {
    let prepend_path = BindingPath(vec![]);
    let to_path = BindingPath(vec![
        BindingPathSegment::Root,
        BindingPathSegment::AbstractIndex,
        BindingPathSegment::Root,
    ]);
    assert_eq!(BindingPath::prepend(&prepend_path, &to_path), Err(()));
}

#[test]
fn should_not_prepend_if_prepend_path_invalid() {
    let prepend_path = BindingPath(vec![]);
    let to_path = BindingPath::with_segments(vec![BindingPathSegment::AbstractIndex]);
    assert_eq!(BindingPath::prepend(&prepend_path, &to_path), Err(()));
}

#[test]
fn should_not_prepend_if_to_path_invalid() {
    let prepend_path = BindingPath::with_segments(vec![BindingPathSegment::AbstractIndex]);
    let to_path = BindingPath(vec![]);
    assert_eq!(BindingPath::prepend(&prepend_path, &to_path), Err(()));
}

#[test]
fn should_prepend_valid_paths() {
    let prepend_path = BindingPath::with_segments(vec![BindingPathSegment::Index(1)]);

    let to_path = BindingPath::with_segments(vec![
        BindingPathSegment::Field("test".to_string()),
        BindingPathSegment::AbstractIndex,
    ]);

    let new_path = BindingPath::with_segments(vec![
        BindingPathSegment::Index(1),
        BindingPathSegment::Field("test".to_string()),
        BindingPathSegment::AbstractIndex,
    ]);

    assert_eq!(BindingPath::prepend(&prepend_path, &to_path), Ok(new_path));
}

// ==================================================================
// PREPEND END
// ==================================================================
