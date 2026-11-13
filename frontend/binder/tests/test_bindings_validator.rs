use elise_frontend_binder::bindings_validator::validate_bindings;
use elise_ir_bindings::{
    binding_path::{BindingPath, BindingPathSegment},
    data_bindings::{
        DataBinding, DataBindingDesc, DataBindingDescDict, DataBindingDescList, DataBindingDescPrim,
    },
    type_bindings::{TypeBinding, TypeBindingDataType, TypeBindingDesc},
};
use elise_shared_errors::errors_bindings_validator::BindingsValidatorErr;
use elise_test_utils::test_utils;

mod common;

#[test]
fn should_succeed_for_empty_bindings() {
    let data_bindings = DataBinding::new();
    let type_bindings = TypeBinding::new();
    assert_eq!(validate_bindings(&data_bindings, &type_bindings), Ok(()));
}

#[test]
fn should_succeed_bindings_with_root_segment_only() {
    let mut data_bindings = DataBinding::new();
    data_bindings.map.insert(
        BindingPath::new(),
        DataBindingDesc::Int(DataBindingDescPrim {
            value: "1".to_string(),
            span: test_utils::span(),
        }),
    );

    let mut type_bindings = TypeBinding::new();
    type_bindings.map.insert(
        BindingPath::new(),
        TypeBindingDesc {
            dtype: TypeBindingDataType::Int,
            span: test_utils::span(),
        },
    );

    assert_eq!(validate_bindings(&data_bindings, &type_bindings), Ok(()));
}

#[test]
fn should_succeed_matching_list_indexes() {
    let mut data_bindings = DataBinding::new();
    data_bindings.map.insert(
        BindingPath::with_segments(vec![BindingPathSegment::Index(0)]),
        DataBindingDesc::Int(DataBindingDescPrim {
            value: "1".to_string(),
            span: test_utils::span(),
        }),
    );

    let mut type_bindings = TypeBinding::new();
    type_bindings.map.insert(
        BindingPath::with_segments(vec![BindingPathSegment::AbstractIndex]),
        TypeBindingDesc {
            dtype: TypeBindingDataType::Int,
            span: test_utils::span(),
        },
    );

    assert_eq!(validate_bindings(&data_bindings, &type_bindings), Ok(()));
}

#[test]
fn should_fail_if_type_missing() {
    let mut data_bindings = DataBinding::new();
    data_bindings.map.insert(
        BindingPath::with_segments(vec![BindingPathSegment::Field("name".to_string())]),
        DataBindingDesc::Str(DataBindingDescPrim {
            value: "John".to_string(),
            span: test_utils::span(),
        }),
    );

    let mut type_bindings = TypeBinding::new();
    type_bindings.map.insert(
        BindingPath::with_segments(vec![BindingPathSegment::AbstractIndex]),
        TypeBindingDesc {
            dtype: TypeBindingDataType::Int,
            span: test_utils::span(),
        },
    );

    assert_eq!(
        validate_bindings(&data_bindings, &type_bindings),
        Err(BindingsValidatorErr::MissingTypeDeclaration)
    );
}

#[test]
fn should_fail_if_type_doesnt_match() {
    let mut data_bindings = DataBinding::new();
    data_bindings.map.insert(
        BindingPath::with_segments(vec![BindingPathSegment::Index(0)]),
        DataBindingDesc::Str(DataBindingDescPrim {
            value: "John".to_string(),
            span: test_utils::span(),
        }),
    );

    let mut type_bindings = TypeBinding::new();
    type_bindings.map.insert(
        BindingPath::with_segments(vec![BindingPathSegment::AbstractIndex]),
        TypeBindingDesc {
            dtype: TypeBindingDataType::Int,
            span: test_utils::span(),
        },
    );

    assert_eq!(
        validate_bindings(&data_bindings, &type_bindings),
        Err(BindingsValidatorErr::TypeMismatch)
    );
}

#[test]
fn should_succeed_matching_long_paths() {
    let mut data_bindings = DataBinding::new();
    data_bindings.map.insert(
        BindingPath::with_segments(vec![
            BindingPathSegment::Index(0),
            BindingPathSegment::Field("emails".to_string()),
            BindingPathSegment::Index(1),
        ]),
        DataBindingDesc::Str(DataBindingDescPrim {
            value: "asd@ad.com".to_string(),
            span: test_utils::span(),
        }),
    );

    let mut type_bindings = TypeBinding::new();
    type_bindings.map.insert(
        BindingPath::with_segments(vec![
            BindingPathSegment::AbstractIndex,
            BindingPathSegment::Field("emails".to_string()),
            BindingPathSegment::AbstractIndex,
        ]),
        TypeBindingDesc {
            dtype: TypeBindingDataType::Str,
            span: test_utils::span(),
        },
    );

    assert_eq!(validate_bindings(&data_bindings, &type_bindings), Ok(()));
}

#[test]
fn should_succeed_matching_int() {
    let mut data_bindings = DataBinding::new();
    data_bindings.map.insert(
        BindingPath::new(),
        DataBindingDesc::Int(DataBindingDescPrim {
            value: "1".to_string(),
            span: test_utils::span(),
        }),
    );

    let mut type_bindings = TypeBinding::new();
    type_bindings.map.insert(
        BindingPath::new(),
        TypeBindingDesc {
            dtype: TypeBindingDataType::Int,
            span: test_utils::span(),
        },
    );

    assert_eq!(validate_bindings(&data_bindings, &type_bindings), Ok(()));
}

#[test]
fn should_succeed_matching_str() {
    let mut data_bindings = DataBinding::new();
    data_bindings.map.insert(
        BindingPath::new(),
        DataBindingDesc::Str(DataBindingDescPrim {
            value: "some".to_string(),
            span: test_utils::span(),
        }),
    );

    let mut type_bindings = TypeBinding::new();
    type_bindings.map.insert(
        BindingPath::new(),
        TypeBindingDesc {
            dtype: TypeBindingDataType::Str,
            span: test_utils::span(),
        },
    );

    assert_eq!(validate_bindings(&data_bindings, &type_bindings), Ok(()));
}

#[test]
fn should_succeed_matching_record() {
    let mut data_bindings = DataBinding::new();
    data_bindings.map.insert(
        BindingPath::new(),
        DataBindingDesc::Dict(DataBindingDescDict {
            span: test_utils::span(),
        }),
    );

    let mut type_bindings = TypeBinding::new();
    type_bindings.map.insert(
        BindingPath::new(),
        TypeBindingDesc {
            dtype: TypeBindingDataType::Record,
            span: test_utils::span(),
        },
    );

    assert_eq!(validate_bindings(&data_bindings, &type_bindings), Ok(()));
}

#[test]
fn should_succeed_matching_list() {
    let mut data_bindings = DataBinding::new();
    data_bindings.map.insert(
        BindingPath::new(),
        DataBindingDesc::List(DataBindingDescList {
            size: Some(2),
            span: test_utils::span(),
        }),
    );

    let mut type_bindings = TypeBinding::new();
    type_bindings.map.insert(
        BindingPath::new(),
        TypeBindingDesc {
            dtype: TypeBindingDataType::List,
            span: test_utils::span(),
        },
    );

    assert_eq!(validate_bindings(&data_bindings, &type_bindings), Ok(()));
}
