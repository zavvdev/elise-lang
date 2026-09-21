#[derive(Debug, PartialEq)]
pub struct ParserErrInfo {
    pub pos: usize,
}

#[derive(Debug, PartialEq)]
pub enum ParserErr {
    UnexpTok(ParserErrInfo),
    UnexpEoFile(ParserErrInfo),
    UnexpEoList(ParserErrInfo),
    UnexpEoGeneric(ParserErrInfo),
    UnexpEoDict(ParserErrInfo),
    UnexpEoFn(ParserErrInfo),
    UnexpDictKey(ParserErrInfo),
    UnexpListItem(ParserErrInfo),

    InvalNum(ParserErrInfo),
    InvalStr(ParserErrInfo),
    InvalDictPair(ParserErrInfo),
    InvalFnName(ParserErrInfo),
    InvalGenericTypedef(ParserErrInfo),

    EmptyGeneric(ParserErrInfo),
    EmptyRecord(ParserErrInfo),

    UntermStr(ParserErrInfo),
}
