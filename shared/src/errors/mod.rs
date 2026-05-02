pub mod errors_csv_parser;
pub mod errors_parser;
pub mod errors_schema;

use errors_csv_parser::CsvParserErr;
use errors_parser::ParserErr;
use errors_schema::SchemaErr;

#[derive(Debug, PartialEq)]
pub enum LangErr {
    Parser(ParserErr),
    CsvParser(CsvParserErr),
    Schema(SchemaErr),
}
