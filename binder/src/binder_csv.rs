use elise_csv::{parser::CsvParserRecord, schema_resolver::CsvResolvedSchema};
use elise_errors::errors_binder::BinderErr;

use crate::binding_table::{Binder, DataBindingTable};

pub struct CsvDataBinder {
    pub data: Vec<CsvParserRecord>,
    pub schema: CsvResolvedSchema,
}

type Data = Vec<CsvParserRecord>;
type Schema = CsvResolvedSchema;

impl Binder<Data, Schema> for CsvDataBinder {
    fn new(data: Data, schema: Schema) -> Self {
        CsvDataBinder { data, schema }
    }

    fn bind(&self) -> Result<DataBindingTable, BinderErr> {
        Err(BinderErr::Todo)
    }
}
