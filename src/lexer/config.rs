use super::models::token::TokenKind;

// Allowed set of characters: a-z A-Z - ? ! _ 0-9
// Not allowed at start: 0-9 - ? ! @ .
// Not allowed: whitespace and all others
pub const IDENTIFIER_REGEX: &str = r"^([^\d\-?!\.@\s+])([a-zA-Z\-\?!_\d])*$";

// Set of tokens that can be reduced to only one token if appear in sequence
pub const REDUSEABLE_TOKENS: [TokenKind; 2] = [TokenKind::Whitespace, TokenKind::Newline];
