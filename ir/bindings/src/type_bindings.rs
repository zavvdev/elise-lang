use std::collections::HashMap;

use elise_shared::shared_types::Span;

use crate::binding_path::BindingPath;

pub type TypeAliasTable = HashMap<String, TypeBindings>;
pub type TypeBindings = HashMap<BindingPath, TypeBindingDesc>;

pub struct TypeBindingDesc {
    pub dtype: TypeBindingDataType,
    pub span: Span,
}

pub enum TypeBindingDataType {
    Int,
}
