#[derive(Debug, PartialEq)]
pub enum DataAdapterErr {
    Csv(CsvDataAdapterErr),
}

// =========================================================
//
// CSV ERRORS START
//
// =========================================================

#[derive(Debug, PartialEq)]
pub enum CsvDataAdapterErr {
    Todo,
}

// =========================================================
//
// CSV ERRORS END
//
// =========================================================
