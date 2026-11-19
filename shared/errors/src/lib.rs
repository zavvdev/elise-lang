pub mod errors_bindings_validator;
pub mod errors_csv_data_parser;
pub mod errors_data_binder;
pub mod errors_parser;
pub mod errors_semanalyzer;

use errors_bindings_validator::BindingsValidatorErr;
use errors_csv_data_parser::CsvDataParserErr;
use errors_data_binder::DataBinderErr;
use errors_parser::ParserErr;
use errors_semanalyzer::SemanalyzerErr;

#[derive(Debug, PartialEq)]
pub enum LangErr {
    ParserSource(ParserErr),
    ParserSchema(ParserErr),
    SemanalyzerSource(SemanalyzerErr),
    SemanalyzerSchema(SemanalyzerErr),
    CsvDataParser(CsvDataParserErr),
    BindingsValidator(BindingsValidatorErr),
    DataBinder(DataBinderErr),
}
