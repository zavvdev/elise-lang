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
