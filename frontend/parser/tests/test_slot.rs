use elise_ast::{AstNode, AstNodeExpr, AstNodeExprPrim};
use elise_parser::Prelude;
use elise_shared::{
    shared_errors::errors_parser::{ParserErr, ParserErrInfo},
    shared_types::Span,
};

mod common;

#[test]
fn should_parse() {
    let slots = vec![
        ("@asd", 4),
        ("@asd?", 5),
        ("@as?d", 5),
        ("@as5?d", 6),
        ("@asd-", 5),
        ("@as-d", 5),
        ("@asd!", 5),
        ("@as!d", 5),
        ("@asd_", 5),
    ];
    for (slot, end) in slots {
        let ast = Prelude::new(slot.as_bytes()).parse();
        assert_eq!(
            ast,
            Ok(vec![AstNode::Expr(AstNodeExpr::Slot(AstNodeExprPrim {
                lexeme: slot[1..].to_string(),
                span: Span { start: 0, end },
            }))])
        );
    }
}

#[test]
fn should_reject_invalid_names() {
    let slots = vec![
        ("@1asd", 5),
        ("@!asd", 5),
        ("@@asd", 5),
        ("@#asd", 5),
        ("@$asd", 5),
        ("@%asd", 5),
        ("@^asd", 5),
        ("@&asd", 5),
        ("@*asd", 5),
        ("@-asd", 5),
        ("@_asd", 5),
        ("@=asd", 5),
        ("@+asd", 5),
        ("@?asd", 5),
        ("@?asd", 5),
        ("@>asd", 1),
        ("@<asd", 1),
        ("@/asd", 5),
        ("@@asd", 5),
        ("@ asd", 1),
        ("@asd<", 4),
        ("@asd>", 4),
        ("@asd%", 5),
        ("@asd$", 5),
    ];
    for (slot, pos) in slots {
        assert_eq!(
            Prelude::new(slot.as_bytes()).parse(),
            Err(ParserErr::UnexpTok(ParserErrInfo { pos }))
        );
    }
}
