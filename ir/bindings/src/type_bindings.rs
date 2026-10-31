use std::collections::HashMap;

use elise_shared_types::Span;
use getset::CopyGetters;

use crate::{
    binding_path::{BindingPath, BindingPathSegment},
    common::BindingType,
};

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

    /// Tries to convert binding entry into a BindingPathSegment
    /// if a data type carried by the descriptor matches what path
    /// segment expects.
    /// BindingPath can be built from Int and Str that represent
    /// index and access field respectively. If our TypeBinding
    /// has only one record and that record points to either of
    /// these data types, it means that we can use this data type
    /// as a BindingPathSegment and insert it into the `binding_path`.
    ///
    /// It's used when we have a function that has its return type
    /// represented as TypeBinding, and the result of this function
    /// is used as path segment. For example:
    /// .get(@data, .get(@data, 0, "id"))
    /// In this case, nested .get function return type is TypeBinding
    /// and in order to build a binding path of the outer .get function
    /// we need to resolve the result of the inner into a BindingPathSegment.
    /// So we call this function on the return type which augments our
    /// `binding_path` that is being built.
    // TODO: Add tests for this function.
    pub fn insert_as_binding_path_segment(&self, binding_path: &mut BindingPath) -> Option<()> {
        let mut iter = self.map.iter();
        let (path, desc) = iter.next()?;
        if iter.next().is_some() {
            return None;
        }

        let segment = match desc.dtype {
            BindingType::Int => BindingPathSegment::AbstractIndex,
            BindingType::Str => BindingPathSegment::Field(path.last_seg_field()?),
            _ => return None,
        };

        binding_path.push(segment);
        Some(())
    }
}

#[derive(Debug, PartialEq, Clone, CopyGetters)]
pub struct TypeBindingDesc {
    pub dtype: BindingType,

    #[getset(get_copy = "pub")]
    pub span: Span,
}

pub type AliasedTypeBindings = HashMap<String, TypeBinding>;
