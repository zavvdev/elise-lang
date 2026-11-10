//! Data Binder
//!
//! This data structure allows us to transform AAst node
//! that represents a data like { "id" 123, "name" "John" }
//! into a flat data structure called DataBinding (see ir/bindings).
//!
//! Data AAst node itself is a nested data structure which needs to be
//! flattened in order to simplify data validation and data accessing.

use elise_ir_aast::aast_node_data::{
    AAstNodeData, AAstNodeDataDict, AAstNodeDataList, AAstNodeDataPrim,
};
use elise_ir_bindings::{
    binding_path::{BindingPath, BindingPathSegment},
    data_bindings::{
        DataBinding, DataBindingDesc, DataBindingDescDict, DataBindingDescList, DataBindingDescPrim,
    },
};
use elise_shared_errors::errors_data_binder::DataBinderErr;

pub struct DataBinder<'a> {
    aast_data_node: &'a AAstNodeData,
    current_path: BindingPath,
    current_desc: Option<DataBindingDesc>,
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
        }
    }

    pub fn bind(&mut self) -> Result<DataBinding, DataBinderErr> {
        let mut binding: DataBinding = DataBinding::new();
        self.bind_node(self.aast_data_node, &mut binding)?;
        Ok(binding)
    }

    fn bind_node(
        &mut self,
        node: &AAstNodeData,
        binding: &mut DataBinding,
    ) -> Result<(), DataBinderErr> {
        match node {
            AAstNodeData::Int(prim) => self.bind_primitive(DataBindingDesc::Int, prim, binding),
            AAstNodeData::Str(prim) => self.bind_primitive(DataBindingDesc::Str, prim, binding),
            AAstNodeData::Dict(dict) => self.bind_dict(dict, binding),
            AAstNodeData::List(list) => self.bind_list(list, binding),
        }
    }

    /// Captures the current state and inserts a new record
    /// into the bindings.
    fn commit(&mut self, binding: &mut DataBinding) -> Result<(), DataBinderErr> {
        if let Some(desc) = &self.current_desc {
            binding.map.insert(self.current_path.clone(), desc.clone());
            return Ok(());
        }
        Err(DataBinderErr::UnresolvablePath {
            path: self.current_path.as_str(),
        })
    }

    // ==================================================================
    // PRIMITIVES START
    // ==================================================================

    /// We use the same function for all primitives since they all
    /// adhere to the same semantics.
    fn bind_primitive(
        &mut self,
        desc: fn(DataBindingDescPrim) -> DataBindingDesc,
        aast_prim: &AAstNodeDataPrim,
        binding: &mut DataBinding,
    ) -> Result<(), DataBinderErr> {
        self.current_desc = Some(desc(DataBindingDescPrim {
            value: aast_prim.lexeme(),
            span: aast_prim.span(),
        }));
        self.commit(binding)?;

        // We always remove the last path segment after resolving primitives
        // regardless if they nested or not, because if they are nested,
        // then it removes nested path segment which is correct. If they are not
        // nested, which means they are top level type definition, then this will
        // be noop because we can't remove Root segment from Path.
        self.current_path.pop();

        Ok(())
    }

    // ==================================================================
    // PRIMITIVES END
    // ==================================================================

    // ==================================================================
    // DICT START
    // ==================================================================

    fn bind_dict(
        &mut self,
        aast_dict: &AAstNodeDataDict,
        binding: &mut DataBinding,
    ) -> Result<(), DataBinderErr> {
        // Capture current descriptor as Dict and resolve it right away
        // in order to create a parent entry like:
        // [Root, Field("some")] => Dict
        // We do this before recursing into tested entries
        // because resolving nested ones will alter current_path
        // state, so committing parent after resolving recursively
        // will produce invalid path segments to the parent.
        self.current_desc = Some(DataBindingDesc::Dict(DataBindingDescDict {
            span: aast_dict.span(),
        }));
        self.commit(binding)?;

        for (key, value) in &aast_dict.entries {
            // Push new segment into the current_path since we enter a new
            // scope with dict key.
            self.current_path
                .push(BindingPathSegment::Field(key.lexeme()));
            // Recurse into the value. This will commit
            // new descriptors with path including the respective key.
            self.bind_node(value, binding)?;
        }

        self.current_path.pop();
        Ok(())
    }

    // ==================================================================
    // DICT END
    // ==================================================================

    // ==================================================================
    // LIST START
    // ==================================================================

    /// Lists are monomorphic because schema resolution is a single
    /// deterministic AST walk producing one path -> type entry —
    /// there is no representation for a path resolving
    /// to more than one data type.
    fn bind_list(
        &mut self,
        aast_list: &AAstNodeDataList,
        binding: &mut DataBinding,
    ) -> Result<(), DataBinderErr> {
        // Capture current descriptor and commit it before recursing
        // in order to prevent committing parent with invalid
        // path segments since recursing will alter current_path.
        self.current_desc = Some(DataBindingDesc::List(DataBindingDescList {
            size: Some(aast_list.items.len()),
            span: aast_list.span(),
        }));
        self.commit(binding)?;

        for (idx, item) in aast_list.items.iter().enumerate() {
            self.current_path.push(BindingPathSegment::Index(idx));
            self.bind_node(item, binding)?;
            self.current_path.pop();
        }

        Ok(())
    }

    // ==================================================================
    // LIST END
    // ==================================================================
}

// ==================================================================
//
// BINDER END
//
// ==================================================================
