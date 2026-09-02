//! # Data Binder
//!
//! This file defines a common, data agnostic interface
//! for representation of any set of data (csv or json).
//!
//! The main idea is that our schema resolution table
//! and data binding table have HashTable's with shared
//! keys that are represented as ResolutionPath. Because of
//! that, we can validate data type in O(1) just by
//! extracting type information from resolved schema by
//! the same key from the binding table.

use std::collections::HashMap;

use crate::resolution_path::ResolutionPath;

/// Union of types that are available for all
/// types of data like csv or json.
#[derive(PartialEq, Debug, Clone)]
pub enum DataBinderDataType {
    Int,
    Float,
    String,
    Bool,
    Null,
    // Since we support only CSV for now, we skip these,
    // but once we start supporting JSON or any other data
    // type that has lists and dictionaries, they will be
    // included.

    // List,
    // Dict,
}

/// Data structure that describes the data itself as it's
/// represented in the source file. Type information in this
/// struct is not considered as a source of truth and must not
/// be used as a proof for type validity.
#[derive(Debug, PartialEq)]
pub struct DataBinderDataDescriptor<P> {
    // Data type derived from parser.
    // This type is what we have inside the source file.
    pub ty: DataBinderDataType,
    // Literal value from the source file.
    pub value: String,

    // Generic descriptor for the position of the
    // carried value. Can be different for each
    // data types.
    pub pos_descriptor: P,

    // The idea behind ResolutionPath is that we use it as a
    // single source of truth for data access and type validation.
    // When we construct schema resolution table from schema AST,
    // our path to type descriptors uses AbstractIndex for lists
    // since our lists can include only one data type for now and
    // each element regardless of its index resolves to the same
    // type.
    // But when we construct binding table, we derive it from the
    // actual data where we have access to indexes in lists,
    // so we must use Index(usize) in order to insert into table
    // and we can't use AbstractIndex here since we can have key
    // collisions. So keys of the binding table carry the path for
    // resolving data itself, and type_resolution_path carries
    // the path for resolving type for that data from resolved
    // schema in order to prove it during validation stage.
    pub type_resolution_path: ResolutionPath,
}

#[derive(Debug, PartialEq)]
pub struct DataBindingTable<P> {
    pub table: HashMap<ResolutionPath, DataBinderDataDescriptor<P>>,
}

/// Must be implemented for any binder of any data type.
pub trait DataBinder<'a, Data, Error, PosDescriptor> {
    fn new(data: &'a Data) -> Self;
    fn bind(&self) -> Result<DataBindingTable<PosDescriptor>, Error>;
}
