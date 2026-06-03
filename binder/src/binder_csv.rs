use std::collections::HashMap;

use elise_csv::{parser::CsvRow, schema_resolver::CsvResolvedSchema};
use elise_errors::errors_binder::{BinderErr, BinderErr::*, BinderErrInfo};

use crate::binding_table::{Binder, DataBindingTable, DataDescriptor, Path};

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

        for (index, row) in self.rows.iter().enumerate() {
            if index == 0 && row.cols.len() != self.schema.row.len() {
                let col = row.cols.get(index).unwrap();
                return Err(RowLenMismatch(BinderErrInfo {
                    row: col.row,
                    col: col.col,
                }));
            }
        }

        if table.is_empty() {
            return Err(NoData);
        }

        Ok(DataBindingTable { table })
    }
}
