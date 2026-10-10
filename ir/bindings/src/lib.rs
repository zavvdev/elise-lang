pub mod binding_path;

use std::collections::HashMap;

use elise_shared::shared_types::{Locatable, Span};

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

pub type TypeBinding = HashMap<BindingPath, TypeBindingDesc>;
pub type TypeBindingsMap = HashMap<String, TypeBinding>;

#[derive(Debug, PartialEq, Clone)]
pub struct TypeBindingDesc {
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

pub struct DataBinding<L: Locatable> {
    pub binding: HashMap<BindingPath, DataBindingDesc<L>>,
}

pub struct DataBindingDesc<L: Locatable> {
    pub dtype: BindingType,
    pub value: String,
    pub location: L,
}

// ==================================================================
//
// DATA BINDINGS END
//
// ==================================================================
