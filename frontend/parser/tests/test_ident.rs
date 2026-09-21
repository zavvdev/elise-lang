use elise_ast::{AstNode, AstNodeExpr, AstNodeExprPrim};
use elise_parser::Prelude;
use elise_shared::{
    shared_errors::errors_parser::{ParserErr, ParserErrInfo},
    shared_types::Span,
};

mod common;

#[test]
fn should_reject_invalid_names() {
    let identifiers: Vec<(&str, usize, fn(ParserErrInfo) -> ParserErr)> = vec![
        ("1asd", 1, ParserErr::InvalNum),
        ("!asd", 0, ParserErr::UnexpTok),
        ("#asd", 0, ParserErr::UnexpTok),
        ("$asd", 0, ParserErr::UnexpTok),
        ("%asd", 0, ParserErr::UnexpTok),
        ("^asd", 0, ParserErr::UnexpTok),
        ("&asd", 0, ParserErr::UnexpTok),
        ("*asd", 0, ParserErr::UnexpTok),
        ("-asd", 1, ParserErr::InvalNum),
        ("_asd", 0, ParserErr::UnexpTok),
        ("=asd", 0, ParserErr::UnexpTok),
        ("+asd", 0, ParserErr::UnexpTok),
        ("?asd", 0, ParserErr::UnexpTok),
        ("?asd", 0, ParserErr::UnexpTok),
        (">asd", 0, ParserErr::UnexpTok),
        ("<asd", 0, ParserErr::UnexpTok),
        ("/asd", 0, ParserErr::UnexpTok),
        ("asd<", 3, ParserErr::UnexpTok),
        ("asd>", 3, ParserErr::UnexpTok),
        ("asd/", 4, ParserErr::UnexpTok),
        ("asd+", 4, ParserErr::UnexpTok),
        ("asd%", 4, ParserErr::UnexpTok),
    ];
    for (identifier, pos, err) in identifiers {
        assert_eq!(
            Prelude::new(identifier.as_bytes()).parse(),
            Err(err(ParserErrInfo { pos }))
        );
    }
}

#[test]
fn should_parse() {
    let identifiers = vec![
        ("asd", 3),
        ("asd?", 4),
        ("as?d", 4),
        ("as5?d", 5),
        ("asd-", 4),
        ("as-d", 4),
        ("asd!", 4),
        ("as!d", 4),
        ("asd_", 4),
    ];
    for (identifier, end) in identifiers {
        let ast = Prelude::new(identifier.as_bytes()).parse();
        assert_eq!(
            ast,
            Ok(vec![AstNode::Expr(AstNodeExpr::Ident(AstNodeExprPrim {
                lexeme: identifier.to_string(),
                span: Span { start: 0, end },
            }))])
        );
    }
}
