use elise_ast::{AstNode, AstNodeExpr, AstNodeExprDict, AstNodeExprPrim, AstNodeExprDictKey,
AstNodeExprList};
use elise_parser::Prelude;
use elise_shared::{
    shared_errors::errors_parser::{ParserErr, ParserErrInfo},
    shared_types::Span,
};

mod common;

#[test]
fn should_parse_empty() {
    let ast = Prelude::new("{}".as_bytes()).parse();
    assert_eq!(
        ast,
        Ok(vec![AstNode::Expr(AstNodeExpr::Dict(AstNodeExprDict {
            span: Span { start: 0, end: 2 },
            entries: vec![],
        }))])
    );
}

#[test]
fn should_parse_non_empty() {
    let ast = Prelude::new(
        r##"{
                    "a" 1,
                    "b" 2.3,
                    "c" false,
                    "d" null,
                    "e" [1],
                    "f" { "a2"  some-value }
                }"##
        .as_bytes(),
    )
    .parse();

    let pair_1 = (
        AstNodeExprDictKey {
            span: Span { start: 22, end: 25 },
            lexeme: "a".to_string(),
        },
        Box::new(AstNodeExpr::Int(AstNodeExprPrim {
            lexeme: "1".to_string(),
            span: Span { start: 26, end: 27 },
        })),
    );

    let pair_2 = (
        AstNodeExprDictKey {
            span: Span { start: 49, end: 52 },
            lexeme: "b".to_string(),
        },
        Box::new(AstNodeExpr::Float(AstNodeExprPrim {
            lexeme: "2.3".to_string(),
            span: Span { start: 53, end: 56 },
        })),
    );

    let pair_3 = (
        AstNodeExprDictKey {
            span: Span { start: 78, end: 81 },
            lexeme: "c".to_string(),
        },
        Box::new(AstNodeExpr::Bool(AstNodeExprPrim {
            lexeme: "false".to_string(),
            span: Span { start: 82, end: 87 },
        })),
    );

    let pair_4 = (
        AstNodeExprDictKey {
            span: Span {
                start: 109,
                end: 112,
            },
            lexeme: "d".to_string(),
        },
        Box::new(AstNodeExpr::Null(AstNodeExprPrim {
            lexeme: "null".to_string(),
            span: Span {
                start: 113,
                end: 117,
            },
        })),
    );

    let pair_5 = (
        AstNodeExprDictKey {
            span: Span {
                start: 139,
                end: 142,
            },
            lexeme: "e".to_string(),
        },
        Box::new(AstNodeExpr::List(AstNodeExprList {
            span: Span {
                start: 143,
                end: 146,
            },
            items: vec![Box::new(AstNodeExpr::Int(AstNodeExprPrim {
                span: Span {
                    start: 144,
                    end: 145,
                },
                lexeme: "1".to_string(),
            }))],
        })),
    );

    let pair_6 = (
        AstNodeExprDictKey {
            span: Span {
                start: 168,
                end: 171,
            },
            lexeme: "f".to_string(),
        },
        Box::new(AstNodeExpr::Dict(AstNodeExprDict {
            span: Span {
                start: 172,
                end: 192,
            },
            entries: vec![(
                AstNodeExprDictKey {
                    span: Span {
                        start: 174,
                        end: 178,
                    },
                    lexeme: "a2".to_string(),
                },
                Box::new(AstNodeExpr::Ident(AstNodeExprPrim {
                    span: Span {
                        start: 180,
                        end: 190,
                    },
                    lexeme: "some-value".to_string(),
                })),
            )],
        })),
    );

    assert_eq!(
        ast,
        Ok(vec![AstNode::Expr(AstNodeExpr::Dict(AstNodeExprDict {
            span: Span { start: 0, end: 210 },
            entries: vec![pair_1, pair_2, pair_3, pair_4, pair_5, pair_6],
        }))])
    );
}

#[test]
fn should_not_allow_invalid_pair() {
    let code = r##"{ "a" 1, "b" }"##;
    assert_eq!(
        Prelude::new(code.as_bytes()).parse(),
        Err(ParserErr::InvalDictPair(ParserErrInfo { pos: 13 }))
    );
}

#[test]
fn should_not_allow_invalid_key() {
    let inputs = vec![
        ("{ a 1 }", 3),
        (r##"{ 1 "2" }"##, 3),
        ("{ null false }", 6),
        ("{ false true }", 7),
        (r##"{ [] "`" }"##, 4),
        ("{ {} a }", 4),
    ];
    for (input, pos) in inputs {
        assert_eq!(
            Prelude::new(input.as_bytes()).parse(),
            Err(ParserErr::UnexpDictKey(ParserErrInfo { pos }))
        );
    }
}

#[test]
fn should_not_allow_non_closed() {
    let inputs: Vec<(&str, usize, fn(ParserErrInfo) -> ParserErr)> = vec![
        (r##"{ "a" 1 }}"##, 9, ParserErr::UnexpTok),
        (r##"{{ "1" "2" }"##, 12, ParserErr::UnexpDictKey),
    ];
    for (input, pos, err) in inputs {
        assert_eq!(
            Prelude::new(input.as_bytes()).parse(),
            Err(err(ParserErrInfo { pos }))
        );
    }
}
