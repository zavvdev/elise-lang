//! # Data Binder
//!
//! This file defines a common, data agnostic interface
//! for representation of any set of data (csv or json).

use std::collections::HashMap;

use crate::resolution_path::ResolutionPath;

#[derive(PartialEq, Debug, Clone)]
pub enum DataBinderDataType {
    Int,
    Float,
    String,
    Bool,
    Null,
    // Since we support only CSV for now, we skip these
    // List,
    // Dict,
}

#[derive(Debug, PartialEq)]
pub struct DataBinderDataDescriptor {
    pub ty: DataBinderDataType,
    pub value: String,
}

#[derive(Debug, PartialEq)]
pub struct DataBindingTable {
    // TODO:
    // Carry Selector {
    //          data_resolution_path: ResolutionPath,
    //          type_resolution_path: Option<ResolutionPath>
    //       }
    // as a key where the former is for accessing data, and the latter
    // is for type resolution. For example, we can create a data resolution
    // path with Index(usize) but our schema resolution uses
    // abstract indexes which means we can't use data accessor path
    // for type validation. We need to carry both.
    pub table: HashMap<ResolutionPath, DataBinderDataDescriptor>,
}

/// Must be implemented for any binder of any data type.
pub trait DataBinder<'a, D, E> {
    fn new(data: &'a D) -> Self;
    fn bind(&self) -> Result<DataBindingTable, E>;
}
