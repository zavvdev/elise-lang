use crate::out::utils::{self, print_source_code_slice};
use crate::out::utils::{get_source_code_slice, print_err_source_code_pos};
use elise_shared_diagnostics::errors_parser::ParserErr;

pub fn print_err(parser_err: &ParserErr, source_code: &[u8]) {
    use ParserErr::*;

    let (category, message, info) = match parser_err {
        UnexpTok(err_info) => ("TODO", "Unexpected token", err_info),
        UnexpEoList(err_info) => ("TODO", "Unexpected end of list", err_info),
        UnexpEoGeneric(err_info) => ("TODO", "Unexpected end of generic", err_info),
        UnexpEoDict(err_info) => ("TODO", "Unexpected end of dictionary", err_info),
        UnexpEoFn(err_info) => ("TODO", "Unexpected end of function", err_info),
        UnexpDictKey(err_info) => (
            "TODO",
            "Unexpected dictionary key. Keys must be strings",
            err_info,
        ),
        UnexpListItem(err_info) => (
            "TODO",
            "Unexpected list item. List can contain only expressions",
            err_info,
        ),

        InvalNum(err_info) => ("TODO", "Invalid number", err_info),
        InvalStr(err_info) => ("TODO", "Invalid string", err_info),
        InvalDictPair(err_info) => ("TODO", "Invalid dictionary key-value pair", err_info),
        InvalFnName(err_info) => (
            "TODO",
            "Invalid function name. Function names must start with a letter and may only contain letters, digits, '-', '?', '!' and '_'",
            err_info,
        ),
        InvalGenericTypeAnnot(err_info) => (
            "TODO",
            r##"Invalid generic type annotation. Generic can only contain either a type alias (:List<:Int>), or a record type literal (:List<{ "id" :Int }>"##,
            err_info,
        ),

        EmptyGeneric(err_info) => ("TODO", "Generic cannot be empty", err_info),
        EmptyRecord(err_info) => (
            "TODO",
            "Record type literal must contain at least one key-value pair",
            err_info,
        ),

        UntermStr(err_info) => ("TODO", "Unclosed string", err_info),
        UntermDelim(err_info) => ("TODO", "Unclosed delimiter", err_info),
    };

    utils::print_err(category, parser_err.code(), message);

    if let Some(code) = &get_source_code_slice(source_code, info.pos) {
        print_err_source_code_pos(code.pos, "TODO");
        print_source_code_slice(&code.slice, code.pos.col);
    };
}
