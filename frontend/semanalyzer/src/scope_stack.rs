//! # Scope Stack
//!
//! Tracks identifier bindings across nested scopes during semantic analysis.
//!
//! Each scope is a flat map of identifier name => SymbolId. Scopes are pushed
//! when entering a new block and popped on exit, naturally modeling lexical
//! scoping rules.
//!
//! This structure is a compile-time artifact only. It is discarded after
//! semantic analysis produces the HIR.

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

    /// Registers a new identifier in the current (innermost) scope.
    /// Called when semantic analysis encounters .let or .define declarations.
    pub fn define(&mut self, identifier_name: String, symbol_id: SymbolId) {
        if let Some(last_scope) = self.scopes.last_mut() {
            last_scope.bindings.insert(identifier_name, symbol_id);
        }
    }

    /// Walks from innermost to outermost scope to find an identifier.
    /// Returns (SymbolId, depth) where depth 0 means current scope. Depth is
    /// reserved for closure analysis.
    pub fn resolve(&self, name: &str) -> Option<(SymbolId, usize)> {
        for (depth, scope) in self.scopes.iter().rev().enumerate() {
            if let Some(&id) = scope.bindings.get(name) {
                return Some((id, depth));
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use crate::{scope_stack::ScopeStack, symbol_table::SymbolId};

    #[test]
    fn should_create_with_empty_scopes() {
        let stack = ScopeStack::new();
        assert_eq!(stack.scopes.len(), 0);
    }

    #[test]
    fn should_push_new_scope_with_empty_bindings() {
        let mut stack = ScopeStack::new();
        assert_eq!(stack.scopes.len(), 0);
        stack.push();
        assert_eq!(stack.scopes.len(), 1);
        assert_eq!(stack.scopes.first().unwrap().bindings.capacity(), 0);
    }

    #[test]
    fn should_pop_scope() {
        let mut stack = ScopeStack::new();
        assert_eq!(stack.scopes.len(), 0);
        stack.push();
        assert_eq!(stack.scopes.len(), 1);
        stack.pop();
        assert_eq!(stack.scopes.len(), 0);
    }

    #[test]
    fn should_not_define_if_stack_is_empty() {
        let mut stack = ScopeStack::new();
        stack.define("name".to_string(), SymbolId(1));
        assert_eq!(stack.scopes.len(), 0);
    }

    #[test]
    fn should_define_in_last_scope() {
        let mut stack = ScopeStack::new();
        stack.push();
        stack.define("name".to_string(), SymbolId(1));
        assert_eq!(
            stack.scopes.first().unwrap().bindings.get("name"),
            Some(&SymbolId(1))
        );
    }

    #[test]
    fn should_resolve_to_none_if_not_found() {
        let stack = ScopeStack::new();
        assert_eq!(stack.resolve("name"), None);
    }

    #[test]
    fn should_resolve_with_depth_0() {
        let mut stack = ScopeStack::new();
        stack.push();
        stack.define("name".to_string(), SymbolId(1));
        assert_eq!(stack.resolve("name"), Some((SymbolId(1), 0)));
    }

    #[test]
    fn should_resolve_with_depth_gt_0() {
        let mut stack = ScopeStack::new();
        stack.push();
        stack.define("name".to_string(), SymbolId(1));
        stack.push();
        assert_eq!(stack.resolve("name"), Some((SymbolId(1), 1)));
    }
}
