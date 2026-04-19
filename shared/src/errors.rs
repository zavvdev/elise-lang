// ==========================
//
//  PARSER ERRORS START
//
// ==========================

#[derive(Debug, PartialEq)]
pub struct ParserErrInfo {
    pub row: usize,
    pub col: usize,
    // This field should not store the whole source code.
    // Instead we just keep a slice of it where exactly
    // an error happened.
    pub source_code_slice: Option<String>,
}

#[derive(Debug, PartialEq)]
pub enum ParserErr {
    UnexpTok(ParserErrInfo),
    UnexpEoFile(ParserErrInfo),
    UnexpEoList(ParserErrInfo),
    UnexpEoDict(ParserErrInfo),
    UnexpEoFn(ParserErrInfo),
    UnexpDictKey(ParserErrInfo),
    InvalNum(ParserErrInfo),
    InvalStr(ParserErrInfo),
    InvalDictPair(ParserErrInfo),
    InvalFnName(ParserErrInfo),
}

// ==========================
//
//  PARSER ERRORS END
//
// ==========================

#[derive(Debug, PartialEq)]
pub enum LangErr {
    Parser(ParserErr),
}
