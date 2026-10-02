pub mod binding_path;

use std::collections::HashMap;

use elise_shared::{
    shared_node_names::NodeName,
    shared_types::{Locatable, Span},
};

use crate::binding_path::BindingPath;

// ==================================================================
//
// COMMON START
//
// ==================================================================

#[derive(Debug, PartialEq, Clone)]
pub enum BindingType {
    Int,
    List,
    Record,
}

impl BindingType {
    pub fn as_str(&self) -> &'static str {
        match self {
            BindingType::Int => NodeName::INT,
            BindingType::List => NodeName::LIST,
            BindingType::Record => NodeName::RECORD,
        }
    }
}

// ==================================================================
//
// COMMON END
//
// ==================================================================

// ==================================================================
//
// TYPE BINDINGS START
//
// ==================================================================

pub type TypeBindings = HashMap<BindingPath, TypeBinding>;

#[derive(Debug, PartialEq, Clone)]
pub struct TypeBinding {
    pub dtype: BindingType,
    pub span: Span,
}

// ==================================================================
//
// TYPE BINDINGS END
//
// ==================================================================

// ==================================================================
//
// DATA BINDINGS START
//
// ==================================================================

pub struct DataBindings<L: Locatable> {
    pub bindings: HashMap<BindingPath, DataBinding<L>>,
}

pub struct DataBinding<L: Locatable> {
    pub dtype: BindingType,
    pub value: String,
    pub location: L,
}

// ==================================================================
//
// DATA BINDINGS END
//
// ==================================================================
