use csv::{ErrorKind, ReaderBuilder};
use elise_shared::errors::{
    LangErr,
    errors_csv_parser::{CsvParserErr, Pos},
};

pub struct CsvParser<'a> {
    data: &'a str,
}

#[derive(Debug)]
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
                pos: pos.as_ref().map(|p| Pos { line: p.line() }),
                expected_len: *expected_len,
                actual_len: *len,
            },
            csv::ErrorKind::Utf8 { pos, err } => CsvParserErr::InvalidUtf8 {
                pos: pos.as_ref().map(|p| Pos { line: p.line() }),
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
