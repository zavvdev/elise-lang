//! # Binding Table
//!
//! This file defines a common, data agnostic interface
//! for representation of any set of data (csv or json).
//! It encapsulates the process of data validation against
//! its schema and produces a data structure that will
//! allow fast and easier data access during compilation.

use std::collections::HashMap;

use crate::data_types::DataType;

/// Building blocks of the HashMap key.
/// We can represent any data access path
/// using these segments by replicating
/// nesting.
#[derive(Debug, Eq, Hash, PartialEq)]
pub enum PathSegment {
    // We can use index for cases when user iterates
    // over some iterable data and we can track indexes
    // and use them for building a Path key.
    Index(usize),
    // Just a regular string segment such as csv column
    // name or json object property.
    Field(String),
}

/// Key for HashMap in order to access data descriptor.
pub type Path = Vec<PathSegment>;

/// Provides some basic information for the compiler about
/// underlying data.
#[derive(Debug, PartialEq)]
pub struct DataDescriptor {
    // We don't keep parsed values in enum options.
    pub ty: DataType,
    // Instead we just use `value` prop as String type
    // because our values will be serialized into bytecode
    // anyway, so there is no need to parse "true" to bool,
    // "34" to number etc. It's been left for VM implementation
    // details.
    pub value: String,
}

/// Data structure that provides convenient way of accessing data
/// for the compiler. Keep in mind, that this data structure
/// is intended to be used only inside the compiler and it must not
/// be used in any following stage. This is a pure metadata for
/// compiler which will be discarded either after compilation or
/// in process of compilation since compiler is going to build
/// its own constant pool for data access.
#[derive(Debug, PartialEq)]
pub struct DataBindingTable {
    pub table: HashMap<Path, DataDescriptor>,
}

/// Must be implemented for any binder of any data type.
pub trait DataBinder<D, S, E> {
    fn new(data: D, schema: S) -> Self;
    fn bind(&self) -> Result<DataBindingTable, E>;
}
