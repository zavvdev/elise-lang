//! # SchemaBinder
//!
//! The reason we want to have a schema binder is to be able to
//! build a convenient way of retrieving type information during
//! compilation stage.
//! Schema definition is just a source code with special semantics
//! which means that the result of its parsing is the same AST.
//!
//! This file contains the algorithm of transforming schema AST
//! into the binding table which is represented as a HashMap
//! where each key is a path to the type definition, and value is a
//! type descriptor. This allows us to build a map for each possible
//! path of data access.
//!
//! For example, consider this schema definition:
//! .schema(
//!    .dict(
//!       "name"    .string()
//!       "address" .dict(
//!                    "street" .string()
//!                    "house"  .int())
//!    )
//! )
//!
//! In this case our result will look like this:
//! [Root, Field("name")] => TypeString
//! [Root, Field("address")] => TypeDict
//! [Root, Field("address"), Field("street")] => TypeString
//! [Root, Field("address"), Field("house")] => TypeInt
//!
//! So during compilation when we work with some source code that accesses data
//! like: .get(@data "address" "street"), we can build a resolution path from
//! .get function arguments and access type descriptor.

use std::collections::HashMap;

use elise_ast::{AstCall, AstNode};

use elise_shared::shared_types::ArityMismatchKind;
use elise_shared::{shared_errors::errors_schema_binder::SchemaBinderErr, shared_types::Span};

use elise_shared::shared_node_names::NodeName;

use crate::binding_path::{BindingPath, BindingPathSegment};

// ==================================================================
//
// DATA TYPES START
//
// ==================================================================

#[derive(Debug, PartialEq, Clone)]
pub enum SchemaBinderDataType {
    // We don't carry full type information
    // for compound data types, like dict and
    // lists because:
    // 1. These data structures can have deep
    //    nesting;
    // 2. We only need to know a type of the
    //    data being accessed at the lowest
    //    level, so it's enough to follow the
    //    resolution path in order to get
    //    underlying type descriptor.
    Int,
    Float,
    String,
    Bool,
    ListAbstract,
    ListFixed(usize),
    Dict,
    // Union,
}

impl SchemaBinderDataType {
    pub fn as_str(&self) -> &'static str {
        match self {
            SchemaBinderDataType::Int => NodeName::INT,
            SchemaBinderDataType::Float => NodeName::FLOAT,
            SchemaBinderDataType::String => NodeName::STRING,
            SchemaBinderDataType::Bool => NodeName::BOOL,
            SchemaBinderDataType::ListAbstract => NodeName::LIST,
            SchemaBinderDataType::ListFixed(_) => NodeName::LIST,
            SchemaBinderDataType::Dict => NodeName::DICT,
        }
    }
}

// ==================================================================
//
// DATA TYPES END
//
// ==================================================================

// ==================================================================
//
// FN SETTINGS START
//
// ==================================================================

/// Set of known function calls that are used for
/// type definitions.
pub struct SchemaFnLexeme;
impl SchemaFnLexeme {
    // Top level call. Technically we don't need this
    // but I left it in case we need to provide some
    // specific metadata for schema being defined
    // in the future.
    pub const ROOT: &'static str = "schema";

    // Modifiers.
    pub const NULLABLE: &'static str = "nullable";
    pub const OPTIONAL: &'static str = "optional";

    // Type resolution functions.
    pub const INT: &'static str = "int";
    pub const FLOAT: &'static str = "float";
    pub const STRING: &'static str = "string";
    pub const BOOL: &'static str = "bool";
    pub const DICT: &'static str = "dict";
    pub const LIST: &'static str = "list";
    // pub const UNION: &'static str = "union";
}

/// Argument length requirements for different type
/// definition calls.
pub struct ArgLen;
impl ArgLen {
    pub const ROOT: usize = 1;
    // All primitives don't need any arguments for now.
    // If needed, create a separate variable for each
    // primitive and remove this PRIMITIVE variable.
    pub const PRIMITIVE: usize = 0;

    // Modifiers.
    pub const NULLABLE: usize = 1;
    pub const OPTIONAL: usize = 1;

    pub const LIST: (usize, usize) = (1, 2);

    // pub const UNION_MIN: usize = 2;
}

// ==================================================================
//
// FN SETTINGS END
//
// ==================================================================

// ==================================================================
//
// MODIFIERS START
//
// Modifiers are functions that wrap type definitions in order
// to provide some additional metadata. They can modify inner types
// in a different way. They can be applied either to one direct
// child type, or to all inner nested types. The list of available
// modifiers:
//
// 1. .nullable - makes a type to be nullable, so it can be either
//                NULL or type itself. Ex: .nullable(.int()), so
//                here it's either Int or Null. It applies to
//                the direct child only. If its argument is
//                any compound structure like list or dict,
//                values of these compound structures won't be
//                nullable.
//
// 2. .optional - makes its direct child type definition to be
//                an optional field. Optional is not the same as
//                nullable. Nullable means that field itself
//                exists but its value can be NULL. Optional on the
//                other hand implies that value can be either some
//                type or this field that holds this type can be
//                missing. When optional applied to a type, that
//                type cannot be null. When nullable applied to
//                a type, that type cannot be optional.
//                Optional modifier cannot be applied to a list item
//                type.
//
// ==================================================================

/// Descriptor for modifier itself to provide a
/// settings of its behavior.
#[derive(Debug, PartialEq, Clone)]
struct ModifierDescriptor {
    // Whether we need to apply this modifier for all
    // nested types or only for the direct child.
    // We don't have modifiers that apply to all
    // nested types for now.
    deep: bool,
    // Whether it can be applied again or not.
    // Used together with `deep`. For example, if we
    // have a modifier with `deep=false`, then after
    // applying it we set `active` to `true` therefore
    // this modifier will be skipped for nested nodes.
    active: bool,
}
impl Default for ModifierDescriptor {
    fn default() -> Self {
        Self {
            deep: false,
            active: true,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
enum ModifierKind {
    Nullable,
    Optional,
}

/// Modifier is a special type of schema function
/// that does not produce a type definition but
/// rather provide some metadata for its children.
#[derive(Debug, PartialEq, Clone)]
struct Modifier {
    kind: ModifierKind,
    descriptor: ModifierDescriptor,
}

// ==================================================================
//
// MODIFIERS END
//
// ==================================================================

// ==================================================================
//
// SCHEMA BINDER START
//
// ==================================================================

/// Data type descriptor that is a value each resolution
/// path resolves to.
#[derive(Debug, PartialEq, Clone)]
pub struct SchemaBinderTypeDescriptor {
    pub dtype: SchemaBinderDataType,
    // Either type or NULL.
    pub nullable: bool,
    // Either type or field is missing.
    pub optional: bool,
    pub span: Span,
}
impl SchemaBinderTypeDescriptor {
    pub fn with_defaults(dtype: SchemaBinderDataType) -> Self {
        Self {
            dtype,
            // We set all modifiers to false by default
            // because we need to use apply_modifiers
            // after we create type descriptor.
            nullable: false,
            optional: false,
            span: Span { start: 0, end: 0 },
        }
    }
}

type TSchemaBindings = HashMap<BindingPath, SchemaBinderTypeDescriptor>;

#[derive(Debug, PartialEq)]
pub struct SchemaBindings {
    pub bindings: TSchemaBindings,
}

pub struct SchemaBinder<'a> {
    // AST of the schema definition file.
    schema_ast: &'a Vec<AstNode>,
    current_path: BindingPath,
    current_type: Option<SchemaBinderDataType>,
    current_modifiers: Vec<Modifier>,
    current_span: Span,
}

impl<'a> SchemaBinder<'a> {
    pub fn new(schema_ast: &'a Vec<AstNode>) -> Self {
        Self {
            schema_ast,
            // Current path that changes according to nesting.
            // We push here every time we recurse into nested fields
            // like list items or dict keys in order to resolve them.
            current_path: BindingPath::new(),
            // Data type that we're currently in and want to resolve.
            // Whenever we encounter a type definition that we distinguish,
            // we capture it into this field.
            current_type: None,
            // Track current modifier in order to be able to provide metadata
            // for the type being resolved.
            current_modifiers: vec![],

            current_span: Span { start: 0, end: 0 },
        }
    }

    pub fn bind(&mut self) -> Result<SchemaBindings, SchemaBinderErr> {
        // Do not allow schema to be empty.
        let first_node = self.schema_ast.first().ok_or(SchemaBinderErr::Empty)?;

        // First node must always be a root function call.
        let call = match first_node {
            AstNode::Call(call) if call.lexeme == SchemaFnLexeme::ROOT => call,
            node => {
                return Err(SchemaBinderErr::Unexp {
                    span: node.span().clone(),
                });
            }
        };

        match call.children.len() {
            ArgLen::ROOT => {
                let root_node = call.children.first().unwrap();
                let mut bindings: TSchemaBindings = HashMap::new();
                self.resolve_from_node(root_node, &mut bindings)?;
                Ok(SchemaBindings { bindings })
            }
            args_len => Err(SchemaBinderErr::ArityMismatch {
                fn_name: SchemaFnLexeme::ROOT,
                kind: ArityMismatchKind::Eq(ArgLen::ROOT),
                found: args_len,
                span: call.span.clone(),
            }),
        }
    }

    /// Main function for resolving type from AST nodes.
    fn resolve_from_node(
        &mut self,
        node: &AstNode,
        bindings: &mut TSchemaBindings,
    ) -> Result<(), SchemaBinderErr> {
        self.current_span = node.span().clone();
        match node {
            AstNode::Call(call) => match call.lexeme.as_str() {
                SchemaFnLexeme::NULLABLE => {
                    self.resolve_modifier(ModifierKind::Nullable, call, bindings)
                }
                SchemaFnLexeme::OPTIONAL => {
                    self.resolve_modifier(ModifierKind::Optional, call, bindings)
                }
                SchemaFnLexeme::INT => self.resolve_primitive(
                    call,
                    SchemaBinderDataType::Int,
                    SchemaFnLexeme::INT,
                    bindings,
                ),
                SchemaFnLexeme::FLOAT => self.resolve_primitive(
                    call,
                    SchemaBinderDataType::Float,
                    SchemaFnLexeme::FLOAT,
                    bindings,
                ),
                SchemaFnLexeme::STRING => self.resolve_primitive(
                    call,
                    SchemaBinderDataType::String,
                    SchemaFnLexeme::STRING,
                    bindings,
                ),
                SchemaFnLexeme::BOOL => self.resolve_primitive(
                    call,
                    SchemaBinderDataType::Bool,
                    SchemaFnLexeme::BOOL,
                    bindings,
                ),
                SchemaFnLexeme::DICT => self.resolve_dict(call, bindings),
                SchemaFnLexeme::LIST => self.resolve_list(call, bindings),
                // SchemaFnLexeme::UNION => self.resolve_union(call, bindings),
                _ => Err(SchemaBinderErr::InvalTypeDef {
                    span: call.span.clone(),
                }),
            },
            node => Err(SchemaBinderErr::InvalTypeDef {
                span: node.span().clone(),
            }),
        }
    }

    fn appy_modifiers(&mut self, type_descriptor: &mut SchemaBinderTypeDescriptor) {
        // Iterate over mutable modifiers since we need to update
        // descriptor in case we need to.
        for modifier in self.current_modifiers.iter_mut() {
            if !modifier.descriptor.active {
                continue;
            }
            match modifier.kind {
                ModifierKind::Nullable => {
                    type_descriptor.nullable = true;
                }
                ModifierKind::Optional => {
                    type_descriptor.optional = true;
                }
            }
            if !modifier.descriptor.deep {
                modifier.descriptor.active = false;
            }
        }
    }

    /// Captures the current state and inserts a new record into the
    /// bindings.
    fn commit(&mut self, bindings: &mut TSchemaBindings) -> Result<(), SchemaBinderErr> {
        if let Some(dtype) = &self.current_type {
            let mut type_descriptor = SchemaBinderTypeDescriptor::with_defaults(dtype.clone());
            self.appy_modifiers(&mut type_descriptor);
            type_descriptor.span = self.current_span.clone();
            bindings.insert(self.current_path.clone(), type_descriptor);
            return Ok(());
        }
        Err(SchemaBinderErr::UnresolvablePath {
            path: self.current_path.as_str(),
        })
    }

    // ==================================================================
    // MODIFIERS START
    // ==================================================================

    fn resolve_modifier(
        &mut self,
        modifier_kind: ModifierKind,
        call: &AstCall,
        bindings: &mut TSchemaBindings,
    ) -> Result<(), SchemaBinderErr> {
        let result = match modifier_kind {
            ModifierKind::Nullable => self.resolve_modifier_nullable(call, bindings),
            ModifierKind::Optional => self.resolve_modifier_optional(call, bindings),
        };
        self.current_modifiers.clear();
        result
    }

    /// Provides nullable metadata for nested type definitions.
    /// Does not create/remove any path segments from the current_path
    /// after resolution.
    fn resolve_modifier_nullable(
        &mut self,
        call: &AstCall,
        bindings: &mut TSchemaBindings,
    ) -> Result<(), SchemaBinderErr> {
        let args_len = call.children.len();

        if args_len != ArgLen::NULLABLE {
            return Err(SchemaBinderErr::ArityMismatch {
                fn_name: SchemaFnLexeme::NULLABLE,
                kind: ArityMismatchKind::Eq(ArgLen::NULLABLE),
                found: args_len,
                span: call.span.clone(),
            });
        }

        let descriptor = ModifierDescriptor {
            // Nullable modifier is not supposed to be a deep modifier.
            deep: false,
            ..ModifierDescriptor::default()
        };

        self.current_modifiers.push(Modifier {
            kind: ModifierKind::Nullable,
            descriptor,
        });

        self.resolve_from_node(call.children.first().unwrap(), bindings)?;

        // We do not pop path segment after resolving nullable since it's just a modifier.
        Ok(())
    }

    fn resolve_modifier_optional(
        &mut self,
        call: &AstCall,
        bindings: &mut TSchemaBindings,
    ) -> Result<(), SchemaBinderErr> {
        let args_len = call.children.len();

        if args_len != ArgLen::OPTIONAL {
            return Err(SchemaBinderErr::ArityMismatch {
                fn_name: SchemaFnLexeme::OPTIONAL,
                kind: ArityMismatchKind::Eq(ArgLen::OPTIONAL),
                found: args_len,
                span: call.span.clone(),
            });
        }

        if let Some(ty) = &self.current_type
            && (matches!(ty, &SchemaBinderDataType::ListFixed(..))
                || ty == &SchemaBinderDataType::ListAbstract)
        {
            return Err(SchemaBinderErr::InvalUseOfModifier {
                span: call.span.clone(),
            });
        }

        let descriptor = ModifierDescriptor {
            deep: false,
            ..ModifierDescriptor::default()
        };

        self.current_modifiers.push(Modifier {
            kind: ModifierKind::Optional,
            descriptor,
        });

        self.resolve_from_node(call.children.first().unwrap(), bindings)?;
        Ok(())
    }

    // ==================================================================
    // MODIFIERS END
    // ==================================================================

    // ==================================================================
    // PRIMITIVES START
    // ==================================================================

    /// We use the same function for all primitives since they all
    /// adhere to the same semantics.
    fn resolve_primitive(
        &mut self,
        call: &AstCall,
        dtype: SchemaBinderDataType,
        lexeme: &'static str,
        bindings: &mut TSchemaBindings,
    ) -> Result<(), SchemaBinderErr> {
        let args_len = call.children.len();
        self.current_type = Some(dtype);

        if args_len > 0 {
            return Err(SchemaBinderErr::ArityMismatch {
                fn_name: lexeme,
                kind: ArityMismatchKind::Eq(ArgLen::PRIMITIVE),
                found: args_len,
                span: call.span.clone(),
            });
        }

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
    // DICT START
    // ==================================================================

    fn resolve_dict(
        &mut self,
        call: &AstCall,
        bindings: &mut TSchemaBindings,
    ) -> Result<(), SchemaBinderErr> {
        let args_len = call.children.len();

        // TODO: Might not be correct to enforce these semantics
        // here, but I don't know how to solve it for now.
        // Maybe we could run semantic analysis for schema AST
        // before type resolution?
        if !args_len.is_multiple_of(2) || args_len == 0 {
            return Err(SchemaBinderErr::InvalDict {
                span: call.span.clone(),
            });
        }

        // Capture current type as Dict and resolve it right away
        // in order to create a parent entry like:
        // [Root, Field("some")] => TDict
        // We do this before recursing into tested definitions
        // because resolving nested types will alter current_path
        // state, so commiting parent after resolving recursively
        // will produce invalid path segments to the parent.
        self.current_type = Some(SchemaBinderDataType::Dict);
        self.commit(bindings)?;

        // Since we know that the number of arguments is even, then
        // odd elements are keys, and even elements are values (type
        // definitions).
        let keys: Vec<_> = call.children.iter().step_by(2).collect();
        let values: Vec<_> = call.children.iter().skip(1).step_by(2).collect();

        let mut index = 0;

        while index < keys.len() {
            let key = *keys.get(index).unwrap();
            let value = *values.get(index).unwrap();

            match &**key {
                // TODO: Same here..
                // Probably needs to be enforced during some premature
                // semantic analysis.
                AstNode::String(prim) => {
                    // Push new segment into the current_path since we enter a new
                    // scope with dict key.
                    self.current_path
                        .push(BindingPathSegment::Field(prim.value.clone()));
                    // Recurse into the key value type definition. This will commit
                    // new type definitions with path including the respective key.
                    self.resolve_from_node(value, bindings)?;
                }
                node => {
                    return Err(SchemaBinderErr::InvalDict {
                        span: node.span().clone(),
                    });
                }
            };

            index += 1;
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

    fn resolve_list_size(ast_node: &AstNode) -> Result<usize, SchemaBinderErr> {
        match ast_node {
            AstNode::Int(prim) => {
                let size: usize = prim.value.parse().unwrap();
                Ok(size)
            }
            node => Err(SchemaBinderErr::UndexpType {
                expected: NodeName::INT.to_string(),
                found: node.as_str().to_string(),
                span: node.span().clone(),
            }),
        }
    }

    /// Lists are monomorphic because schema resolution is a single
    /// deterministic AST walk producing one path -> type entry —
    /// there is no representation for a path resolving
    /// to more than one type.
    fn resolve_list(
        &mut self,
        call: &AstCall,
        bindings: &mut TSchemaBindings,
    ) -> Result<(), SchemaBinderErr> {
        let args_len = call.children.len();

        if args_len > ArgLen::LIST.1 || args_len < ArgLen::LIST.0 {
            return Err(SchemaBinderErr::ArityMismatch {
                fn_name: SchemaFnLexeme::LIST,
                kind: ArityMismatchKind::Range(ArgLen::LIST),
                found: args_len,
                span: call.span.clone(),
            });
        }

        let first_arg = call.children.first().unwrap();
        let mut list_size: Option<usize> = None;

        if call.children.len() == ArgLen::LIST.1 {
            let size_arg = call.children.last().unwrap();
            list_size = Some(Self::resolve_list_size(size_arg)?);
        }

        // Capture current type and commit it before recursing
        // in order to prevent committing parent type with invalid
        // path segments since recursing will alter current_path.
        if let Some(size) = list_size {
            self.current_type = Some(SchemaBinderDataType::ListFixed(size));
        } else {
            self.current_type = Some(SchemaBinderDataType::ListAbstract);
        }
        self.commit(bindings)?;

        // Pusing AbstractIndex since our list can have any number of
        // items of the same type.
        self.current_path.push(BindingPathSegment::AbstractIndex);
        self.resolve_from_node(first_arg, bindings)?;

        self.current_path.pop();
        Ok(())
    }

    // ==================================================================
    // LIST END
    // ==================================================================

    // ==================================================================
    // UNION START
    // ==================================================================

    // "case3" .list(.union(
    //                    .list(.dict(
    //                            "some"  .int()
    //                            "some2" .float()
    //                    )),
    //                    .list(.dict(
    //                            "some3" .string()
    //                            "some"  .list(.int())
    //                    ))))
    //                    .list(.dict(
    //                            "some3" .float()
    //                            "some"  .list(.string())
    //                            "other" .string()
    //                    ))))
    //
    // [Root] -> Dict

    // [Root, "case3"] -> ListAbstract

    // HashMap {
    //     (Root, "case3", AbstractIndex, AbstractIndex) => Dict,
    //     (Root, "case3", AbstractIndex, AbstractIndex, Field("some")) => Int,
    //     (Root, "case3", AbstractIndex, AbstractIndex, Field("some2")) => Float,
    // }

    // HashMap {
    //     (Root, "case3", AbstractIndex, AbstractIndex) => Dict
    //     (Root, "case3", AbstractIndex, AbstractIndex, Field("some")) => List
    //     (Root, "case3", AbstractIndex, AbstractIndex, Field("some"), AbstractIndex) => Int
    //     (Root, "case3", AbstractIndex, AbstractIndex, Field("some3")) => String
    // }

    // Remove common key-value pairs and insert them into main:

    // HashMap {
    //     (Root, "case3", AbstractIndex, AbstractIndex, Field("some")) => Int,
    //     (Root, "case3", AbstractIndex, AbstractIndex, Field("some2")) => Float,
    // }

    // HashMap {
    //     (Root, "case3", AbstractIndex, AbstractIndex, Field("some")) => List
    //     (Root, "case3", AbstractIndex, AbstractIndex, Field("some"), AbstractIndex) => Int
    //     (Root, "case3", AbstractIndex, AbstractIndex, Field("some3")) => String
    // }

    // HashMap {
    //     (Root, "case3", AbstractIndex, AbstractIndex, Field("some")) => List
    //     (Root, "case3", AbstractIndex, AbstractIndex, Field("some"), AbstractIndex) => String
    //     (Root, "case3", AbstractIndex, AbstractIndex, Field("some3")) => Float
    //     (Root, "case3", AbstractIndex, AbstractIndex, Field("other")) => String
    // }

    // 1. Create a HashMap<key, [usize; N]> where N is a number of tables;
    // 2. Iterate over each table key:
    //    if key is not in hashmap, then insert it with key => [0, 0, 0]
    //    and increment the number on the index of the current table by one;
    //    if key is inside the hashmap, then increment the number on the index
    //    of the current table by one;
    //
    // This gives us information about how many keys are present inside each table and how
    // many times.

    // Needs to be transformed into:
    //
    // HashMap {
    //     (Root, "case3", AbstractIndex, AbstractIndex) => Dict,
    //     (Root, "case3", AbstractIndex, AbstractIndex, Field("some"), Assertion(Int)) => Int
    //     (Root, "case3", AbstractIndex, AbstractIndex, Field("some"), Assertion(List)) => List
    //     (Root, "case3", AbstractIndex, AbstractIndex, Field("some2")) => Float
    //     (Root, "case3", AbstractIndex, AbstractIndex, Field("some"), Assertion(List), AbstractIndex, Assertion(Int)) => Int
    //     (Root, "case3", AbstractIndex, AbstractIndex, Field("some"), Assertion(List), AbstractIndex, Assertion(String)) => String
    //     (Root, "case3", AbstractIndex, AbstractIndex, Field("some3")) => String
    // }

    // fn resolve_union(
    //     &mut self,
    //     call: &AstCall,
    //     bindings: &mut TSchemaBindings,
    // ) -> Result<(), SchemaBinderErr> {
    //     let args_len = call.children.len();

    //     if args_len < ArgLen::UNION_MIN {
    //         return Err(SchemaBinderErr::ArityMismatch {
    //             fn_name: SchemaFnLexeme::UNION,
    //             kind: ArityMismatchKind::MoreEq(ArgLen::UNION_MIN),
    //             found: args_len,
    //             span: call.span.clone(),
    //         });
    //     }

    //     // Disallow direct usage of union inside the union.
    //     for child in &call.children {
    //         if let AstNode::Call(inner) = &**child
    //             && inner.lexeme == SchemaFnLexeme::UNION
    //         {
    //             return Err(SchemaBinderErr::NoUnionOfUnion {
    //                 span: inner.span.clone(),
    //             });
    //         }
    //     }

    //     // Capture global state at the moment of the branches resolution start
    //     // since each of the branches needs to start its own resolution from
    //     // the same state.
    //     let captured_path = self.current_path.clone();
    //     let captured_modifiers = self.current_modifiers.clone();

    //     // Main {
    //     //     (Root) => Dict
    //     //     (Root, "case3") => List
    //     // }
    //     //
    //     // [
    //     //     HashMap {
    //     //         (Root, "case3", Index) => List
    //     //         (Root, "case3", Index, Index) => Dict
    //     //         (Root, "case3", Index, Index, "some") => Int
    //     //         (Root, "case3", Index, Index, "some2") => Float
    //     //     }
    //     //
    //     //     HashMap {
    //     //        (Root, "case3", Index) => List
    //     //        (Root, "case3", Index, Index) => Dict
    //     //        (Root, "case3", Index, Index, "some") => List
    //     //        (Root, "case3", Index, Index, "some", Index) => Int
    //     //        (Root, "case3", Index, Index, "some3") => String
    //     //     }
    //     // ]

    //     // New resolution table for each Union branch.
    //     let mut resolution_tables: Vec<TSchemaBindings> = Vec::with_capacity(call.children.len());

    //     for child in &call.children {
    //         // TODO: Works but cloning on each iteration again is not good.
    //         // Although there is less likely to be a case where we have
    //         // hundreds of Union branches, but I think we need to check
    //         // if we can find another solution here.
    //         self.current_path = captured_path.clone();
    //         self.current_modifiers = captured_modifiers.clone();
    //         let mut table: TSchemaBindings = HashMap::new();
    //         self.resolve_from_node(child, &mut table)?;
    //         resolution_tables.push(table);
    //     }

    //     // Remove the same key-value pairs from all union branch tables
    //     // only if each key-value pair is present in each table.

    //     let smallest_table_index = resolution_tables
    //         .iter()
    //         .enumerate()
    //         // Enumerate gives us a tuple with (index, map), min_by_key
    //         // returns the item that yields the smallest number in predicate.
    //         .min_by_key(|(_, map)| map.len())
    //         // Extract only index.
    //         .map(|(i, _)| i)
    //         .unwrap();

    //     // We start from the smallest table since we want to remove the key-value pairs
    //     // that are present in ALL union branch tables, so the subset of all potential
    //     // entries is a smallest one.
    //     let smallest_table = &resolution_tables[smallest_table_index];

    //     // Key-value pairs that are present in ALL union branch tables.
    //     let common_key_values: Vec<(BindingPath, SchemaBinderTypeDescriptor)> = smallest_table
    //         .iter()
    //         .filter(|(key, value)| {
    //             resolution_tables
    //                 .iter()
    //                 .enumerate()
    //                 // Skip smallest table since we're already iterating over its values.
    //                 .filter(|(index, _)| *index != smallest_table_index)
    //                 // Tests if all elements (tables) match the predicate.
    //                 // In our case, each table must have the same key-value in order to be removed
    //                 // and inserted into the main resolution table.
    //                 .all(|(_, table)| table.get(*key) == Some(*value))
    //         })
    //         .map(|(key, value)| (key.clone(), value.clone()))
    //         .collect();

    //     // Remove common key-value pairs from all Union branch tables
    //     // and insert them into the main resolution table since there is
    //     // no type ambiguity in this case.
    //     for (key, value) in common_key_values {
    //         for table in resolution_tables.iter_mut() {
    //             table.remove(&key);
    //         }
    //         bindings.insert(key, value);
    //     }

    //     //let _without_common_prefix = Self::remove_common_path_segments(&resolution_tables);
    //     println!("-------- Children tables: {:#?}", resolution_tables);

    //     Ok(())
    // }

    // ==================================================================
    // UNION END
    // ==================================================================
}

// ==================================================================
//
// SCHEMA BINDER END
//
// ==================================================================
