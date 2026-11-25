use crate::codes::{DiagnosticCode, DiagnosticCodeType};

#[derive(Debug, PartialEq)]
pub struct ParserErrInfo {
    pub pos: usize,
}

#[derive(Debug, PartialEq)]
pub enum ParserErr {
    UnexpTok(ParserErrInfo),
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
    InvalGenericTypeAnnot(ParserErrInfo),

    EmptyGeneric(ParserErrInfo),
    EmptyRecord(ParserErrInfo),

    UntermStr(ParserErrInfo),
    UntermDelim(ParserErrInfo),
}

impl ParserErr {
    pub fn code(&self) -> DiagnosticCodeType {
        match self {
            Self::UnexpTok(_) => DiagnosticCode::P101.code(),
            Self::UnexpEoList(_) => DiagnosticCode::P102.code(),
            Self::UnexpEoGeneric(_) => DiagnosticCode::P103.code(),
            Self::UnexpEoDict(_) => DiagnosticCode::P104.code(),
            Self::UnexpEoFn(_) => DiagnosticCode::P105.code(),
            Self::UnexpDictKey(_) => DiagnosticCode::P106.code(),
            Self::UnexpListItem(_) => DiagnosticCode::P107.code(),
            Self::InvalNum(_) => DiagnosticCode::P108.code(),
            Self::InvalStr(_) => DiagnosticCode::P109.code(),
            Self::InvalDictPair(_) => DiagnosticCode::P110.code(),
            Self::InvalFnName(_) => DiagnosticCode::P111.code(),
            Self::InvalGenericTypeAnnot(_) => DiagnosticCode::P112.code(),
            Self::EmptyGeneric(_) => DiagnosticCode::P113.code(),
            Self::EmptyRecord(_) => DiagnosticCode::P114.code(),
            Self::UntermStr(_) => DiagnosticCode::P115.code(),
            Self::UntermDelim(_) => DiagnosticCode::P116.code(),
        }
    }
}
