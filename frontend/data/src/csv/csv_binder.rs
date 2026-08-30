use std::collections::HashMap;

use elise_shared::shared_errors::errors_csv_binder::CsvBinderErr;

use crate::{
    binder::{BinderDataDescriptor, BinderDataType, DataBinder, DataBindingTable},
    csv::csv_parser::{CsvRow, ParserDataType},
    resolution_path::{ResolutionPath, ResolutionPathSegment},
};

type Rows = Vec<CsvRow>;

pub struct CsvDataBinder<'a> {
    pub rows: &'a Rows,
}

impl<'a> CsvDataBinder<'a> {
    fn map_data_type(ty: &ParserDataType) -> BinderDataType {
        match ty {
            ParserDataType::Int => BinderDataType::Int,
            ParserDataType::Float => BinderDataType::Float,
            ParserDataType::String => BinderDataType::String,
            ParserDataType::Bool => BinderDataType::Bool,
            ParserDataType::Null => BinderDataType::Null,
        }
    }
}

impl<'a> DataBinder<'a, Rows, CsvBinderErr> for CsvDataBinder<'a> {
    fn new(rows: &'a Rows) -> Self {
        CsvDataBinder { rows }
    }

    fn bind(&self) -> Result<DataBindingTable, CsvBinderErr> {
        let mut table: HashMap<ResolutionPath, BinderDataDescriptor> = HashMap::new();

        for row in self.rows {
            for col in &row.cols {
                table.insert(
                    ResolutionPath::with_segments(vec![
                        ResolutionPathSegment::AbstractIndex,
                        ResolutionPathSegment::Field(col.name.clone()),
                    ]),
                    BinderDataDescriptor {
                        ty: Self::map_data_type(&col.ty),
                        value: col.value.clone(),
                    },
                );
            }
        }

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
