use elise_data::data_validator::validate_data;
use elise_shared::{
    shared_errors::errors_data_validator::DataValidatorErr, shared_node_names::NodeName,
};
use elise_test_utils::test_utils;

mod common;

// ==================================================================
//
// CSV START
//
// ==================================================================

#[test]
fn should_return_err_if_data_has_no_type_def() {
    let csv_bindings = test_utils::csv::bind(&vec![&vec!["23", "John", "false"]]);
    let schema_bindings = test_utils::bind_schema(&format!(
        r##"
        .schema(
          .list(
            .dict(
              "{}" .int()
              "{}" .string())))
        "##,
        test_utils::csv::build_header(0),
        test_utils::csv::build_header(1)
    ));
    let result = validate_data(&csv_bindings, &schema_bindings);
    assert!(matches!(
        result,
        Err(DataValidatorErr::DataMissingTypeDef { .. })
    ));
}

#[test]
fn should_return_err_if_data_has_diff_type() {
    let csv_bindings = test_utils::csv::bind(&vec![&vec!["23", "John"]]);
    let schema_bindings = test_utils::bind_schema(&format!(
        r##"
        .schema(
          .list(
            .dict(
              "{}" .string()
              "{}" .string())))
        "##,
        test_utils::csv::build_header(0),
        test_utils::csv::build_header(1),
    ));
    let result = validate_data(&csv_bindings, &schema_bindings);
    assert!(matches!(
        result,
        Err(DataValidatorErr::DataTypeMismatch {
            expected: NodeName::STRING,
            found: NodeName::INT,
            ..
        })
    ));
}

#[test]
fn should_return_err_if_data_is_not_nullable_but_got_null() {
    let csv_bindings = test_utils::csv::bind(&vec![&vec!["23", "null"]]);
    let schema_bindings = test_utils::bind_schema(&format!(
        r##"
        .schema(
          .list(
            .dict(
              "{}" .int()
              "{}" .string())))
        "##,
        test_utils::csv::build_header(0),
        test_utils::csv::build_header(1),
    ));
    let result = validate_data(&csv_bindings, &schema_bindings);
    assert!(matches!(
        result,
        Err(DataValidatorErr::DataTypeMismatch {
            expected: NodeName::STRING,
            found: NodeName::NULL,
            ..
        })
    ));
}

#[test]
fn should_return_err_if_type_non_optional_and_data_missing() {
    let csv_bindings = test_utils::csv::bind(&vec![&vec!["23"]]);
    let schema_bindings = test_utils::bind_schema(&format!(
        r##"
        .schema(
          .list(
            .dict(
              "{}" .int()
              "{}" .string())))
        "##,
        test_utils::csv::build_header(0),
        test_utils::csv::build_header(1),
    ));
    let result = validate_data(&csv_bindings, &schema_bindings);
    assert!(matches!(result, Err(DataValidatorErr::DataMissing { .. })));
}

#[test]
fn should_successfully_validate_matching_types() {
    let csv_bindings = test_utils::csv::bind(&vec![&vec!["23", "John", "34.3", "false"]]);
    let schema_bindings = test_utils::bind_schema(&format!(
        r##"
        .schema(
          .list(
            .dict(
              "{}" .int()
              "{}" .string()
              "{}" .float()
              "{}" .bool())))
        "##,
        test_utils::csv::build_header(0),
        test_utils::csv::build_header(1),
        test_utils::csv::build_header(2),
        test_utils::csv::build_header(3),
    ));
    let result = validate_data(&csv_bindings, &schema_bindings);
    assert_eq!(result, Ok(()));
}

#[test]
fn should_successfully_validate_nullable() {
    let csv_bindings = test_utils::csv::bind(&vec![&vec!["23", "null"]]);
    let schema_bindings = test_utils::bind_schema(&format!(
        r##"
        .schema(
          .list(
            .dict(
              "{}" .int()
              "{}" .nullable(.string()))))
        "##,
        test_utils::csv::build_header(0),
        test_utils::csv::build_header(1),
    ));
    let result = validate_data(&csv_bindings, &schema_bindings);
    assert_eq!(result, Ok(()));
}

#[test]
fn should_successfully_validate_optional() {
    let csv_bindings = test_utils::csv::bind(&vec![&vec!["23"]]);
    let schema_bindings = test_utils::bind_schema(&format!(
        r##"
        .schema(
          .list(
            .dict(
              "{}" .int()
              "{}" .optional(.string()))))
        "##,
        test_utils::csv::build_header(0),
        test_utils::csv::build_header(1),
    ));
    let result = validate_data(&csv_bindings, &schema_bindings);
    assert_eq!(result, Ok(()));
}

// ==================================================================
//
// CSV END
//
// ==================================================================
