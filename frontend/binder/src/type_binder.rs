use std::collections::HashMap;

use elise_aast::{AAstNodeTypedef, AAstNodeTypedefRecordEntries};
use elise_bindings::{
    BindingType, TypeBinding, TypeBindings,
    binding_path::{BindingPath, BindingPathSegment},
};
use elise_shared::{shared_errors::errors_type_binder::TypeBinderErr, shared_types::Span};

pub struct TypeBinder<'a> {
    aast_typedef: &'a AAstNodeTypedef,
    current_path: BindingPath,
    current_type: Option<BindingType>,
    current_span: Span,
    globals: &'a HashMap<String, TypeBindings>,
}

// TODO:
// 1. Semantic analyzer must carry three things:
//    - local_type_bindings which is a HashMap<String, TypeBindings>
//    - local_type_aliases which is a HashMap<String, AAstNodeTypedef>
//    - global_type_bindings which is a HashMap<String, TypeBindings>
//
// 2. Internally, semanalyzer walks AST, when it encounters .typedef
//    it creates AAst node for typedef, adds it into local_type_aliases,
//    then calls TypeBinder in order to bind this type and stores the
//    result into local_type_bindings. When building AAst and encounter
//    custom types, check local_type_bindings and global_type_bindings,
//    if alias is not there - emit error. During building bindings from
//    AAst node (calling this binder) pass a local_type_bindings +
//    global_type_bindings into this binder, so it can build a binding
//    if there is a custom type. So this binder must accept a second arg
//    which is a type of its return type.

impl<'a> TypeBinder<'a> {
    pub fn new(
        aast_typedef: &'a AAstNodeTypedef,
        globals: &'a HashMap<String, TypeBindings>,
    ) -> Self {
        println!("------------globals: {:#?}", globals);
        Self {
            aast_typedef,
            // Current path that changes according to nesting.
            // We push here every time we recurse into nested fields
            // like list items or record keys in order to resolve them.
            current_path: BindingPath::new(),
            // Data type that we're currently in and want to resolve.
            // Whenever we encounter a type definition that we distinguish,
            // we capture it into this field.
            current_type: None,

            current_span: Span { start: 0, end: 0 },

            // Global type bindings injected from outside.
            globals,
        }
    }

    pub fn bind(&mut self) -> Result<TypeBindings, TypeBinderErr> {
        let mut bindings: TypeBindings = HashMap::new();
        self.bind_node(self.aast_typedef, &mut bindings)?;
        Ok(bindings)
    }

    fn bind_node(
        &mut self,
        node: &AAstNodeTypedef,
        bindings: &mut TypeBindings,
    ) -> Result<(), TypeBinderErr> {
        self.current_span = node.span().clone();
        match node {
            AAstNodeTypedef::Custom { alias, .. } => self.bind_custom(alias, bindings),
            AAstNodeTypedef::Record { entries, .. } => self.bind_record(entries, bindings),
            AAstNodeTypedef::List { item_type, .. } => self.bind_list(item_type, bindings),
            AAstNodeTypedef::Int { .. } => self.bind_primitive(BindingType::Int, bindings),
        }
    }

    /// Captures the current state and inserts a new record
    /// into the bindings.
    fn commit(&mut self, bindings: &mut TypeBindings) -> Result<(), TypeBinderErr> {
        if let Some(dtype) = &self.current_type {
            bindings.insert(
                self.current_path.clone(),
                TypeBinding {
                    dtype: dtype.clone(),
                    span: self.current_span.clone(),
                },
            );
            return Ok(());
        }
        Err(TypeBinderErr::UnresolvablePath {
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
        dtype: BindingType,
        bindings: &mut TypeBindings,
    ) -> Result<(), TypeBinderErr> {
        self.current_type = Some(dtype);
        self.commit(bindings)?;

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
    // RECORD START
    // ==================================================================

    fn bind_record(
        &mut self,
        entries: &AAstNodeTypedefRecordEntries,
        bindings: &mut TypeBindings,
    ) -> Result<(), TypeBinderErr> {
        // Capture current type as Record and resolve it right away
        // in order to create a parent entry like:
        // [Root, Field("some")] => TRecord
        // We do this before recursing into tested definitions
        // because resolving nested types will alter current_path
        // state, so commiting parent after resolving recursively
        // will produce invalid path segments to the parent.
        self.current_type = Some(BindingType::Record);
        self.commit(bindings)?;

        for (key, value) in entries {
            // Push new segment into the current_path since we enter a new
            // scope with dict key.
            self.current_path
                .push(BindingPathSegment::Field(key.clone()));
            // Recurse into the key value type definition. This will commit
            // new type definitions with path including the respective key.
            self.bind_node(value, bindings)?;
        }

        self.current_path.pop();
        Ok(())
    }

    // ==================================================================
    // RECORD END
    // ==================================================================

    // ==================================================================
    // LIST START
    // ==================================================================

    /// Lists are monomorphic because schema resolution is a single
    /// deterministic AST walk producing one path -> type entry —
    /// there is no representation for a path resolving
    /// to more than one type.
    fn bind_list(
        &mut self,
        item_type: &AAstNodeTypedef,
        bindings: &mut TypeBindings,
    ) -> Result<(), TypeBinderErr> {
        // Capture current type and commit it before recursing
        // in order to prevent committing parent type with invalid
        // path segments since recursing will alter current_path.
        self.current_type = Some(BindingType::List);
        self.commit(bindings)?;

        // Pushing AbstractIndex since our list can have any number of
        // items of the same type.
        self.current_path.push(BindingPathSegment::AbstractIndex);
        self.bind_node(item_type, bindings)?;

        self.current_path.pop();
        Ok(())
    }

    // ==================================================================
    // LIST END
    // ==================================================================

    // ==================================================================
    // CUSTOM START
    // ==================================================================

    fn bind_custom(
        &mut self,
        alias: &str,
        _bindings: &mut TypeBindings,
    ) -> Result<(), TypeBinderErr> {
        let Some(_global_bindings) = self.globals.get(alias) else {
            return Err(TypeBinderErr::UnknownTypedef {
                span: self.current_span.clone(),
            });
        };

        // TODO

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
