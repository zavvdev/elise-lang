use std::collections::HashMap;

use crate::symbol_table::SymbolId;

pub struct Scope {
    pub bindings: HashMap<String, SymbolId>,
}

pub struct ScopeStack {
    pub scopes: Vec<Scope>,
}

impl Default for ScopeStack {
    fn default() -> Self {
        Self::new()
    }
}

impl ScopeStack {
    pub fn new() -> Self {
        Self { scopes: vec![] }
    }

    pub fn push(&mut self) {
        self.scopes.push(Scope {
            bindings: HashMap::new(),
        });
    }

    pub fn pop(&mut self) {
        self.scopes.pop();
    }

    pub fn define(&mut self, identifier_name: String, symbol_id: SymbolId) {
        if let Some(last_scope) = self.scopes.last_mut() {
            last_scope.bindings.insert(identifier_name, symbol_id);
        }
    }

    // Returns (SymbolId, depth_from_top) - depth matters for closures
    pub fn resolve(&self, name: &str) -> Option<(SymbolId, usize)> {
        for (depth, scope) in self.scopes.iter().rev().enumerate() {
            if let Some(&id) = scope.bindings.get(name) {
                return Some((id, depth));
            }
        }
        None
    }
}
