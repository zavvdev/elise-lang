use std::collections::HashMap;

type TSymbolId = u32;

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
pub struct SymbolId(TSymbolId);

#[derive(Debug)]
pub struct SymbolDescriptor {
    pub name: String,
    pub ty: String, // TODO: Update to enum.
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

    pub fn fresh(&mut self, name: String, ty: String) -> SymbolId {
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
