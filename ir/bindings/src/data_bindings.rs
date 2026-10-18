use std::collections::HashMap;

use elise_shared::shared_types::Locatable;

use crate::{binding_path::BindingPath, common::BindingType};

pub struct DataBinding<L: Locatable> {
    pub map: HashMap<BindingPath, DataBindingDesc<L>>,
}

pub struct DataBindingDesc<L: Locatable> {
    pub dtype: BindingType,
    pub value: String,
    pub location: L,
}
