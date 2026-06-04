use crate::out::utils::{self, print_err_source_code_slice};
use crate::out::utils::{get_source_code_slice, print_err_source_code_pos};
use elise_errors::errors_parser::ParserErr;

pub fn print_err(parser_err: &ParserErr, source_code: &[u8]) {
    use ParserErr::*;

    let info = match parser_err {
        UnexpTok(err_info) => ("Unexpected token", err_info),
        UnexpEoFile(err_info) => ("Unexpected end of file", err_info),
        UnexpEoList(err_info) => ("Unexpected end of list", err_info),
        UnexpEoDict(err_info) => ("Unexpected end of dictionary", err_info),
        UnexpEoFn(err_info) => ("Unexpected end of function", err_info),
        UnexpDictKey(err_info) => ("Unexpected dictionary key", err_info),
        InvalNum(err_info) => ("Invalid number", err_info),
        InvalStr(err_info) => ("Invalid string", err_info),
        UntermStr(err_info) => ("Unterminated string", err_info),
        InvalDictPair(err_info) => ("Invalid dictionary key value pair", err_info),
        InvalFnName(err_info) => ("Invalid function name", err_info),
    };

    utils::print_err(info.0, Some("Parser error"));

    if let Some(code) = &get_source_code_slice(source_code, info.1.pos) {
        print_err_source_code_pos(code.row, code.col);
        print_err_source_code_slice(&code.slice, code.col);
    };
}
