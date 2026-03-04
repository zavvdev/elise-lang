use crate::out;

// =======================
// Token Definitions
// =======================

const T_FN_PREFIX: char = '.';
const T_FN_DECLARE: &str = "declare";

const T_LEFT_PAREN: char = '(';
const T_RIGHT_PAREN: char = ')';

const T_LEFT_SQR_BRACKET: char = '[';
const T_RIGHT_SQR_BRACKET: char = ']';

const T_MINUS: char = '-';
const T_PERIOD: char = '.';
const T_COMMA: char = ',';

// =======================
// Custom Types
// =======================

type TNumber = f64;

// =======================
// Parser
// =======================

#[derive(Debug)]
enum AstNodeValue {
    // Function,
    // Identifier,
    Number(TNumber),
}

#[derive(Debug)]
pub struct AstNode {
    value: AstNodeValue,
    tok_start: usize,
    children: Vec<Box<AstNode>>,
}

pub struct Parser<'a> {
    source_code: &'a str,
    tok_pos: usize,
    depth_stack: Vec<char>,
}

impl<'a> Parser<'a> {
    pub fn new(source_code: &'a str) -> Self {
        Self {
            source_code,
            tok_pos: 0,
            depth_stack: vec![],
        }
    }

    pub fn parse(&mut self) -> Vec<AstNode> {
        let mut ast: Vec<AstNode> = vec![];

        while let Some(current_char) = self.tok_get_at(self.tok_pos) {
            if current_char.is_whitespace() {
                self.tok_consume();
            } else if Self::number_is_start(&current_char) {
                ast.push(self.number_consume());
            }
        }

        ast
    }

    // Token

    fn tok_consume(&mut self) -> Option<char> {
        let tok = self.tok_get_at(self.tok_pos);
        self.tok_pos += 1;
        tok
    }

    fn tok_get_at(&mut self, pos: usize) -> Option<char> {
        if pos >= self.source_code.len() {
            return None;
        }
        self.source_code.chars().nth(pos)
    }

    // Error message

    fn error_arrow(len: usize) -> String {
        "-".repeat(len) + "^"
    }

    fn error_print(&self, message: &str, char_pos: usize) -> ! {
        let mut row = 0;
        let mut col = 0;

        let mut previous_row_start = 0;
        let mut preview_row_start = 0;
        let mut preview_row_end = 0;

        let mut found = false;

        for char in self.source_code.chars() {
            if preview_row_end == char_pos {
                found = true;
            }

            preview_row_end += 1;

            if char == '\n' {
                if found {
                    break;
                }

                previous_row_start = preview_row_start;
                preview_row_start = preview_row_end;

                row += 1;
                col = 0;
            } else if !found {
                col += 1;
            }
        }

        out::msg(&format!("\n{}", message));
        out::msg(&format!("At {}:{}\n", row + 1, col + 1));
        // out::msg(&format!("{}", self.source_code[previous_row_start..preview_row_end]));
        out::msg(&format!("{}\n", Self::error_arrow(col)));
        out::crash("Parsing error");
    }

    // Number

    fn number_is_digit(c: &char) -> bool {
        c.is_digit(10)
    }

    fn number_is_start(char: &char) -> bool {
        Self::number_is_digit(char) || *char == T_MINUS
    }

    fn number_is_end(char: &char) -> bool {
        char.is_whitespace()
            || *char == T_COMMA
            || *char == T_RIGHT_PAREN
            || *char == T_RIGHT_SQR_BRACKET
    }

    fn number_consume(&mut self) -> AstNode {
        let mut value = String::new();
        let mut float = false;
        let tok_start = self.tok_pos;

        while let Some(c) = self.tok_get_at(self.tok_pos) {
            if Self::number_is_end(&c) {
                break;
            } else if Self::number_is_digit(&c) {
                value.push(c);
                self.tok_consume();
            } else if c == T_MINUS && value.is_empty() {
                value.push(c);
                self.tok_consume();
            } else if c == T_PERIOD && !value.is_empty() && !float {
                float = true;
                value.push(c);
                self.tok_consume();
            } else {
                self.error_print("Invalid Number", self.tok_pos);
            }
        }

        let value = value.parse::<TNumber>();

        if !value.is_ok() {
            self.error_print("Invalid number", self.tok_pos)
        }

        AstNode {
            value: AstNodeValue::Number(value.unwrap()),
            tok_start,
            children: vec![],
        }
    }
}

// =======================
// Tests
// =======================

#[cfg(test)]
mod tests {
    #[test]
    fn should_return_none_if_source_code_is_empty_string() {
        panic!("TODO");
    }

    #[test]
    fn should_return_none_if_source_code_is_string_with_spaces() {
        panic!("TODO");
    }

    // Number

    #[test]
    #[should_panic]
    fn should_panic_if_number_contains_non_numeric_token() {
        // 1a, 12a2, 0.2a, -1a
        panic!("TODO");
    }

    #[test]
    #[should_panic]
    fn should_panic_if_number_contains_more_than_one_minus_token() {
        // --1, -1-2, -2-3-
        panic!("TODO");
    }

    #[test]
    #[should_panic]
    fn should_panic_if_number_contains_more_than_one_period_token() {
        // 0.2.3, 0.3.
        panic!("TODO");
    }

    #[test]
    #[should_panic]
    fn should_panic_if_number_starts_with_zero_and_not_float() {
        // 023
        panic!("TODO");
    }

    #[test]
    #[should_panic]
    fn should_panic_if_number_starts_with_period() {
        // .23
        panic!("TODO");
    }

    #[test]
    #[should_panic]
    fn should_panic_if_we_start_from_minus_and_nothing_follows() {
        // -
        panic!("TODO");
    }

    #[test]
    fn should_parse_positive_numbers() {
        // 2, 33, 444, 9999
        panic!("TODO");
    }

    #[test]
    fn should_parse_negative_numbers() {
        // -2, -33, -444, -9999
        panic!("TODO");
    }

    #[test]
    fn should_parse_positive_float_numbers() {
        // 2.0, 3.3, 4.44, 99.99, 0.234
        panic!("TODO");
    }

    #[test]
    fn should_parse_negative_float_numbers() {
        // -2.0, -3.3, -4.44, -99.99, -0.234
        panic!("TODO");
    }

    #[test]
    fn should_parse_numbers_correctly_that_are_separated() {
        // separated with space, multiple spaces, new lines, tabs
        panic!("TODO");
    }
}

// TODO:
// - [ ] Move messages to separate module
// - [ ] Move message pring to a separate module
// - [ ] Review number parsing
// - [ ] Fix an issue with message print
// - [ ] Write tests for number parsing
