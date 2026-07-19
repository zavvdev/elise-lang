pub mod errors_common;
pub mod errors_csv_binder;
pub mod errors_csv_parser;
pub mod errors_csv_schema_resolver;
pub mod errors_parser;
pub mod errors_semanalyzer;

use errors_csv_binder::CsvBinderErr;
use errors_csv_parser::CsvParserErr;
use errors_csv_schema_resolver::CsvSchemaResolverErr;
use errors_parser::ParserErr;
use errors_semanalyzer::SemanalyzerErr;

use crate::shared_errors::errors_common::CommonErr;

#[derive(Debug, PartialEq)]
pub enum LangErr {
    Common(CommonErr),
    ParserSource(ParserErr),
    ParserSchema(ParserErr),
    SemanticAnalyzer(SemanalyzerErr),
    CsvParser(CsvParserErr),
    CsvSchemaResolver(CsvSchemaResolverErr),
    CsvBinder(CsvBinderErr),
}
