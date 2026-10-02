pub mod errors_csv_data_binder;
pub mod errors_csv_data_parser;
pub mod errors_data_validator;
pub mod errors_parser;
pub mod errors_preexec;
pub mod errors_semanalyzer;
pub mod errors_type_binder;

use errors_csv_data_binder::CsvDataBinderErr;
use errors_csv_data_parser::CsvDataParserErr;
use errors_parser::ParserErr;
use errors_semanalyzer::SemanalyzerErr;
use errors_type_binder::TypeBinderErr;

use crate::shared_errors::{errors_data_validator::DataValidatorErr, errors_preexec::PreExecErr};

#[derive(Debug, PartialEq)]
pub enum LangErr {
    PreExec(PreExecErr),
    ParserSource(ParserErr),
    ParserSchema(ParserErr),
    TypeBinder(TypeBinderErr),
    Semanalyzer(SemanalyzerErr),
    DataValidator(DataValidatorErr),
    CsvDataParser(CsvDataParserErr),
    CsvDataBinder(CsvDataBinderErr),
}
