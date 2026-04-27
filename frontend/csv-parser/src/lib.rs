use csv::ReaderBuilder;
use elise_shared::errors::{LangErr, errors_csv_parser::CsvParserErr};

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
            match result {
                Ok(rec) => records.push(CsvParserRecord {
                    row: rec.iter().map(str::to_owned).collect::<Vec<String>>(),
                }),
                Err(err) => {
                    println!("{:#?}", err);
                    return Err(LangErr::CsvParser(CsvParserErr::Impl));
                }
            }
        }

        Ok(records)
    }
}
