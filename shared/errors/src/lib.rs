pub mod errors_binder;
pub mod errors_csv_parser;
pub mod errors_csv_schema_resolver;
pub mod errors_parser;

use errors_binder::BinderErr;
use errors_csv_parser::CsvParserErr;
use errors_csv_schema_resolver::CsvSchemaResolverErr;
use errors_parser::ParserErr;

#[derive(Debug, PartialEq)]
pub enum LangErr {
    ParserSource(ParserErr),
    ParserSchema(ParserErr),
    CsvParser(CsvParserErr),
    CsvSchemaResolver(CsvSchemaResolverErr),
    Binder(BinderErr),
}
