use std::collections::HashMap;

use elise_shared::shared_types::Locatable;

use crate::binding_path::BindingPath;

pub struct DataBindings<L: Locatable> {
    pub bindings: HashMap<BindingPath, DataBindingDesc<L>>,
}

pub struct DataBindingDesc<L: Locatable> {
    pub dtype: DataBindingDataType,
    pub value: String,
    pub location: L,
}

pub enum DataBindingDataType {
    Int,
}
