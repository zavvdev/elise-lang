pub mod binding_path;
pub mod binding_types;

use std::collections::HashMap;

use elise_shared::shared_types::{Locatable, Span};

use crate::{binding_path::BindingPath, binding_types::BindingType};

pub type TypeAliases = HashMap<String, TypeBindings>;
pub type TypeBindings = HashMap<BindingPath, TypeBindingDesc>;

pub struct TypeBindingDesc {
    pub dtype: BindingType,
    pub span: Span,
}

pub struct DataBindings<L: Locatable> {
    pub bindings: HashMap<BindingPath, DataBindingDesc<L>>,
}

pub struct DataBindingDesc<L: Locatable> {
    pub dtype: BindingType,
    pub value: String,
    pub location: L,
}
