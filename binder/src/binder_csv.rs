use std::collections::HashMap;

use elise_csv::{
    config::{CSV_BOOL_FALSE_TOKENS_LOWER, CSV_BOOL_TRUE_TOKENS_LOWER},
    parser::CsvRow,
    schema_resolver::CsvResolvedSchema,
};
use elise_errors::errors_csv_binder::{
    CsvBinderErr::{self, *},
    PosInfo, TypeMismatchInfo,
};
use elise_types::DataSourceFieldType;

use crate::{
    binding_table::{Binder, DataBindingTable, DataDescriptor, Path, PathSegment::*},
    config::{BOOL_FALSE_COERCED, BOOL_TRUE_COERCED},
};

pub struct CsvDataBinder {
    pub rows: Vec<CsvRow>,
    pub schema: CsvResolvedSchema,
}

type Rows = Vec<CsvRow>;
type Schema = CsvResolvedSchema;

impl CsvDataBinder {
    fn is_bool(value: &str) -> bool {
        let lower_value = value.to_lowercase();
        CSV_BOOL_TRUE_TOKENS_LOWER.contains(&lower_value.as_str())
            || CSV_BOOL_FALSE_TOKENS_LOWER.contains(&lower_value.as_str())
    }

    fn coerce_bool(value: &str) -> String {
        let lower_value = value.to_lowercase();
        if CSV_BOOL_TRUE_TOKENS_LOWER.contains(&lower_value.as_str()) {
            return BOOL_TRUE_COERCED.to_string();
        }
        if CSV_BOOL_FALSE_TOKENS_LOWER.contains(&lower_value.as_str()) {
            return BOOL_FALSE_COERCED.to_string();
        }
        value.to_string()
    }

    fn coerce(value: &str) -> String {
        if Self::is_bool(value) {
            return Self::coerce_bool(value);
        }
        value.to_string()
    }
}

impl Binder<Rows, Schema, CsvBinderErr> for CsvDataBinder {
    fn new(rows: Rows, schema: Schema) -> Self {
        CsvDataBinder { rows, schema }
    }

    fn bind(&self) -> Result<DataBindingTable, CsvBinderErr> {
        let mut table: HashMap<Path, DataDescriptor> = HashMap::new();

        for (row_idx, row) in self.rows.iter().enumerate() {
            // We can check against the first record only since if
            // csv row length consistency is being handled by CsvParser.
            if row_idx == 0 && row.cols.len() != self.schema.row.len() {
                let col = row.cols.get(row_idx).unwrap();
                return Err(RowLenMismatch(PosInfo {
                    row: col.row,
                    col: col.col,
                }));
            }

            for (col_idx, col) in row.cols.iter().enumerate() {
                let col_schema = self.schema.row.get(col_idx).unwrap();
                let is_opt = col_schema.opt && col.ty == DataSourceFieldType::Empty;

                if col.ty == col_schema.ty || is_opt {
                    let path = vec![Index(row_idx), Field(col_schema.name.clone())];

                    table.insert(
                        path,
                        DataDescriptor {
                            ty: col.ty.clone(),
                            value: Self::coerce(&col.value),
                        },
                    );
                    continue;
                }

                return Err(TypeMismatch(TypeMismatchInfo {
                    pos: PosInfo {
                        row: col.row,
                        col: col.col,
                    },
                    expected: col_schema.ty.clone(),
                    got: col.ty.clone(),
                }));
            }
        }

        if table.is_empty() {
            return Err(NoData);
        }

        Ok(DataBindingTable { table })
    }
}

// ==================================================================
//
//  TESTS START
//
// ==================================================================

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use elise_csv::parser::{CsvCol, CsvRow};
    use elise_csv::schema_resolver::{CsvColDescriptor, CsvResolvedSchema};
    use elise_errors::errors_csv_binder::PosInfo;
    use elise_errors::errors_csv_binder::{CsvBinderErr::*, TypeMismatchInfo};
    use elise_types::DataSourceFieldType;

    use crate::binder_csv::CsvDataBinder;
    use crate::binding_table::PathSegment::{Field, Index};
    use crate::binding_table::{Binder, DataBindingTable, DataDescriptor};
    use crate::config::{BOOL_FALSE_COERCED, BOOL_TRUE_COERCED};

    #[test]
    fn bind_should_return_error_if_schema_row_len_bigger_than_csv_row_len() {
        let data = vec![CsvRow {
            cols: vec![CsvCol {
                ty: DataSourceFieldType::Number,
                value: "32".to_string(),
                row: 0,
                col: 0,
            }],
        }];
        let schema = CsvResolvedSchema {
            row: vec![
                CsvColDescriptor {
                    ty: DataSourceFieldType::Number,
                    name: "age".to_string(),
                    opt: false,
                },
                CsvColDescriptor {
                    ty: DataSourceFieldType::String,
                    name: "name".to_string(),
                    opt: false,
                },
            ],
        };
        let binder = CsvDataBinder::new(data, schema);
        assert_eq!(
            binder.bind(),
            Err(RowLenMismatch(PosInfo { row: 0, col: 0 }))
        )
    }

    #[test]
    fn bind_should_return_error_if_csv_row_len_bigger_than_schema_row_len() {
        let data = vec![CsvRow {
            cols: vec![
                CsvCol {
                    ty: DataSourceFieldType::Number,
                    value: "32".to_string(),
                    row: 0,
                    col: 0,
                },
                CsvCol {
                    ty: DataSourceFieldType::Number,
                    value: "33".to_string(),
                    row: 0,
                    col: 1,
                },
            ],
        }];
        let schema = CsvResolvedSchema {
            row: vec![CsvColDescriptor {
                ty: DataSourceFieldType::Number,
                name: "age".to_string(),
                opt: false,
            }],
        };
        let binder = CsvDataBinder::new(data, schema);
        assert_eq!(
            binder.bind(),
            Err(RowLenMismatch(PosInfo { row: 0, col: 0 }))
        )
    }

    #[test]
    fn bind_should_return_error_if_type_mismatch_and_opt_false() {
        let data = vec![CsvRow {
            cols: vec![CsvCol {
                ty: DataSourceFieldType::Number,
                value: "32".to_string(),
                row: 0,
                col: 0,
            }],
        }];
        let schema = CsvResolvedSchema {
            row: vec![CsvColDescriptor {
                ty: DataSourceFieldType::String,
                name: "name".to_string(),
                opt: false,
            }],
        };
        let binder = CsvDataBinder::new(data, schema);
        assert_eq!(
            binder.bind(),
            Err(TypeMismatch(TypeMismatchInfo {
                pos: PosInfo { row: 0, col: 0 },
                expected: DataSourceFieldType::String,
                got: DataSourceFieldType::Number,
            }))
        )
    }

    #[test]
    fn bind_should_return_error_if_type_mismatch_opt_true_and_not_empty() {
        let data = vec![CsvRow {
            cols: vec![CsvCol {
                ty: DataSourceFieldType::Number,
                value: "32".to_string(),
                row: 0,
                col: 0,
            }],
        }];
        let schema = CsvResolvedSchema {
            row: vec![CsvColDescriptor {
                ty: DataSourceFieldType::String,
                name: "name".to_string(),
                opt: true,
            }],
        };
        let binder = CsvDataBinder::new(data, schema);
        assert_eq!(
            binder.bind(),
            Err(TypeMismatch(TypeMismatchInfo {
                pos: PosInfo { row: 0, col: 0 },
                expected: DataSourceFieldType::String,
                got: DataSourceFieldType::Number,
            }))
        )
    }

    #[test]
    fn bind_should_bind_if_type_match_and_opt_false() {
        let data = vec![CsvRow {
            cols: vec![CsvCol {
                ty: DataSourceFieldType::Number,
                value: "32".to_string(),
                row: 0,
                col: 0,
            }],
        }];
        let schema = CsvResolvedSchema {
            row: vec![CsvColDescriptor {
                ty: DataSourceFieldType::Number,
                name: "age".to_string(),
                opt: false,
            }],
        };
        let binder = CsvDataBinder::new(data, schema);
        let mut table = HashMap::new();
        let path = vec![Index(0), Field("age".to_string())];

        table.insert(
            path,
            DataDescriptor {
                ty: DataSourceFieldType::Number,
                value: "32".to_string(),
            },
        );

        let result = DataBindingTable { table };

        assert_eq!(binder.bind(), Ok(result));
    }

    #[test]
    fn bind_should_bind_if_type_match_and_opt_true() {
        let data = vec![CsvRow {
            cols: vec![CsvCol {
                ty: DataSourceFieldType::Number,
                value: "32".to_string(),
                row: 0,
                col: 0,
            }],
        }];
        let schema = CsvResolvedSchema {
            row: vec![CsvColDescriptor {
                ty: DataSourceFieldType::Number,
                name: "age".to_string(),
                opt: true,
            }],
        };
        let binder = CsvDataBinder::new(data, schema);
        let mut table = HashMap::new();
        let path = vec![Index(0), Field("age".to_string())];

        table.insert(
            path,
            DataDescriptor {
                ty: DataSourceFieldType::Number,
                value: "32".to_string(),
            },
        );

        let result = DataBindingTable { table };

        assert_eq!(binder.bind(), Ok(result));
    }

    #[test]
    fn bind_should_bind_if_opt_true_and_empty() {
        let data = vec![CsvRow {
            cols: vec![CsvCol {
                ty: DataSourceFieldType::Empty,
                value: "".to_string(),
                row: 0,
                col: 0,
            }],
        }];
        let schema = CsvResolvedSchema {
            row: vec![CsvColDescriptor {
                ty: DataSourceFieldType::Number,
                name: "age".to_string(),
                opt: true,
            }],
        };
        let binder = CsvDataBinder::new(data, schema);
        let mut table = HashMap::new();
        let path = vec![Index(0), Field("age".to_string())];

        table.insert(
            path,
            DataDescriptor {
                ty: DataSourceFieldType::Empty,
                value: "".to_string(),
            },
        );

        let result = DataBindingTable { table };

        assert_eq!(binder.bind(), Ok(result));
    }

    #[test]
    fn bind_should_coerce_bool_true() {
        let true_values = vec![
            "True", "true", "TRUE", "Yes", "yes", "YES", "on", "On", "ON", "y", "Y",
        ];

        for bool_true in true_values {
            let data = vec![CsvRow {
                cols: vec![CsvCol {
                    ty: DataSourceFieldType::Bool,
                    value: bool_true.to_string(),
                    row: 0,
                    col: 0,
                }],
            }];
            let schema = CsvResolvedSchema {
                row: vec![CsvColDescriptor {
                    ty: DataSourceFieldType::Bool,
                    name: "test".to_string(),
                    opt: false,
                }],
            };
            let binder = CsvDataBinder::new(data, schema);
            let mut table = HashMap::new();
            let path = vec![Index(0), Field("test".to_string())];

            table.insert(
                path,
                DataDescriptor {
                    ty: DataSourceFieldType::Bool,
                    value: BOOL_TRUE_COERCED.to_string(),
                },
            );

            let result = DataBindingTable { table };

            assert_eq!(binder.bind(), Ok(result));
        }
    }

    #[test]
    fn bind_should_coerce_bool_false() {
        let false_values = vec![
            "False", "false", "FALSE", "No", "no", "NO", "off", "Off", "OFF", "n", "N",
        ];

        for bool_false in false_values {
            let data = vec![CsvRow {
                cols: vec![CsvCol {
                    ty: DataSourceFieldType::Bool,
                    value: bool_false.to_string(),
                    row: 0,
                    col: 0,
                }],
            }];
            let schema = CsvResolvedSchema {
                row: vec![CsvColDescriptor {
                    ty: DataSourceFieldType::Bool,
                    name: "test".to_string(),
                    opt: false,
                }],
            };
            let binder = CsvDataBinder::new(data, schema);
            let mut table = HashMap::new();
            let path = vec![Index(0), Field("test".to_string())];

            table.insert(
                path,
                DataDescriptor {
                    ty: DataSourceFieldType::Bool,
                    value: BOOL_FALSE_COERCED.to_string(),
                },
            );

            let result = DataBindingTable { table };

            assert_eq!(binder.bind(), Ok(result));
        }
    }
}

// ==================================================================
//
//  TESTS END
//
// ==================================================================
