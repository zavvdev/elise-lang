pub mod errors_csv_binder;
pub mod errors_csv_parser;
pub mod errors_csv_schema_resolver;
pub mod errors_parser;
pub mod errors_semantic_analyzer;

use errors_csv_binder::CsvBinderErr;
use errors_csv_parser::CsvParserErr;
use errors_csv_schema_resolver::CsvSchemaResolverErr;
use errors_parser::ParserErr;
use errors_semantic_analyzer::SemanticAnalyzerErr;

#[derive(Debug, PartialEq)]
pub enum LangErr {
    ParserSource(ParserErr),
    ParserSchema(ParserErr),
    SemanticAnalyzer(SemanticAnalyzerErr),
    CsvParser(CsvParserErr),
    CsvSchemaResolver(CsvSchemaResolverErr),
    CsvBinder(CsvBinderErr),
}
