use std::collections::HashMap;

use elise_shared::shared_errors::errors_csv_data_binder::CsvDataBinderErr;

use crate::{
    csv::csv_parser::{CsvParserDataType, CsvRow},
    data_binder::{DataBinder, DataBinderDataDescriptor, DataBinderDataType, DataBindingTable},
    resolution_path::{ResolutionPath, ResolutionPathSegment},
};

type Rows = Vec<CsvRow>;

#[derive(Debug, PartialEq)]
pub struct CsvDataBinder<'a> {
    pub rows: &'a Rows,
}

impl<'a> CsvDataBinder<'a> {
    fn map_data_type(ty: &CsvParserDataType) -> DataBinderDataType {
        match ty {
            CsvParserDataType::Int => DataBinderDataType::Int,
            CsvParserDataType::Float => DataBinderDataType::Float,
            CsvParserDataType::String => DataBinderDataType::String,
            CsvParserDataType::Bool => DataBinderDataType::Bool,
            CsvParserDataType::Null => DataBinderDataType::Null,
        }
    }
}

impl<'a> DataBinder<'a, Rows, CsvDataBinderErr> for CsvDataBinder<'a> {
    fn new(rows: &'a Rows) -> Self {
        Self { rows }
    }

    fn bind(&self) -> Result<DataBindingTable, CsvDataBinderErr> {
        let mut table: HashMap<ResolutionPath, DataBinderDataDescriptor> = HashMap::new();

        for (idx, row) in self.rows.iter().enumerate() {
            for col in &row.cols {
                table.insert(
                    ResolutionPath::with_segments(vec![
                        ResolutionPathSegment::Index(idx),
                        ResolutionPathSegment::Field(col.name.clone()),
                    ]),
                    DataBinderDataDescriptor {
                        ty: Self::map_data_type(&col.ty),
                        value: col.value.clone(),
                    },
                );
            }
        }

        if table.is_empty() {
            return Err(CsvDataBinderErr::NoData);
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

    use elise_shared::shared_errors::errors_csv_data_binder::CsvDataBinderErr;
    use elise_shared::shared_types::Keyword;

    use crate::csv::csv_data_binder::CsvDataBinder;
    use crate::csv::csv_parser::{CsvCol, CsvParserDataType, CsvRow};
    use crate::data_binder::{
        DataBinder, DataBinderDataDescriptor, DataBinderDataType, DataBindingTable,
    };
    use crate::resolution_path::{ResolutionPath, ResolutionPathSegment};

    #[test]
    fn should_return_no_data_err() {
        assert_eq!(
            CsvDataBinder::new(&vec![]).bind(),
            Err(CsvDataBinderErr::NoData)
        );
    }

    #[test]
    fn should_bind_csv_data() {
        let cols = vec!["name", "age", "score", "employed", "address"];

        let rows = vec![
            vec![
                (
                    "John",
                    CsvParserDataType::String,
                    DataBinderDataType::String,
                ),
                ("23", CsvParserDataType::Int, DataBinderDataType::Int),
                ("2.3", CsvParserDataType::Float, DataBinderDataType::Float),
                (
                    Keyword::TRUE,
                    CsvParserDataType::Bool,
                    DataBinderDataType::Bool,
                ),
                (
                    Keyword::NULL,
                    CsvParserDataType::Null,
                    DataBinderDataType::Null,
                ),
            ],
            vec![
                (
                    "Jane",
                    CsvParserDataType::String,
                    DataBinderDataType::String,
                ),
                ("24", CsvParserDataType::Int, DataBinderDataType::Int),
                ("4.3", CsvParserDataType::Float, DataBinderDataType::Float),
                (
                    Keyword::FALSE,
                    CsvParserDataType::Bool,
                    DataBinderDataType::Bool,
                ),
                (
                    Keyword::NULL,
                    CsvParserDataType::Null,
                    DataBinderDataType::Null,
                ),
            ],
        ];

        let mut parsed_data = vec![];

        for (row_idx, row) in rows.iter().enumerate() {
            let mut final_cols = vec![];
            for (col_idx, col) in row.iter().enumerate() {
                final_cols.push(CsvCol {
                    name: cols[col_idx].to_string(),
                    ty: col.1.clone(),
                    value: col.0.to_string(),
                    row: row_idx,
                    col: col_idx,
                });
            }

            parsed_data.push(CsvRow { cols: final_cols });
        }

        let mut table: HashMap<ResolutionPath, DataBinderDataDescriptor> = HashMap::new();

        for (row_idx, row) in rows.iter().enumerate() {
            for (col_idx, col) in row.iter().enumerate() {
                table.insert(
                    ResolutionPath::with_segments(vec![
                        ResolutionPathSegment::Index(row_idx),
                        ResolutionPathSegment::Field(cols[col_idx].to_string()),
                    ]),
                    DataBinderDataDescriptor {
                        ty: col.2.clone(),
                        value: col.0.to_string(),
                    },
                );
            }
        }

        assert_eq!(
            CsvDataBinder::new(&parsed_data).bind(),
            Ok(DataBindingTable { table })
        );
    }
}

// ==================================================================
//
//  TESTS END
//
// ==================================================================
