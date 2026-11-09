//! Data Binder
//!
//! This data structure allows us to transform AAst node
//! that represents a data like { "id" 123, "name" "John" }
//! into a flat data structure called DataBinding (see ir/bindings).
//!
//! Data AAst node itself is a nested data structure which needs to be
//! flattened in order to simplify data validation and data accessing.

use elise_ir_aast::aast_node_data::AAstNodeData;
use elise_ir_bindings::{
    binding_path::BindingPath,
    data_bindings::{DataBinding, DataBindingDesc},
};
use elise_shared_errors::errors_data_binder::DataBinderErr;
use elise_shared_types::Span;

pub struct DataBinder<'a> {
    aast_data_node: &'a AAstNodeData,
    current_path: BindingPath,
    current_desc: Option<DataBindingDesc>,
    current_span: Span,
}

impl<'a> DataBinder<'a> {
    pub fn new(aast_data_node: &'a AAstNodeData) -> Self {
        Self {
            // Data node itself that needs to be flattened.
            aast_data_node,

            // Current path that is being bind. Enriched with
            // path segments during the process and gets commited
            // into the resulting binding HashMap when ready.
            current_path: BindingPath::new(),

            // Data descriptor that we're currently in and want to resolve.
            // Whenever we encounter a data aast node that we distinguish,
            // we capture it into this field.
            current_desc: None,

            current_span: Span { start: 0, end: 0 },
        }
    }

    pub fn bind(&mut self) -> Result<DataBinding, DataBinderErr> {
        let binding: DataBinding = DataBinding::new();
        println!("{:#?}", self.aast_data_node);
        println!("{:#?}", self.current_path);
        println!("{:#?}", self.current_desc);
        println!("{:#?}", self.current_span);
        //self.bind_node(self.aast_data_node, &mut binding)?;
        Ok(binding)
    }

    //fn bind_node(
    //    &mut self,
    //    node: &AAstNodeTypeAnnot,
    //    binding: &mut TypeBinding,
    //) -> Result<(), TypeBinderErr> {
    //    self.current_span = node.span();
    //    match node {
    //        AAstNodeTypeAnnot::Custom(AAstNodeTypeAnnotCustom { alias, .. }) => {
    //            self.bind_custom(alias, binding)
    //        }
    //        AAstNodeTypeAnnot::Record(AAstNodeTypeAnnotRecord { entries, .. }) => {
    //            self.bind_record(entries, binding)
    //        }
    //        AAstNodeTypeAnnot::List(AAstNodeTypeAnnotList { item_type, .. }) => {
    //            self.bind_list(item_type, binding)
    //        }
    //        AAstNodeTypeAnnot::Int { .. } => self.bind_primitive(TypeBindingDataType::Int, binding),
    //        AAstNodeTypeAnnot::Str { .. } => self.bind_primitive(TypeBindingDataType::Str, binding),
    //    }
    //}

    ///// Captures the current state and inserts a new record
    ///// into the bindings.
    //fn commit(&mut self, binding: &mut TypeBinding) -> Result<(), TypeBinderErr> {
    //    if let Some(dtype) = &self.current_type {
    //        binding.map.insert(
    //            self.current_path.clone(),
    //            TypeBindingDesc {
    //                dtype: dtype.clone(),
    //                span: self.current_span,
    //            },
    //        );
    //        return Ok(());
    //    }
    //    Err(TypeBinderErr::UnresolvablePath {
    //        path: self.current_path.as_str(),
    //    })
    //}

    //// ==================================================================
    //// PRIMITIVES START
    //// ==================================================================

    ///// We use the same function for all primitives since they all
    ///// adhere to the same semantics.
    //fn bind_primitive(
    //    &mut self,
    //    dtype: TypeBindingDataType,
    //    binding: &mut TypeBinding,
    //) -> Result<(), TypeBinderErr> {
    //    self.current_type = Some(dtype);
    //    self.commit(binding)?;

    //    // We always remove the last path segment after resolving primitives
    //    // regardless if they nested or not, because if they are nested,
    //    // then it removes nested path segment which is correct. If they are not
    //    // nested, which means they are top level type definition, then this will
    //    // be noop because we can't remove Root segment from Path.
    //    self.current_path.pop();

    //    Ok(())
    //}

    //// ==================================================================
    //// PRIMITIVES END
    //// ==================================================================

    //// ==================================================================
    //// RECORD START
    //// ==================================================================

    //fn bind_record(
    //    &mut self,
    //    entries: &AAstNodeTypeAnnotRecordEntries,
    //    binding: &mut TypeBinding,
    //) -> Result<(), TypeBinderErr> {
    //    // Capture current type as Record and resolve it right away
    //    // in order to create a parent entry like:
    //    // [Root, Field("some")] => TRecord
    //    // We do this before recursing into tested definitions
    //    // because resolving nested types will alter current_path
    //    // state, so commiting parent after resolving recursively
    //    // will produce invalid path segments to the parent.
    //    self.current_type = Some(TypeBindingDataType::Record);
    //    self.commit(binding)?;

    //    for (key, value) in entries {
    //        // Push new segment into the current_path since we enter a new
    //        // scope with dict key.
    //        self.current_path
    //            .push(BindingPathSegment::Field(key.clone()));
    //        // Recurse into the key value type definition. This will commit
    //        // new type definitions with path including the respective key.
    //        self.bind_node(value, binding)?;
    //    }

    //    self.current_path.pop();
    //    Ok(())
    //}

    //// ==================================================================
    //// RECORD END
    //// ==================================================================

    //// ==================================================================
    //// LIST START
    //// ==================================================================

    ///// Lists are monomorphic because schema resolution is a single
    ///// deterministic AST walk producing one path -> type entry —
    ///// there is no representation for a path resolving
    ///// to more than one type.
    //fn bind_list(
    //    &mut self,
    //    item_type: &AAstNodeTypeAnnot,
    //    binding: &mut TypeBinding,
    //) -> Result<(), TypeBinderErr> {
    //    // Capture current type and commit it before recursing
    //    // in order to prevent committing parent type with invalid
    //    // path segments since recursing will alter current_path.
    //    self.current_type = Some(TypeBindingDataType::List);
    //    self.commit(binding)?;

    //    // Pushing AbstractIndex since our list can have any number of
    //    // items of the same type.
    //    self.current_path.push(BindingPathSegment::AbstractIndex);
    //    self.bind_node(item_type, binding)?;

    //    self.current_path.pop();
    //    Ok(())
    //}

    //// ==================================================================
    //// LIST END
    //// ==================================================================

    //// ==================================================================
    //// CUSTOM START
    //// ==================================================================

    //fn bind_custom(&mut self, alias: &str, binding: &mut TypeBinding) -> Result<(), TypeBinderErr> {
    //    let Some(global_bindings) = self.globals.get(alias) else {
    //        return Err(TypeBinderErr::Unresolvable {
    //            span: self.current_span,
    //        });
    //    };

    //    for (global_binding_path, global_binding) in global_bindings.map.iter() {
    //        let Some(next_path) = BindingPath::prepend(&self.current_path, global_binding_path)
    //        else {
    //            return Err(TypeBinderErr::UnableToMerge {
    //                span: global_binding.span(),
    //            });
    //        };
    //        binding.map.insert(next_path, global_binding.clone());
    //    }

    //    Ok(())
    //}

    // ==================================================================
    // LIST END
    // ==================================================================
}

// ==================================================================
//
// BINDER END
//
// ==================================================================
