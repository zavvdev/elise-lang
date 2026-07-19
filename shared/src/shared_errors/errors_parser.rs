#[derive(Debug, PartialEq)]
pub struct ParserErrInfo {
    pub pos: usize,
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
    UntermStr(ParserErrInfo),
    InvalDictPair(ParserErrInfo),
    InvalFnName(ParserErrInfo),
}
