const FN_PREFIX: &str = ".";
const FN_DECLARE: &str = "declare";
const FN_OPEN: &str = "(";
const FN_CLOSE: &str = ")";

type Number = f64;

#[derive(Debug)]
enum AstNodeKind {
    Function,
    Identifier,
    Number,
}

#[derive(Debug)]
pub struct AstNode {
    kind: AstNodeKind,
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

    fn next_node() -> Option<AstNode> {
        None
    }

    pub fn parse(self) -> Vec<AstNode> {
        let mut ast: Vec<AstNode> = vec![];

        while let Some(node) = Self::next_node() {
            ast.push(node);
        }

        ast
    }

    fn consume_token(&mut self) -> Option<char> {
        let tok = self.consume_token_at(self.tok_pos);
        self.tok_pos += 1;
        tok
    }

    fn consume_token_at(&mut self, pos: usize) -> Option<char> {
        if pos >= self.source_code.len() {
            return None;
        }
        self.source_code.chars().nth(pos)
    }
}
