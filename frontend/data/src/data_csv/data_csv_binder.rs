use std::collections::HashMap;

use elise_shared::shared_errors::errors_csv_binder::{
    CsvBinderErr::{self, *},
    PosInfo,
};

use crate::{
    data_binder::{DataBinder, DataBindingTable, DataDescriptor, Path, PathSegment::*},
    data_csv::{data_csv_parser::CsvRow, data_csv_schema_resolver::CsvResolvedSchema},
    data_types::DataType,
};

type Rows = Vec<CsvRow>;
type Schema = CsvResolvedSchema;

pub struct CsvDataBinder {
    pub rows: Rows,
    pub schema: Schema,
}

impl DataBinder<Rows, Schema, CsvBinderErr> for CsvDataBinder {
    fn new(rows: Rows, schema: Schema) -> Self {
        CsvDataBinder { rows, schema }
    }

    fn bind(&self) -> Result<DataBindingTable, CsvBinderErr> {
        let mut table: HashMap<Path, DataDescriptor> = HashMap::new();

        for (row_idx, row) in self.rows.iter().enumerate() {
            // We can check against the first record only since
            // csv row length consistency is being handled by CsvParser.
            if row_idx == 0 && row.cols.len() != self.schema.resolved_schema.len() {
                let col = row.cols.get(row_idx).unwrap();
                return Err(RowLenMismatch(PosInfo {
                    row: col.row,
                    col: col.col,
                }));
            }

            for col in row.cols.iter() {
                // Get type information about the column from resolved schema.
                let col_schema = self.schema.resolved_schema.get(&col.name).ok_or_else(|| {
                    CsvBinderErr::MissingTypeDefinition {
                        pos: PosInfo {
                            row: col.row,
                            col: col.col,
                        },
                        col: col.name.clone(),
                    }
                })?;

                let is_opt = col_schema.opt && col.ty == DataType::Null;

                if col.ty == col_schema.ty || is_opt {
                    let path = vec![Index(row_idx), Field(col.name.clone())];

                    table.insert(
                        path,
                        DataDescriptor {
                            ty: col.ty.clone(),
                            value: col.value.to_string(),
                        },
                    );
                    continue;
                }

                return Err(TypeMismatch {
                    pos: PosInfo {
                        row: col.row,
                        col: col.col,
                    },
                    expected: col_schema.ty.as_str(),
                    got: col.ty.as_str(),
                });
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

    use elise_shared::shared_errors::errors_csv_binder::CsvBinderErr::*;
    use elise_shared::shared_errors::errors_csv_binder::PosInfo;
    use elise_shared::shared_node_names::NodeName;

    use crate::data_binder::DataBinder;
    use crate::data_binder::{DataBindingTable, DataDescriptor, PathSegment::*};
    use crate::data_csv::data_csv_binder::CsvDataBinder;
    use crate::data_csv::data_csv_parser::{CsvCol, CsvRow};
    use crate::data_csv::data_csv_schema_resolver::{CsvColDescriptor, CsvResolvedSchema};
    use crate::data_types::DataType;

    #[test]
    fn bind_should_return_error_if_schema_row_len_bigger_than_csv_row_len() {
        let data = vec![CsvRow {
            cols: vec![CsvCol {
                name: "a".to_string(),
                ty: DataType::Int,
                value: "32".to_string(),
                row: 0,
                col: 0,
            }],
        }];

        let mut resolved_schema = HashMap::new();
        resolved_schema.insert(
            "age".to_string(),
            CsvColDescriptor {
                ty: DataType::Int,
                opt: false,
            },
        );
        resolved_schema.insert(
            "name".to_string(),
            CsvColDescriptor {
                ty: DataType::String,
                opt: false,
            },
        );

        let schema = CsvResolvedSchema { resolved_schema };
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
                    name: "a".to_string(),
                    ty: DataType::Int,
                    value: "32".to_string(),
                    row: 0,
                    col: 0,
                },
                CsvCol {
                    name: "b".to_string(),
                    ty: DataType::Int,
                    value: "33".to_string(),
                    row: 0,
                    col: 1,
                },
            ],
        }];

        let mut resolved_schema = HashMap::new();
        resolved_schema.insert(
            "age".to_string(),
            CsvColDescriptor {
                ty: DataType::Int,
                opt: false,
            },
        );

        let schema = CsvResolvedSchema { resolved_schema };
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
                name: "name".to_string(),
                ty: DataType::Int,
                value: "32".to_string(),
                row: 0,
                col: 0,
            }],
        }];

        let mut resolved_schema = HashMap::new();
        resolved_schema.insert(
            "name".to_string(),
            CsvColDescriptor {
                ty: DataType::String,
                opt: false,
            },
        );

        let schema = CsvResolvedSchema { resolved_schema };
        let binder = CsvDataBinder::new(data, schema);

        assert_eq!(
            binder.bind(),
            Err(TypeMismatch {
                pos: PosInfo { row: 0, col: 0 },
                expected: NodeName::STRING,
                got: NodeName::INT,
            })
        )
    }

    #[test]
    fn bind_should_return_error_if_typedef_not_found() {
        let data = vec![CsvRow {
            cols: vec![CsvCol {
                name: "a".to_string(),
                ty: DataType::Int,
                value: "32".to_string(),
                row: 0,
                col: 0,
            }],
        }];

        let mut resolved_schema = HashMap::new();
        resolved_schema.insert(
            "name".to_string(),
            CsvColDescriptor {
                ty: DataType::String,
                opt: false,
            },
        );

        let schema = CsvResolvedSchema { resolved_schema };
        let binder = CsvDataBinder::new(data, schema);

        assert_eq!(
            binder.bind(),
            Err(MissingTypeDefinition {
                pos: PosInfo { row: 0, col: 0 },
                col: "a".to_string()
            })
        )
    }

    #[test]
    fn bind_should_return_error_if_type_mismatch_opt_true_and_not_empty() {
        let data = vec![CsvRow {
            cols: vec![CsvCol {
                name: "age".to_string(),
                ty: DataType::Int,
                value: "32".to_string(),
                row: 0,
                col: 0,
            }],
        }];

        let mut resolved_schema = HashMap::new();
        resolved_schema.insert(
            "age".to_string(),
            CsvColDescriptor {
                ty: DataType::String,
                opt: true,
            },
        );

        let schema = CsvResolvedSchema { resolved_schema };
        let binder = CsvDataBinder::new(data, schema);

        assert_eq!(
            binder.bind(),
            Err(TypeMismatch {
                pos: PosInfo { row: 0, col: 0 },
                expected: NodeName::STRING,
                got: NodeName::INT,
            })
        )
    }

    #[test]
    fn bind_should_bind_if_type_match_and_opt_false() {
        let data = vec![CsvRow {
            cols: vec![CsvCol {
                name: "age".to_string(),
                ty: DataType::Int,
                value: "32".to_string(),
                row: 0,
                col: 0,
            }],
        }];

        let mut resolved_schema = HashMap::new();
        resolved_schema.insert(
            "age".to_string(),
            CsvColDescriptor {
                ty: DataType::Int,
                opt: true,
            },
        );

        let schema = CsvResolvedSchema { resolved_schema };
        let binder = CsvDataBinder::new(data, schema);
        let mut table = HashMap::new();
        let path = vec![Index(0), Field("age".to_string())];

        table.insert(
            path,
            DataDescriptor {
                ty: DataType::Int,
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
                name: "age".to_string(),
                ty: DataType::Int,
                value: "32".to_string(),
                row: 0,
                col: 0,
            }],
        }];

        let mut resolved_schema = HashMap::new();
        resolved_schema.insert(
            "age".to_string(),
            CsvColDescriptor {
                ty: DataType::Int,
                opt: true,
            },
        );

        let schema = CsvResolvedSchema { resolved_schema };
        let binder = CsvDataBinder::new(data, schema);
        let mut table = HashMap::new();
        let path = vec![Index(0), Field("age".to_string())];

        table.insert(
            path,
            DataDescriptor {
                ty: DataType::Int,
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
                name: "age".to_string(),
                ty: DataType::Null,
                value: "".to_string(),
                row: 0,
                col: 0,
            }],
        }];

        let mut resolved_schema = HashMap::new();
        resolved_schema.insert(
            "age".to_string(),
            CsvColDescriptor {
                ty: DataType::Int,
                opt: true,
            },
        );

        let schema = CsvResolvedSchema { resolved_schema };
        let binder = CsvDataBinder::new(data, schema);
        let mut table = HashMap::new();
        let path = vec![Index(0), Field("age".to_string())];

        table.insert(
            path,
            DataDescriptor {
                ty: DataType::Null,
                value: "".to_string(),
            },
        );

        let result = DataBindingTable { table };

        assert_eq!(binder.bind(), Ok(result));
    }
}

// ==================================================================
//
//  TESTS END
//
// ==================================================================
