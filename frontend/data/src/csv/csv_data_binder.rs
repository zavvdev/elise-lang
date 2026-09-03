use std::collections::HashMap;

use elise_shared::shared_errors::errors_csv_data_binder::CsvDataBinderErr;

use crate::{
    csv::csv_data_parser::{CsvDataParserDataType, CsvDataRow},
    data_binder::{DataBinder, DataBinderDataDescriptor, DataBinderDataType, DataBindings},
    resolution_path::{ResolutionPath, ResolutionPathSegment},
};

type Rows = Vec<CsvDataRow>;

#[derive(Debug, PartialEq)]
pub struct CsvDataBinder<'a> {
    pub rows: &'a Rows,
}

impl<'a> CsvDataBinder<'a> {
    fn map_data_type(ty: &CsvDataParserDataType) -> DataBinderDataType {
        match ty {
            CsvDataParserDataType::Int => DataBinderDataType::Int,
            CsvDataParserDataType::Float => DataBinderDataType::Float,
            CsvDataParserDataType::String => DataBinderDataType::String,
            CsvDataParserDataType::Bool => DataBinderDataType::Bool,
            CsvDataParserDataType::Null => DataBinderDataType::Null,
        }
    }
}

impl<'a> DataBinder<'a, Rows, CsvDataBinderErr> for CsvDataBinder<'a> {
    fn new(rows: &'a Rows) -> Self {
        Self { rows }
    }

    fn bind(&self) -> Result<DataBindings, CsvDataBinderErr> {
        let mut bindings: HashMap<ResolutionPath, DataBinderDataDescriptor> = HashMap::new();

        for (idx, row) in self.rows.iter().enumerate() {
            for col in &row.cols {
                let key = ResolutionPath::with_segments(vec![
                    ResolutionPathSegment::Index(idx),
                    ResolutionPathSegment::Field(col.name.clone()),
                ]);
                let value = DataBinderDataDescriptor {
                    ty: Self::map_data_type(&col.ty),
                    value: col.value.clone(),
                    pos: col.pos.clone(),
                    type_resolution_path: ResolutionPath::with_segments(vec![
                        ResolutionPathSegment::AbstractIndex,
                        ResolutionPathSegment::Field(col.name.clone()),
                    ]),
                };
                bindings.insert(key, value);
            }
        }

        if bindings.is_empty() {
            return Err(CsvDataBinderErr::NoData);
        }

        Ok(DataBindings { bindings })
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
    use elise_shared::shared_types::{Keyword, Pos};

    use crate::csv::csv_data_binder::CsvDataBinder;
    use crate::csv::csv_data_parser::{CsvDataCol, CsvDataParserDataType, CsvDataRow};
    use crate::data_binder::{
        DataBinder, DataBinderDataDescriptor, DataBinderDataType, DataBindings,
    };
    use crate::resolution_path::{ResolutionPath, ResolutionPathSegment::*};

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
                    CsvDataParserDataType::String,
                    DataBinderDataType::String,
                ),
                ("23", CsvDataParserDataType::Int, DataBinderDataType::Int),
                (
                    "2.3",
                    CsvDataParserDataType::Float,
                    DataBinderDataType::Float,
                ),
                (
                    Keyword::TRUE,
                    CsvDataParserDataType::Bool,
                    DataBinderDataType::Bool,
                ),
                (
                    Keyword::NULL,
                    CsvDataParserDataType::Null,
                    DataBinderDataType::Null,
                ),
            ],
            vec![
                (
                    "Jane",
                    CsvDataParserDataType::String,
                    DataBinderDataType::String,
                ),
                ("24", CsvDataParserDataType::Int, DataBinderDataType::Int),
                (
                    "4.3",
                    CsvDataParserDataType::Float,
                    DataBinderDataType::Float,
                ),
                (
                    Keyword::FALSE,
                    CsvDataParserDataType::Bool,
                    DataBinderDataType::Bool,
                ),
                (
                    Keyword::NULL,
                    CsvDataParserDataType::Null,
                    DataBinderDataType::Null,
                ),
            ],
        ];

        let mut parsed_data = vec![];

        for (row_idx, row) in rows.iter().enumerate() {
            let mut final_cols = vec![];
            for (col_idx, col) in row.iter().enumerate() {
                final_cols.push(CsvDataCol {
                    name: cols[col_idx].to_string(),
                    ty: col.1.clone(),
                    value: col.0.to_string(),
                    pos: Pos {
                        row: row_idx,
                        col: col_idx,
                    },
                });
            }

            parsed_data.push(CsvDataRow { cols: final_cols });
        }

        let mut bindings: HashMap<ResolutionPath, DataBinderDataDescriptor> = HashMap::new();

        for (row_idx, row) in rows.iter().enumerate() {
            for (col_idx, col) in row.iter().enumerate() {
                bindings.insert(
                    ResolutionPath::with_segments(vec![
                        Index(row_idx),
                        Field(cols[col_idx].to_string()),
                    ]),
                    DataBinderDataDescriptor {
                        ty: col.2.clone(),
                        value: col.0.to_string(),
                        pos: Pos {
                            row: row_idx,
                            col: col_idx,
                        },
                        type_resolution_path: ResolutionPath::with_segments(vec![
                            AbstractIndex,
                            Field(cols[col_idx].to_string()),
                        ]),
                    },
                );
            }
        }

        assert_eq!(
            CsvDataBinder::new(&parsed_data).bind(),
            Ok(DataBindings { bindings })
        );
    }
}

// ==================================================================
//
//  TESTS END
//
// ==================================================================
