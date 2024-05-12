use std::collections::HashMap;

use crate::types;

#[derive(Debug, PartialEq)]
pub enum EvalResult {
    Nil,
    Number(types::Number),
}

pub struct EnvRecord {
    value: EvalResult,
    mutable: bool,
}

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

    pub fn attach_parent(&mut self, parent_env: Env) {
        self.parent_env = Some(Box::new(parent_env));
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
}
