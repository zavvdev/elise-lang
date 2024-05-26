use std::collections::HashMap;

use crate::{parser::models::expression::Expr, types};

#[derive(Debug, PartialEq, Clone)]
pub struct FnDeclaration {
    pub name: String,
    pub args: Vec<String>,
    pub body: Vec<Expr>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum EvalResult {
    Nil,
    Number(types::Number),
    Boolean(bool),
    String(String),
    FnDeclaration(FnDeclaration),
}

#[derive(Clone, Debug)]
pub struct EnvRecord {
    pub value: EvalResult,
    pub mutable: bool,
}

#[derive(Clone, Debug)]
pub struct Env {
    table: HashMap<String, EnvRecord>,
    parent_env: Option<Box<Env>>,
}

impl Env {
    pub fn new() -> Self {
        Self {
            table: HashMap::new(),
            parent_env: None,
        }
    }

    pub fn attach_parent(&mut self, parent_env: &Env) {
        // TODO: Get rid of clone
        self.parent_env = Some(Box::new(parent_env.clone()));
    }

    pub fn get(&self, key: &str) -> Option<&EnvRecord> {
        match self.table.get(key) {
            Some(value) => Some(value),
            None => match &self.parent_env {
                Some(parent_env) => parent_env.get(key),
                None => None,
            },
        }
    }

    pub fn set(&mut self, key: String, value: EnvRecord) {
        self.table.insert(key, value);
    }

    pub fn has(&self, key: &str) -> bool {
        self.table.contains_key(key)
            || match &self.parent_env {
                Some(parent_env) => parent_env.has(key),
                None => false,
            }
    }
}
