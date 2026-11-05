use std::collections::HashMap;

use elise_shared_types::Span;
use getset::CopyGetters;

use crate::binding_path::BindingPath;

pub type TypeBindingMap = HashMap<BindingPath, TypeBindingDesc>;

/// This data structure is a flat representation of a type annotation.
///
/// For example, let's say we define this type annotation:
///
/// .typedef (User :Record<{
///                         id   :Int
///                         name :Str}>)
///
/// Instead of representing User type as struct with nested nodes,
/// like we do for Ast and AAst nodes, we flatten it into a HashMap
/// where each key is a path to the type descriptor
/// (see TypeBindingMap type above).
///
/// So in this case our User type alias will be associated with the
/// next TypeBinding:
///
/// HashMap {
///     [Root] => Record
///     [Root, Field("id")] => Int
///     [Root, Field("name")] => Str
/// }
///
/// This gives us a better way of type validation, because we don't need
/// to recurse into the nested nodes, and it also simplifies type access.
/// For example, anyone who has a BindingPath, can try to resolve the type
/// by accessing it from this HashMap.
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

// We keep this enum separately from data_bindings
// because data types for data bindings need to carry
// data inside each enum variant.
#[derive(Debug, PartialEq, Clone)]
pub enum TypeBindingDataType {
    Int,
    Str,
    List,
    Record,
}

#[derive(Debug, PartialEq, Clone, CopyGetters)]
pub struct TypeBindingDesc {
    pub dtype: TypeBindingDataType,

    #[getset(get_copy = "pub")]
    pub span: Span,
}

/// Each key is an alias that was created by .typedef function.
/// For example:
///
/// .typedef (User :Record<{
///                         id   :Int
///                         name :Str}>)
///
/// In this case, the key is "User" string and the value is
/// TypeBinding that represents this type alias.
pub type AliasedTypeBindings = HashMap<String, TypeBinding>;
