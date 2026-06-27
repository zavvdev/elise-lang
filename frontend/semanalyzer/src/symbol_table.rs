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
//! It does NOT store values. Values are either folded into AAST nodes
//! as compile-time constants or resolved at runtime by the VM via
//! LOAD_SYM / LOAD_FIELD opcodes.
//!
//! AAST reference nodes store only a SymbolId. When the emitter needs
//! type information for a referenced identifier it looks it up here
//! via that SymbolId rather than traversing the AAST to find the
//! definition node.

use std::collections::HashMap;

use crate::data_types::LangType;

type TSymbolId = u32;

#[derive(Hash, Eq, PartialEq, Clone, Debug, Copy)]
pub struct SymbolId(TSymbolId);

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

        self.symbols.insert(symbol_id.clone(), symbol_descriptor);
        self.next_id += 1;

        symbol_id
    }
}
