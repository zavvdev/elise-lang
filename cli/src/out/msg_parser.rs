use elise_shared::errors::errors_parser::ParserErr;

use crate::out::utils::{print_error_at_code, print_silent_err};

pub fn print_parser_err(parser_err: &ParserErr) {
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
        InvalDictPair(err_info) => ("Invalid dictionary key value pair", err_info),
        InvalFnName(err_info) => ("Invalid function name", err_info),
    };

    let source_code = match &info.1.source_code_slice {
        Some(code) => code,
        None => "",
    };

    print_silent_err(info.0, Some("Parser error"));
    print_error_at_code(source_code, info.1.row, info.1.col);
}
