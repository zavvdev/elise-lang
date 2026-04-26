use csv::ReaderBuilder;
use elise_shared::errors::{DParserErr::InvalRow, DParserErrInfo, LangErr};

pub struct CsvParser<'a> {
    data: &'a str,
}

#[derive(Debug)]
pub struct CsvParserRecord {
    row: Vec<String>,
}

impl<'a> CsvParser<'a> {
    pub fn new(data: &'a str) -> Self {
        Self { data }
    }

    pub fn parse(&self) -> Result<Vec<CsvParserRecord>, LangErr> {
        let mut records: Vec<CsvParserRecord> = vec![];

        let mut rdr = ReaderBuilder::new()
            .has_headers(true)
            .from_reader(self.data.as_bytes());

        for result in rdr.records() {
            if let Ok(result) = result {
                records.push(CsvParserRecord {
                    row: result.iter().map(str::to_owned).collect::<Vec<String>>(),
                })
            } else {
                // TODO: Add info to parser err info
                return Err(LangErr::DParser(InvalRow(DParserErrInfo {})));
            }
        }

        Ok(records)
    }
}
