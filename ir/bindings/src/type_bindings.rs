use std::collections::HashMap;

use elise_shared_proc_macros::HasSpan;
use elise_shared_types::Span;

use crate::{binding_path::BindingPath, common::BindingType};

pub type TypeBindingMap = HashMap<BindingPath, TypeBindingDesc>;

#[derive(Debug, PartialEq, Clone)]
pub struct TypeBinding {
    pub map: TypeBindingMap,
}

impl Default for TypeBinding {
    fn default() -> Self {
        Self::new()
    }
}

impl TypeBinding {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    pub fn with(map: &TypeBindingMap) -> Self {
        Self { map: map.clone() }
    }

    /// Removes `binding_path_prefix` from each path segment inside self.map.
    /// Used for building return types of functions that access some data and
    /// returns a new set of data that is accesses.
    ///
    /// Ex:
    /// .get(@data, 0)
    /// Function .get accesses data from the @data slot on index 0, so the return
    /// type must be a type binding where Root segment is in place of 0.
    pub fn strip_binding_paths_by_prefix(&self, binding_path_prefix: &BindingPath) -> Self {
        let mut new_map = HashMap::new();

        for (key, value) in self.map.iter() {
            if let Some(next_key) = key.strip(binding_path_prefix) {
                new_map.insert(next_key, value.clone());
            } else {
                continue;
            }
        }

        Self { map: new_map }
    }
}

#[derive(Debug, PartialEq, Clone, HasSpan)]
pub struct TypeBindingDesc {
    pub dtype: BindingType,
    pub span: Span,
}

pub type AliasedTypeBindings = HashMap<String, TypeBinding>;
