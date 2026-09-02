pub mod errors_csv_data_binder;
pub mod errors_csv_parser;
pub mod errors_csv_data_validator;
pub mod errors_parser;
pub mod errors_preexec;
pub mod errors_schema_resolver;
pub mod errors_semanalyzer;

use errors_csv_data_binder::CsvDataBinderErr;
use errors_csv_parser::CsvParserErr;
use errors_parser::ParserErr;
use errors_schema_resolver::SchemaResolverErr;
use errors_semanalyzer::SemanalyzerErr;
use errors_csv_data_validator::CsvDataValidatorErr;

use crate::shared_errors::errors_preexec::PreExecErr;

#[derive(Debug, PartialEq)]
pub enum LangErr {
    PreExec(PreExecErr),
    ParserSource(ParserErr),
    ParserSchema(ParserErr),
    SchemaResolver(SchemaResolverErr),
    SemanticAnalyzer(SemanalyzerErr),
    CsvDataValidator(CsvDataValidatorErr),
    CsvParser(CsvParserErr),
    CsvDataBinder(CsvDataBinderErr),
}
