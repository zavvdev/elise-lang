// ==========================
//
//  SC PARSER ERRORS START
//
// ==========================

#[derive(Debug, PartialEq)]
pub struct ScParserErrInfo {
    pub row: usize,
    pub col: usize,
    // This field should not store the whole source code.
    // Instead we just keep a slice of it where exactly
    // an error happened.
    pub source_code_slice: Option<String>,
}

#[derive(Debug, PartialEq)]
pub enum ScParserErr {
    UnexpTok(ScParserErrInfo),
    UnexpEoFile(ScParserErrInfo),
    UnexpEoList(ScParserErrInfo),
    UnexpEoDict(ScParserErrInfo),
    UnexpEoFn(ScParserErrInfo),
    UnexpDictKey(ScParserErrInfo),
    InvalNum(ScParserErrInfo),
    InvalStr(ScParserErrInfo),
    InvalDictPair(ScParserErrInfo),
    InvalFnName(ScParserErrInfo),
}

// ==========================
//
//  SC PARSER ERRORS END
//
// ==========================

// ==========================
//
//  D PARSER ERRORS START
//
// ==========================

#[derive(Debug, PartialEq)]
pub struct DParserErrInfo {}

#[derive(Debug, PartialEq)]
pub enum DParserErr {
    InvalRow(DParserErrInfo),
}

// ==========================
//
//  D PARSER ERRORS END
//
// ==========================

#[derive(Debug, PartialEq)]
pub enum LangErr {
    ScParser(ScParserErr),
    DParser(DParserErr),
}
