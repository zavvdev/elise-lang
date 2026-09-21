use elise_ast::{AstNode, AstNodeTypedef, AstNodeTypedefGeneric, AstNodeTypedefRecordKey};
use elise_parser::Prelude;
use elise_shared::{
    shared_errors::errors_parser::{ParserErr, ParserErrInfo},
    shared_types::Span,
};

mod common;

#[test]
fn should_parse_without_generic() {
    let input = ":Int".as_bytes();
    let result = Prelude::new(input).parse();

    let expected_result = Ok(vec![AstNode::Typedef(AstNodeTypedef {
        span: Span { start: 0, end: 4 },
        lexeme: "Int".to_string(),
        generic: None,
    })]);

    assert_eq!(result, expected_result);
}

#[test]
fn should_not_allow_separator_after_colon() {
    let input = ": Int".as_bytes();
    let result = Prelude::new(input).parse();
    let expected_result = Err(ParserErr::UnexpTok(ParserErrInfo { pos: 1 }));
    assert_eq!(result, expected_result);
}

#[test]
fn should_reject_invalid_names() {
    let slots = vec![
        (":1asd", 1),
        (":!asd", 1),
        (":@asd", 1),
        (":#asd", 1),
        (":$asd", 1),
        (":%asd", 1),
        (":^asd", 1),
        (":&asd", 1),
        (":*asd", 1),
        (":-asd", 1),
        (":_asd", 1),
        (":=asd", 1),
        (":+asd", 1),
        (":?asd", 1),
        (":?asd", 1),
        (":>asd", 1),
        (":<asd", 1),
        (":/asd", 1),
        (":@asd", 1),
        (": asd", 1),
        (":asd>", 4),
        (":asd%", 5),
        (":asd$", 5),
    ];
    for (slot, pos) in slots {
        assert_eq!(
            Prelude::new(slot.as_bytes()).parse(),
            Err(ParserErr::UnexpTok(ParserErrInfo { pos }))
        );
    }
}

#[test]
fn should_parse_with_single_generic() {
    let input = ":List<:Str>".as_bytes();
    let result = Prelude::new(input).parse();

    let expected_result = Ok(vec![AstNode::Typedef(AstNodeTypedef {
        span: Span { start: 0, end: 11 },
        lexeme: "List".to_string(),
        generic: Some(AstNodeTypedefGeneric::Single(Box::new(AstNodeTypedef {
            span: Span { start: 6, end: 10 },
            lexeme: "Str".to_string(),
            generic: None,
        }))),
    })]);

    assert_eq!(result, expected_result);
}

#[test]
fn should_reject_unclosed_generic() {
    let input = ":List<<:Str>".as_bytes();
    let result = Prelude::new(input).parse();
    let expected_result = Err(ParserErr::UnexpTok(ParserErrInfo { pos: 6 }));
    assert_eq!(result, expected_result);
}

#[test]
fn should_reject_unopened_generic() {
    let input = ":List<:Str>>".as_bytes();
    let result = Prelude::new(input).parse();
    let expected_result = Err(ParserErr::UnexpTok(ParserErrInfo { pos: 11 }));
    assert_eq!(result, expected_result);
}

#[test]
fn should_reject_non_typedef_in_generic() {
    let input = ":List<123>".as_bytes();
    let result = Prelude::new(input).parse();
    let expected_result = Err(ParserErr::InvalGenericTypedef(ParserErrInfo { pos: 9 }));
    assert_eq!(result, expected_result);
}

#[test]
fn should_reject_empty_generic() {
    let input = ":List<>".as_bytes();
    let result = Prelude::new(input).parse();
    let expected_result = Err(ParserErr::EmptyGeneric(ParserErrInfo { pos: 6 }));
    assert_eq!(result, expected_result);
}

#[test]
fn should_reject_more_than_one_generic() {
    let input = ":List<:Str :Int>".as_bytes();
    let result = Prelude::new(input).parse();
    let expected_result = Err(ParserErr::UnexpTok(ParserErrInfo { pos: 10 }));
    assert_eq!(result, expected_result);
}

#[test]
fn should_parse_with_nested_single_generics() {
    let input = ":Optional<:Nullable<:List<:Int>>>".as_bytes();
    let result = Prelude::new(input).parse();

    let int = AstNodeTypedefGeneric::Single(Box::new(AstNodeTypedef {
        span: Span { start: 26, end: 30 },
        lexeme: "Int".to_string(),
        generic: None,
    }));

    let list = AstNodeTypedefGeneric::Single(Box::new(AstNodeTypedef {
        span: Span { start: 20, end: 31 },
        lexeme: "List".to_string(),
        generic: Some(int),
    }));

    let nullable = AstNodeTypedefGeneric::Single(Box::new(AstNodeTypedef {
        span: Span { start: 10, end: 32 },
        lexeme: "Nullable".to_string(),
        generic: Some(list),
    }));

    let expected_result = Ok(vec![AstNode::Typedef(AstNodeTypedef {
        span: Span { start: 0, end: 33 },
        lexeme: "Optional".to_string(),
        generic: Some(nullable),
    })]);

    assert_eq!(result, expected_result);
}

#[test]
fn should_parse_with_record_generic() {
    let input = r##":Dict<{ "id" :Int, "name" :Str }>"##.as_bytes();
    let result = Prelude::new(input).parse();

    let record = vec![
        (
            AstNodeTypedefRecordKey {
                lexeme: "id".to_string(),
                span: Span { start: 8, end: 12 },
            },
            Box::new(AstNodeTypedef {
                span: Span { start: 13, end: 17 },
                lexeme: "Int".to_string(),
                generic: None,
            }),
        ),
        (
            AstNodeTypedefRecordKey {
                lexeme: "name".to_string(),
                span: Span { start: 19, end: 25 },
            },
            Box::new(AstNodeTypedef {
                span: Span { start: 26, end: 30 },
                lexeme: "Str".to_string(),
                generic: None,
            }),
        ),
    ];

    let expected_result = Ok(vec![AstNode::Typedef(AstNodeTypedef {
        span: Span { start: 0, end: 33 },
        lexeme: "Dict".to_string(),
        generic: Some(AstNodeTypedefGeneric::Record(record)),
    })]);

    assert_eq!(result, expected_result);
}

#[test]
fn should_reject_unclosed_record() {
    let input = r##":Dict<{{ "a" :Int }>"##.as_bytes();
    let result = Prelude::new(input).parse();
    let expected_result = Err(ParserErr::UnexpDictKey(ParserErrInfo { pos: 17 }));
    assert_eq!(result, expected_result);
}

#[test]
fn should_reject_unopened_record() {
    let input = r##":Dict<{ "a" :Int }}>"##.as_bytes();
    let result = Prelude::new(input).parse();
    let expected_result = Err(ParserErr::UnexpTok(ParserErrInfo { pos: 18 }));
    assert_eq!(result, expected_result);
}

#[test]
fn should_reject_empty_record_generic() {
    let input = ":List<{}>".as_bytes();
    let result = Prelude::new(input).parse();
    let expected_result = Err(ParserErr::EmptyRecord(ParserErrInfo { pos: 8 }));
    assert_eq!(result, expected_result);
}

#[test]
fn should_reject_more_than_one_record_in_generic() {
    let input = r##":List<{ "id" :Int } { "name" :Str }>"##.as_bytes();
    let result = Prelude::new(input).parse();
    let expected_result = Err(ParserErr::UnexpTok(ParserErrInfo { pos: 19 }));
    assert_eq!(result, expected_result);
}

#[test]
fn should_parse_with_nested_record_generics() {
    let input = r##":Dict<{ "id" :Int, "address" :Dict<{ "street" :Str }> }>"##.as_bytes();
    let result = Prelude::new(input).parse();

    let record_street = vec![(
        AstNodeTypedefRecordKey {
            lexeme: "street".to_string(),
            span: Span { start: 37, end: 45 },
        },
        Box::new(AstNodeTypedef {
            span: Span { start: 46, end: 50 },
            lexeme: "Str".to_string(),
            generic: None,
        }),
    )];

    let record = vec![
        (
            AstNodeTypedefRecordKey {
                lexeme: "id".to_string(),
                span: Span { start: 8, end: 12 },
            },
            Box::new(AstNodeTypedef {
                span: Span { start: 13, end: 17 },
                lexeme: "Int".to_string(),
                generic: None,
            }),
        ),
        (
            AstNodeTypedefRecordKey {
                lexeme: "address".to_string(),
                span: Span { start: 19, end: 28 },
            },
            Box::new(AstNodeTypedef {
                span: Span { start: 29, end: 53 },
                lexeme: "Dict".to_string(),
                generic: Some(AstNodeTypedefGeneric::Record(record_street)),
            }),
        ),
    ];

    let expected_result = Ok(vec![AstNode::Typedef(AstNodeTypedef {
        span: Span { start: 0, end: 56 },
        lexeme: "Dict".to_string(),
        generic: Some(AstNodeTypedefGeneric::Record(record)),
    })]);

    assert_eq!(result, expected_result);
}
