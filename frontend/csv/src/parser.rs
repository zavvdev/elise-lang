use csv::{ErrorKind, ReaderBuilder};
use elise_errors::{
    LangErr,
    errors_csv_parser::{CsvParserErr, CsvParserErrPos},
};

pub struct CsvParser<'a> {
    data: &'a str,
}

#[derive(Debug, PartialEq)]
pub struct CsvParserRecord {
    pub row: Vec<String>,
}

impl<'a> CsvParser<'a> {
    pub fn new(data: &'a str) -> Self {
        Self { data }
    }

    fn map_error(kind: &ErrorKind) -> LangErr {
        LangErr::CsvParser(match kind {
            csv::ErrorKind::UnequalLengths {
                pos,
                expected_len,
                len,
            } => CsvParserErr::UneqLen {
                pos: pos.as_ref().map(|p| CsvParserErrPos { line: p.line() }),
                expected_len: *expected_len,
                actual_len: *len,
            },
            csv::ErrorKind::Utf8 { pos, err } => CsvParserErr::InvalidUtf8 {
                pos: pos.as_ref().map(|p| CsvParserErrPos { line: p.line() }),
                detail: err.to_string(),
            },
            csv::ErrorKind::Io(io_err) => CsvParserErr::Io {
                kind: io_err.kind().to_string(),
                detail: io_err.to_string(),
            },
            _ => CsvParserErr::Unknown,
        })
    }

    pub fn parse(&self) -> Result<Vec<CsvParserRecord>, LangErr> {
        let mut records: Vec<CsvParserRecord> = vec![];

        let mut reader = ReaderBuilder::new()
            .has_headers(true)
            .from_reader(self.data.as_bytes());

        for result in reader.records() {
            match result {
                Ok(rec) => records.push(CsvParserRecord {
                    row: rec.iter().map(str::to_owned).collect::<Vec<String>>(),
                }),
                Err(err) => {
                    return Err(Self::map_error(err.kind()));
                }
            }
        }

        Ok(records)
    }
}

// ==================================================================
//
//  TESTS START
//
// ==================================================================

#[cfg(test)]
mod tests {
    use elise_errors::{
        LangErr,
        errors_csv_parser::{CsvParserErr::*, CsvParserErrPos},
    };

    use crate::parser::{CsvParser, CsvParserRecord};

    #[test]
    fn parse_should_return_parsed_records() {
        let data = "name,age\n\"John\",\"25\"\n\"Jane\",\"26\"";
        let parser = CsvParser::new(&data);

        let row1 = CsvParserRecord {
            row: vec!["John".to_string(), "25".to_string()],
        };

        let row2 = CsvParserRecord {
            row: vec!["Jane".to_string(), "26".to_string()],
        };

        assert_eq!(parser.parse(), Ok(vec![row1, row2]));
    }

    #[test]
    fn parse_should_parse_empty() {
        let data = "name,age";
        let parser = CsvParser::new(&data);
        assert_eq!(parser.parse(), Ok(vec![]));
    }

    #[test]
    fn parse_should_return_uneq_len_error() {
        let data = "name,age\n\"John\"\n\"Jane\",\"26\"";
        let parser = CsvParser::new(&data);

        assert_eq!(
            parser.parse(),
            Err(LangErr::CsvParser(UneqLen {
                pos: Some(CsvParserErrPos { line: 2 }),
                expected_len: 2,
                actual_len: 1
            }))
        );
    }
}

// ==================================================================
//
//  TESTS END
//
// ==================================================================
