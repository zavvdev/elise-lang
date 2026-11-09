use std::collections::HashMap;

use elise_shared_proc_macros::EnumDelegate;
use elise_shared_types::Span;
use getset::CopyGetters;

use crate::binding_path::BindingPath;

pub type DataBindingMap = HashMap<BindingPath, DataBindingDesc>;

/// This data structure is a flat representation of a data.
///
/// For example, let's say we have this data structure:
///
/// { "id" 123, "name" "John" }
///
/// Instead of representing it as dict with nested nodes, like we do
/// for Ast and AAst nodes, we flatten it into a HashMap where each key
/// is a path to the data descriptor (see DataBindingMap type above).
///
/// So in this case our dict will look like this:
///
/// HashMap {
///     [Root] => Record
///     [Root, Field("id")] => 123
///     [Root, Field("name")] => "John"
/// }
///
/// This gives us a better way of type validation, and data access because
/// we don't need to recurse into the nested nodes. For example, anyone who
/// has a BindingPath, can try to get a data by accessing it from this HashMap.
#[derive(Debug, PartialEq, Clone)]
pub struct DataBinding {
    pub map: DataBindingMap,
}

impl Default for DataBinding {
    fn default() -> Self {
        Self::new()
    }
}

impl DataBinding {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }
}

#[derive(Debug, PartialEq, Clone, EnumDelegate)]
#[enum_delegate(
    fn span(&self) -> Span;
)]
pub enum DataBindingDesc {
    Int(DataBindingDescPrim),
    Str(DataBindingDescPrim),
    List(DataBindingDescList),
    Dict(DataBindingDescDict),
    // NOTE: Our current implementation does not support
    // a case when we have a data structure like:
    //
    // {
    //    "id"   123,
    //    "name" .get(@data 0 "name")
    // }
    //
    // where "name" field is bound to a dynamic value,
    // which means that we cannot bind "name" to a data
    // ahead of time. If we decide to support this feature
    // in the future, I think we could add another variant
    // here that holds a reference to AAst node performing
    // a dynamic data access. But since ir/aast already
    // depends on ir/bindings, we might need to introduce
    // something in between in order to avoid circular deps.
}

#[derive(Debug, PartialEq, Clone, CopyGetters)]
pub struct DataBindingDescPrim {
    pub value: String,

    #[getset(get_copy = "pub")]
    pub span: Span,
}

#[derive(Debug, PartialEq, Clone, CopyGetters)]
pub struct DataBindingDescList {
    pub size: Option<usize>,

    #[getset(get_copy = "pub")]
    pub span: Span,
}

#[derive(Debug, PartialEq, Clone, CopyGetters)]
pub struct DataBindingDescDict {
    #[getset(get_copy = "pub")]
    pub span: Span,
}
