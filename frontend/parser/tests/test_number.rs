use elise_ast::{AstNode, AstNodeExpr, AstNodeExprPrim};
use elise_parser::Prelude;
use elise_shared::{
    shared_errors::errors_parser::{ParserErr, ParserErrInfo},
    shared_types::Span,
};

mod common;

#[test]
fn should_not_contain_non_numeric_tokens() {
    let forbidded_tokens = vec![
        ("0a", 1),
        ("-0a", 2),
        ("0.a", 2),
        ("-0.a", 3),
        ("1a", 1),
        ("1.a", 2),
        ("-1a", 2),
        ("-1.a", 3),
        ("12a2", 2),
        ("0.2a", 3),
    ];

    for (token, pos) in forbidded_tokens {
        assert_eq!(
            Prelude::new(token.as_bytes()).parse(),
            Err(ParserErr::InvalNum(ParserErrInfo { pos }))
        );
    }
}

#[test]
fn should_not_allow_more_than_one_minus_token() {
    let forbidded_tokens = vec![("--1", 1), ("-1-2", 2), ("-2-3-", 2)];

    for (token, pos) in forbidded_tokens {
        assert_eq!(
            Prelude::new(token.as_bytes()).parse(),
            Err(ParserErr::InvalNum(ParserErrInfo { pos }))
        );
    }
}

#[test]
fn should_not_allow_more_than_one_period_token() {
    let forbidded_tokens = vec![("0.2.3", 3), ("0.3.", 3)];

    for (token, pos) in forbidded_tokens {
        assert_eq!(
            Prelude::new(token.as_bytes()).parse(),
            Err(ParserErr::InvalNum(ParserErrInfo { pos }))
        );
    }
}

#[test]
fn should_not_allow_start_with_zero_if_not_float() {
    let forbidded_tokens = vec![("02", 1), ("00", 1), ("00.3", 1), ("02.4", 1)];

    for (token, pos) in forbidded_tokens {
        assert_eq!(
            Prelude::new(token.as_bytes()).parse(),
            Err(ParserErr::InvalNum(ParserErrInfo { pos }))
        );
    }
}

#[test]
fn should_not_allow_start_from_minus_if_nothing_follows() {
    let code = "-".to_string();
    assert_eq!(
        Prelude::new(&code.as_bytes()).parse(),
        Err(ParserErr::InvalNum(ParserErrInfo { pos: 1 }))
    );
}

#[test]
fn should_not_allow_separator_after_minus() {
    let forbidded_tokens = vec![("- 2", 1), ("-\n2", 1)];

    for (token, pos) in forbidded_tokens {
        assert_eq!(
            Prelude::new(token.as_bytes()).parse(),
            Err(ParserErr::InvalNum(ParserErrInfo { pos }))
        );
    }
}

#[test]
fn should_parse_integers() {
    let numbers = vec![
        ("-0", 2),
        ("0", 1),
        ("-1", 2),
        ("2", 1),
        ("-9", 2),
        ("123", 3),
        ("-999999", 7),
        ("101", 3),
    ];
    for (number, end) in numbers {
        let ast = Prelude::new(number.as_bytes()).parse();
        assert_eq!(
            ast,
            Ok(vec![AstNode::Expr(AstNodeExpr::Int(AstNodeExprPrim {
                lexeme: number.to_string(),
                span: Span { start: 0, end },
            }))])
        );
    }
}

#[test]
fn should_parse_floats() {
    let numbers = vec![
        ("-0.0", 4),
        ("0.2", 3),
        ("-1.34", 5),
        ("22.4456", 7),
        ("-999.3234", 9),
        ("99999.900", 9),
    ];
    for (number, end) in numbers {
        let ast = Prelude::new(number.as_bytes()).parse();
        assert_eq!(
            ast,
            Ok(vec![AstNode::Expr(AstNodeExpr::Float(AstNodeExprPrim {
                lexeme: number.to_string(),
                span: Span { start: 0, end },
            }))])
        );
    }
}

#[test]
fn should_parse_numbers_that_are_separated() {
    let ast = Prelude::new(
        "3
56  -9   3.2"
            .as_bytes(),
    )
    .parse();
    assert_eq!(
        ast,
        Ok(vec![
            AstNode::Expr(AstNodeExpr::Int(AstNodeExprPrim {
                lexeme: "3".to_string(),
                span: Span { start: 0, end: 1 },
            })),
            AstNode::Expr(AstNodeExpr::Int(AstNodeExprPrim {
                lexeme: "56".to_string(),
                span: Span { start: 2, end: 4 },
            })),
            AstNode::Expr(AstNodeExpr::Int(AstNodeExprPrim {
                lexeme: "-9".to_string(),
                span: Span { start: 6, end: 8 },
            })),
            AstNode::Expr(AstNodeExpr::Float(AstNodeExprPrim {
                lexeme: "3.2".to_string(),
                span: Span { start: 11, end: 14 },
            })),
        ])
    );
}

#[test]
fn should_not_allow_invalid_scientific_notation_numbers() {
    let forbidded_tokens = vec![("1e1.2", 3), ("1e-", 3), ("1e", 2)];

    for (token, pos) in forbidded_tokens {
        assert_eq!(
            Prelude::new(token.as_bytes()).parse(),
            Err(ParserErr::InvalNum(ParserErrInfo { pos }))
        );
    }
}

#[test]
fn should_parse_scientific_numbers_as_floats() {
    let numbers = vec![
        ("0e0", 3),
        ("-0e0", 4),
        ("-0e-0", 5),
        ("0e-0", 4),
        ("1e0", 3),
        ("1e-0", 4),
        ("1e3", 3),
        ("10e3", 4),
        ("102e302", 7),
        ("1E3", 3),
        ("1e-3", 4),
        ("10e-30", 6),
        ("102e-304", 8),
        ("1.5e10", 6),
        ("1.504e101", 9),
        ("-2.3e-5", 7),
        ("-2.30e-502", 10),
    ];
    for (number, end) in numbers {
        let ast = Prelude::new(number.as_bytes()).parse();
        assert_eq!(
            ast,
            Ok(vec![AstNode::Expr(AstNodeExpr::Float(AstNodeExprPrim {
                lexeme: number.to_string(),
                span: Span { start: 0, end },
            }))])
        );
    }
}
