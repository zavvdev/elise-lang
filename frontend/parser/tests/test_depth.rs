use elise_parser::Prelude;
use elise_shared::shared_errors::errors_parser::{ParserErr, ParserErrInfo};

mod common;

#[test]
fn should_reject_invalid() {
    let depth_cases: Vec<(&str, usize, fn(ParserErrInfo) -> ParserErr)> = vec![
        (".a())", 4, ParserErr::UnexpTok),
        (".a(()", 3, ParserErr::UnexpTok),
        (".a().a()))", 8, ParserErr::UnexpTok),
        ("()()))", 0, ParserErr::UnexpTok),
        ("())", 0, ParserErr::UnexpTok),
        ("(()", 0, ParserErr::UnexpTok),
        ("[]]", 2, ParserErr::UnexpTok),
        ("[][[][][]]][[", 10, ParserErr::UnexpTok),
        ("[{}}]", 3, ParserErr::UnexpTok),
        ("[{{{{}]", 6, ParserErr::UnexpDictKey),
        ("[{{}]", 4, ParserErr::UnexpDictKey),
    ];
    for (depth_case, pos, err) in depth_cases {
        assert_eq!(
            Prelude::new(depth_case.as_bytes()).parse(),
            Err(err(ParserErrInfo { pos }))
        );
    }
}
