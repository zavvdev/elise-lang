mod common;

use elise_frontend_data::adapter::DataAdapter;
use elise_frontend_data::csv::{csv_data_adapter::CsvDataAdapter, csv_data_parser::CsvDataParser};
use elise_test_utils::test_utils;

#[test]
fn should_translate_csv_into_source_code() {
    let row1 = vec!["42", "1.2", "null", "false", "\"hello\""];
    let row2 = vec!["43", "1.3", "null", "true", "\"world\""];
    let csv = test_utils::csv::build(&vec![&row1, &row2]);
    let parsed = CsvDataParser::new(&csv).parse().unwrap();
    let adapted = CsvDataAdapter::translate(&parsed);
    let expected = r#"[{"n0" 42 "n1" 1.2 "n2" null "n3" false "n4" "hello" } {"n0" 43 "n1" 1.3 "n2" null "n3" true "n4" "world" } ]"#;
    assert_eq!(String::from_utf8(adapted).unwrap(), expected);
}
