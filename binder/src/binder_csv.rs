use std::collections::HashMap;

use elise_csv::{parser::CsvRow, schema_resolver::CsvResolvedSchema};
use elise_errors::errors_binder::{BinderErr, BinderErr::*, BinderErrInfo};

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

    fn bind(&self) -> Result<DataBindingTable, BinderErr> {
        let mut table: HashMap<Path, DataDescriptor> = HashMap::new();

        for (row_idx, row) in self.rows.iter().enumerate() {
            if row_idx == 0 && row.cols.len() != self.schema.row.len() {
                let col = row.cols.get(row_idx).unwrap();
                return Err(RowLenMismatch(BinderErrInfo {
                    row: col.row,
                    col: col.col,
                }));
            }

            for (col_idx, col) in row.cols.iter().enumerate() {
                let col_schema = self.schema.row.get(col_idx).unwrap();

                if col.ty != col_schema.ty {
                    // TODO: Add more metadata about mismatched types.
                    return Err(TypeMismatch(BinderErrInfo {
                        row: col.row,
                        col: col.col,
                    }));
                }
                
                // TODO: Fix
                table.insert(
                    (Index(col_idx), Field(col_schema.name)),
                    DataDescriptor {
                        ty: col_schema.ty,
                        value: col.value,
                    },
                )
            }
        }

        if table.is_empty() {
            return Err(NoData);
        }

        Ok(DataBindingTable { table })
    }
}
