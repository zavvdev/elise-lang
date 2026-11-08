pub mod errors_csv_data_binder;
pub mod errors_csv_data_parser;
pub mod errors_data_validator;
pub mod errors_parser;
pub mod errors_preexec;
pub mod errors_semanalyzer;
pub mod errors_type_binder;

use errors_csv_data_binder::CsvDataBinderErr;
use errors_csv_data_parser::CsvDataParserErr;
use errors_data_validator::DataValidatorErr;
use errors_parser::ParserErr;
use errors_preexec::PreExecErr;
use errors_semanalyzer::SemanalyzerErr;

#[derive(Debug, PartialEq)]
pub enum LangErr {
    PreExec(PreExecErr),
    ParserSource(ParserErr),
    ParserSchema(ParserErr),
    SemanalyzerSource(SemanalyzerErr),
    SemanalyzerSchema(SemanalyzerErr),
    DataValidator(DataValidatorErr),
    CsvDataParser(CsvDataParserErr),
    CsvDataBinder(CsvDataBinderErr),
}
