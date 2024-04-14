pub mod models;

use self::models::{ast::AstNode, Parser};
use crate::lexer::models::token::Token;

pub fn parse(tokens: Vec<Token>) -> Vec<AstNode> {
    let mut parser = Parser::new(tokens);
    let mut nodes: Vec<AstNode> = Vec::new();

    while let Some(node) = parser.next_node() {
        nodes.push(node);
    }

    nodes
}

// ======== Tests ========

#[cfg(test)]
mod tests {
    use crate::lexer::{
        lexemes::{self, fn_lexeme_to_string},
        models::token::{TokenKind, TokenSpan},
    };

    use super::*;

    // ==========================

    //          Numbers

    // ==========================

    #[test]
    fn test_parse_int() {
        assert_eq!(
            parse(vec![Token {
                kind: TokenKind::Int(42),
                span: TokenSpan::new(0, 2, "42".to_string())
            }]),
            vec![AstNode {
                token_kind: TokenKind::Int(42),
                branches: vec![],
            }]
        );
    }

    #[test]
    fn test_parse_int_negative() {
        assert_eq!(
            parse(vec![
                Token {
                    kind: TokenKind::Minus,
                    span: TokenSpan::new(0, 1, lexemes::L_MINUS.to_string())
                },
                Token {
                    kind: TokenKind::Int(2),
                    span: TokenSpan::new(1, 2, "2".to_string())
                }
            ]),
            vec![AstNode {
                token_kind: TokenKind::Int(-2),
                branches: vec![],
            }]
        );
    }

    #[test]
    fn test_parse_float() {
        assert_eq!(
            parse(vec![Token {
                kind: TokenKind::Float(4.2),
                span: TokenSpan::new(0, 3, "4.2".to_string())
            }]),
            vec![AstNode {
                token_kind: TokenKind::Float(4.2),
                branches: vec![],
            }]
        );
    }

    #[test]
    fn test_parse_float_negative() {
        assert_eq!(
            parse(vec![
                Token {
                    kind: TokenKind::Minus,
                    span: TokenSpan::new(0, 1, lexemes::L_MINUS.to_string())
                },
                Token {
                    kind: TokenKind::Float(5.6),
                    span: TokenSpan::new(1, 4, "5.6".to_string())
                }
            ]),
            vec![AstNode {
                token_kind: TokenKind::Float(-5.6),
                branches: vec![],
            }]
        );
    }

    // ==========================

    //     Unexpected Tokens

    // ==========================

    #[test]
    #[should_panic(expected = "Parse error. Unexpected token \"-\"")]
    fn test_unexpected_token_trailing_minus() {
        parse(vec![Token {
            kind: TokenKind::Minus,
            span: TokenSpan::new(0, 1, lexemes::L_MINUS.to_string()),
        }]);
    }

    #[test]
    #[should_panic(expected = "Parse error. Unexpected token \"@add\"")]
    fn test_unexpected_token_minus() {
        parse(vec![
            Token {
                kind: TokenKind::Minus,
                span: TokenSpan::new(0, 1, lexemes::L_MINUS.to_string()),
            },
            Token {
                kind: TokenKind::FnAdd,
                span: TokenSpan::new(1, 5, fn_lexeme_to_string(lexemes::L_FN_ADD)),
            },
        ]);
    }
}
