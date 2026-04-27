pub mod errors_csv_parser;
pub mod errors_parser;

use errors_csv_parser::CsvParserErr;
use errors_parser::ParserErr;

#[derive(Debug, PartialEq)]
pub enum LangErr {
    Parser(ParserErr),
    CsvParser(CsvParserErr),
}
