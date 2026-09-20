#[derive(Debug, PartialEq)]
pub struct ParserErrInfo {
    pub pos: usize,
}

#[derive(Debug, PartialEq)]
pub enum ParserErr {
    UnexpTok(ParserErrInfo),
    UnexpEoFile(ParserErrInfo),
    UnexpEoList(ParserErrInfo),
    UnexpEoTypedefGeneric(ParserErrInfo),
    UnexpEoDict(ParserErrInfo),
    UnexpEoFn(ParserErrInfo),
    UnexpDictKey(ParserErrInfo),
    UnexpRecordKey(ParserErrInfo),
    UnexpRecordValue(ParserErrInfo),
    UnexpListItem(ParserErrInfo),
    InvalNum(ParserErrInfo),
    InvalStr(ParserErrInfo),
    UntermStr(ParserErrInfo),
    InvalDictPair(ParserErrInfo),
    InvalFnName(ParserErrInfo),
    InvalGenericTypedef(ParserErrInfo),
    EmptyTypedefGeneric(ParserErrInfo),
}
