//! # Symbol Table
//!
//! Maps identifiers to their semantic descriptors during compilation.
//!
//! The symbol table is a compile-time only structure — it is built during
//! semantic analysis and consumed by the bytecode emitter. It is discarded
//! after emission and never reaches the VM.
//!
//! Its sole purpose is to answer questions about identifiers:
//!   - Does this identifier exist in the current scope?
//!   - What type is it?
//!
//! It does NOT store values. Values are folded into AAST nodes
//! as compile-time constants.
//!
//! AAST reference nodes store only a SymbolId. When the emitter needs
//! type information for a referenced identifier it looks it up here
//! via that SymbolId rather than traversing the AAST to find the
//! definition node.
//!
//! ## Why we can't store type information inside AAstNode directly?
//!
//! AAST nodes represent structure (what the program says), while the
//! symbol table represents meaning (what identifiers refer to).
//! These are different concerns. What happens with a reference to an identifier:
//! let x: Int = 42
//! let y = x + 1   // <-- this reference to x
//! The AAST node for x in the second line is just a name — a ReferenceNode("x").
//! We have two options:
//! Option A — no symbol table: Store type info directly on the reference node.
//! But to do that, we'd have to resolve x during AST construction (or do a full
//! traversal every time the emitter needs the type). Worse, if x is referenced 50
//! times, we have 50 copies of the same type metadata scattered across the tree.
//! Option B — symbol table: The reference node stores only a SymbolId. The symbol
//! table holds the type once. The emitter does a O(1) lookup.

use std::collections::HashMap;

use crate::data_types::LangType;

type TSymbolId = u32;

#[derive(Hash, Eq, PartialEq, Clone, Debug, Copy)]
pub struct SymbolId(pub TSymbolId);

#[derive(Debug)]
pub struct SymbolDescriptor {
    pub name: String,
    pub ty: LangType,
    // True if any closure captures it.
    pub is_captured: bool,
}

#[derive(Debug)]
pub struct SymbolTable {
    symbols: HashMap<SymbolId, SymbolDescriptor>,
    next_id: TSymbolId,
}

impl Default for SymbolTable {
    fn default() -> Self {
        Self::new()
    }
}

impl SymbolTable {
    pub fn new() -> Self {
        Self {
            symbols: HashMap::new(),
            next_id: 0,
        }
    }

    pub fn fresh(&mut self, name: String, ty: LangType) -> SymbolId {
        let symbol_id = SymbolId(self.next_id);
        let symbol_descriptor = SymbolDescriptor {
            name,
            ty,
            is_captured: false,
        };

        self.symbols.insert(symbol_id, symbol_descriptor);
        self.next_id += 1;

        symbol_id
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        data_types::{LangPrimitiveType, LangType},
        symbol_table::{SymbolId, SymbolTable},
    };

    #[test]
    fn should_create_empty_table() {
        let table = SymbolTable::new();
        assert_eq!(table.symbols.capacity(), 0);
        assert_eq!(table.next_id, 0);
    }

    #[test]
    fn should_create_new_entry() {
        let mut table = SymbolTable::new();
        assert_eq!(table.symbols.capacity(), 0);
        let id = table.fresh(
            "name".to_string(),
            LangType::Primitive(LangPrimitiveType::Int),
        );
        assert_eq!(id, SymbolId(0));
        assert_eq!(table.next_id, 1);
        let descriptor = table.symbols.get(&id).unwrap();
        assert_eq!(descriptor.name, "name".to_string());
        assert_eq!(descriptor.ty, LangType::Primitive(LangPrimitiveType::Int));
        assert_eq!(descriptor.is_captured, false);
    }
}
