pub mod errors_csv_parser;
pub mod errors_sc_parser;

use errors_csv_parser::CsvParserErr;
use errors_sc_parser::ScParserErr;

#[derive(Debug, PartialEq)]
pub enum LangErr {
    ScParser(ScParserErr),
    CsvParser(CsvParserErr),
}
