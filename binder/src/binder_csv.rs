use std::collections::HashMap;

use elise_csv::{parser::CsvRow, schema_resolver::CsvResolvedSchema};
use elise_errors::errors_binder::{
    BinderErr::{self, *},
    PosInfo, TypeMismatchInfo,
};

use crate::binding_table::{Binder, DataBindingTable, DataDescriptor, Path, PathSegment::*};

pub struct CsvDataBinder {
    pub rows: Vec<CsvRow>,
    pub schema: CsvResolvedSchema,
}

type Rows = Vec<CsvRow>;
type Schema = CsvResolvedSchema;

impl Binder<Rows, Schema> for CsvDataBinder {
    fn new(rows: Rows, schema: Schema) -> Self {
        CsvDataBinder { rows, schema }
    }

    // TODO: Handle optional case
    fn bind(&self) -> Result<DataBindingTable, BinderErr> {
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

                if col.ty != col_schema.ty {
                    return Err(TypeMismatch(TypeMismatchInfo {
                        pos: PosInfo {
                            row: col.row,
                            col: col.col,
                        },
                        expected: col_schema.ty.clone(),
                        got: col.ty.clone(),
                    }));
                }

                let path = vec![Index(col_idx), Field(col_schema.name.clone())];

                table.insert(
                    path,
                    DataDescriptor {
                        ty: col_schema.ty.clone(),
                        value: col.value.clone(),
                    },
                );
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
    use elise_csv::parser::{CsvCol, CsvRow};
    use elise_csv::schema_resolver::{CsvColDescriptor, CsvResolvedSchema};
    use elise_errors::errors_binder::BinderErr::RowLenMismatch;
    use elise_errors::errors_binder::PosInfo;
    use elise_types::DataSourceFieldType;

    use crate::binder_csv::CsvDataBinder;
    use crate::binding_table::Binder;

    #[test]
    fn bind_should_return_error_if_schema_row_len_mismatch() {
        let data = vec![CsvRow {
            cols: vec![CsvCol {
                ty: DataSourceFieldType::Number,
                value: "32".to_string(),
                row: 1,
                col: 1,
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
            Err(RowLenMismatch(PosInfo { row: 1, col: 1 }))
        )
    }
}

// ==================================================================
//
//  TESTS END
//
// ==================================================================
