use std::collections::HashMap;

use elise_shared::shared_errors::errors_csv_binder::CsvBinderErr;

use crate::{
    binder::{DataBinder, DataBindingTable, DataDescriptor},
    csv::csv_parser::CsvRow,
    resolution_path::ResolutionPath,
};

type Rows = Vec<CsvRow>;

pub struct CsvDataBinder {
    pub rows: Rows,
}

impl DataBinder<Rows, CsvBinderErr> for CsvDataBinder {
    fn new(rows: Rows) -> Self {
        CsvDataBinder { rows }
    }

    fn bind(&self) -> Result<DataBindingTable, CsvBinderErr> {
        let table: HashMap<ResolutionPath, DataDescriptor> = HashMap::new();

        for (_row_idx, _row) in self.rows.iter().enumerate() {}

        if table.is_empty() {
            return Err(CsvBinderErr::NoData);
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
    // TODO
}

// ==================================================================
//
//  TESTS END
//
// ==================================================================
