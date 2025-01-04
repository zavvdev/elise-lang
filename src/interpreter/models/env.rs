use std::collections::HashMap;

use crate::{parser::models::expression::Expr, types};

#[derive(Debug, PartialEq, Clone)]
pub struct FnDeclaration {
    pub name: String,
    pub args: Vec<String>,
    pub body: Vec<Expr>,
    pub lexical_env: Env,
}

#[derive(Debug, PartialEq, Clone)]
pub enum EvalResult {
    Nil,
    Number(types::Number),
    Boolean(bool),
    String(String),
    FnDeclaration(FnDeclaration),
}

#[derive(Clone, Debug, PartialEq)]
pub struct EnvRecord {
    pub value: EvalResult,
    pub mutable: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Env {
    table: HashMap<String, EnvRecord>,
    parent_env: Option<Box<Env>>,

    // Closure environment. Prioritizes over parent_env.
    lexical_env: Option<Box<Env>>,
}

impl Env {
    pub fn new() -> Self {
        Self {
            table: HashMap::new(),
            parent_env: None,
            lexical_env: None,
        }
    }

    pub fn attach_parent(&mut self, parent_env: &Env) {
        // TODO: Get rid of clone
        self.parent_env = Some(Box::new(parent_env.clone()));
    }

    pub fn attach_lexical(&mut self, lexical_env: &Env) {
        // TODO: Get rid of clone
        self.lexical_env = Some(Box::new(lexical_env.clone()));
    }

    pub fn get(&self, key: &str) -> Option<&EnvRecord> {
        if let Some(record) = self.table.get(key) {
            return Some(record);
        }
    
        // Lexical environment has higher priority

        if let Some(lexical) = &self.lexical_env {
            if let Some(record) = lexical.get(key) {
                return Some(record);
            }
        }

        if let Some(parent) = &self.parent_env {
            if let Some(record) = parent.get(key) {
                return Some(record);
            }
        }

        None
    }

    pub fn set(&mut self, key: String, value: EnvRecord) {
        self.table.insert(key, value);
    }

    pub fn has(&self, key: &str) -> bool {
        self.table.contains_key(key)
    }

    pub fn has_deep(&self, key: &str) -> bool {
        match &self.parent_env {
            Some(parent_env) => Self::has_in_parent(parent_env, key),
            None => false,
        }
    }

    fn has_in_parent(parent: &Box<Env>, key: &str) -> bool {
        parent.table.contains_key(key)
            || match &parent.parent_env {
                Some(parent_env) => Self::has_in_parent(&parent_env, key),
                None => false,
            }
    }
}
