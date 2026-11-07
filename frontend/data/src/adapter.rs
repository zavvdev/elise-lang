//! Data Adapter Trait
//!
//! This trait defines a common adapter interface that each
//! data type must implement in order to be transformed into
//! a valid Elise source code.
//!
//! The idea is that we can translate data regardless of its
//! format (csv, json, etc.) into a source code that can be
//! later used in order to create a data binding. Because of
//! this, our data (such as lists, dicts etc.) that is defined
//! in our source code, can use the same data binder as our
//! data that comes from data file intself.

pub trait DataAdapter<D, E> {
    fn translate(parsed_data: &D) -> Result<Vec<u8>, E>;
}
