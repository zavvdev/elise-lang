//! # Binding Table
//!
//! This file defines a common, data agnostic interface
//! for representation of any set of data (csv or json).

use std::collections::HashMap;

use crate::resolution_path::ResolutionPath;

pub enum BinderDataType {
    Int,
    Float,
    String,
    Null,
    // Since we support only CSV for now, we skip these
    // List,
    // Dict,
}

/// Provides some basic information for about
/// underlying data.
#[derive(Debug, PartialEq)]
pub struct DataDescriptor {
    pub ty: BinderDataType,
    pub value: String,
    // How can we represent position if we can have json as well?
}

#[derive(Debug, PartialEq)]
pub struct DataBindingTable {
    pub table: HashMap<ResolutionPath, DataDescriptor>,
}

/// Must be implemented for any binder of any data type.
pub trait DataBinder<D, E> {
    fn new(data: D) -> Self;
    fn bind(&self) -> Result<DataBindingTable, E>;
}
