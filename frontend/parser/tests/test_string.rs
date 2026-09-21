use elise_ast::{AstNode, AstNodeExpr, AstNodeExprPrim};
use elise_parser::Prelude;
use elise_shared::{shared_errors::errors_parser::{ParserErr, ParserErrInfo}, shared_types::Span};

mod common;

#[test]
fn should_not_allow_new_line() {
    assert_eq!(
        Prelude::new(
            r#""Hello
                World""#
                .as_bytes()
        )
        .parse(),
        Err(ParserErr::InvalStr(ParserErrInfo { pos: 6 }))
    );
}

#[test]
fn should_not_allow_unterminated() {
    let strings = vec![(r#""Hello"#, 6), (r#""Hello\""#, 8)];
    for (string, end) in strings {
        assert_eq!(
            Prelude::new(string.as_bytes()).parse(),
            Err(ParserErr::UntermStr(ParserErrInfo { pos: end }))
        );
    }
}

#[test]
fn should_parse() {
    let strings = vec![
        (r#""""#, 2),
        (r#""Hello""#, 7),
        (r#""Hello World""#, 13),
        (r#""Hello       world!""#, 20),
        // Span is always bytes aware.
        // Each of these emojis are 4 bytes.
        (r#""123 2323 😄😄""#, 19),
    ];
    for (string, end) in strings {
        let ast = Prelude::new(string.as_bytes()).parse();
        assert_eq!(
            ast,
            Ok(vec![AstNode::Expr(AstNodeExpr::Str(AstNodeExprPrim {
                lexeme: string
                    .split("\"")
                    .into_iter()
                    .collect::<Vec<&str>>()
                    .get(1)
                    .unwrap()
                    .to_string(),
                span: Span { start: 0, end },
            }))])
        );
    }
}

#[test]
fn should_parse_escape_chars() {
    let strings = vec![
        (r#""\"""#, "\"", 4),
        (r#""Hello\r""#, "Hello\r", 9),
        (r#""Hello\n""#, "Hello\n", 9),
        (r#""Hello\0""#, "Hello\0", 9),
        (r#""Hello\\""#, "Hello\\", 9),
        (r#""Hello\tworld!""#, "Hello\tworld!", 15),
        (r#""\y""#, "y", 4),
    ];
    for (string, expected, end) in strings {
        let ast = Prelude::new(string.as_bytes()).parse();
        assert_eq!(
            ast,
            Ok(vec![AstNode::Expr(AstNodeExpr::Str(AstNodeExprPrim {
                lexeme: expected.to_string(),
                span: Span { start: 0, end },
            }))])
        );
    }
}
