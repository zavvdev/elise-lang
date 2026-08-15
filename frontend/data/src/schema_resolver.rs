//! # SchemaResolver
//!
//! The reason we want to have a schema resolution is to be able to
//! build a convenient way of retrieving type information during
//! compilation stage.
//! Schema definition is just a source code with special semantics
//! which means that the result of its parsing is the same AST.
//!
//! This file contains the algorithm of transforming schema AST
//! into resolved schema which is represented as a HashMap
//! where each key is a resolution path, and value is a type descriptor.
//! This allows us to build a map for each possible path of data access.
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
//! In this case our resolved schema will look like this:
//! [Root, Field("name")] => TypeString
//! [Root, Field("address")] => TypeDict
//! [Root, Field("address"), Field("street")] => TypeString
//! [Root, Field("address"), Field("house")] => TypeInt
//!
//! So during compilation when we work with some source code that accesses data
//! like: .get(@data "address" "street"), we can build a resolution path from
//! .get function arguments and access type descriptor from this SchemaResolver
//! result.

use std::collections::HashMap;

use elise_ast::{AstCall, AstNode};

use elise_shared::shared_errors::errors_schema_resolver::SchemaResolverErr;
use elise_shared::shared_types::ArityMismatchKind;

use elise_shared::shared_node_names::NodeName;

use crate::resolution_path::{ResolutionPath, ResolutionPathSegment};

// ==================================================================
//
// DATA TYPES START
//
// ==================================================================

#[derive(Debug, PartialEq, Clone)]
pub enum SchemaDataType {
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
    Union,
}
// TODO: Check if we need this.
impl SchemaDataType {
    pub fn as_str(&self) -> &'static str {
        match self {
            SchemaDataType::Int => NodeName::INT,
            SchemaDataType::Float => NodeName::FLOAT,
            SchemaDataType::String => NodeName::STRING,
            SchemaDataType::Bool => NodeName::BOOL,
            SchemaDataType::ListAbstract => NodeName::LIST,
            SchemaDataType::ListFixed(_) => NodeName::LIST,
            SchemaDataType::Dict => NodeName::DICT,
            SchemaDataType::Union => NodeName::UNION,
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
    pub const UNION: &'static str = "union";
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

    pub const UNION_MIN: usize = 2;
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
#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq)]
enum ModifierKind {
    Nullable,
    Optional,
}

/// Modifier is a special type of schema function
/// that does not produce a type definition but
/// rather provide some metadata for its children.
#[derive(Debug, PartialEq)]
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
// SCHEMA RESOLVER START
//
// ==================================================================

/// Data type descriptor that is a value each resolution
/// path resolves to.
#[derive(Debug, PartialEq)]
pub struct SchemaTypeDescriptor {
    pub dtype: SchemaDataType,
    // Either type or NULL.
    pub nullable: bool,
    // Either type or field is missing.
    pub optional: bool,
}
impl SchemaTypeDescriptor {
    pub fn with_defaults(dtype: SchemaDataType) -> Self {
        Self {
            dtype,
            // We set all modifiers to false by default
            // because we need to use apply_modifiers
            // after we create type descriptor.
            nullable: false,
            optional: false,
        }
    }
}

type TResolvedSchema = HashMap<ResolutionPath, SchemaTypeDescriptor>;

#[derive(Debug, PartialEq)]
pub struct ResolvedSchema {
    pub resolved_schema: TResolvedSchema,
}

pub struct SchemaResolver<'a> {
    // AST of the schema definition file.
    schema_ast: &'a Vec<AstNode>,
    current_path: ResolutionPath,
    current_type: Option<SchemaDataType>,
    current_modifiers: Vec<Modifier>,
}

impl<'a> SchemaResolver<'a> {
    pub fn new(schema_ast: &'a Vec<AstNode>) -> Self {
        Self {
            schema_ast,
            // Current path that changes according to nesting.
            // We push here every time we recurse into nested fields
            // like list items or dict keys in order to resolve them.
            current_path: ResolutionPath::new(),
            // Data type that we're currently in and want to resolve.
            // Whenever we encounter a type definition that we distinguish,
            // we capture it into this field.
            current_type: None,
            // Track current modifier in order to be able to provide metadata
            // for the type being resolved.
            current_modifiers: vec![],
        }
    }

    pub fn resolve(&mut self) -> Result<ResolvedSchema, SchemaResolverErr> {
        // Do not allow schema to be empty.
        let first_node = self.schema_ast.first().ok_or(SchemaResolverErr::Empty)?;

        // First node must always be a root function call.
        let call = match first_node {
            AstNode::Call(call) if call.lexeme == SchemaFnLexeme::ROOT => call,
            node => {
                return Err(SchemaResolverErr::Unexp {
                    span: node.span().clone(),
                });
            }
        };

        match call.children.len() {
            ArgLen::ROOT => {
                let root_node = call.children.first().unwrap();
                let mut resolved_schema: TResolvedSchema = HashMap::new();
                self.resolve_from_node(root_node, &mut resolved_schema)?;
                Ok(ResolvedSchema { resolved_schema })
            }
            args_len => Err(SchemaResolverErr::ArityMismatch {
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
        resolved_schema: &mut TResolvedSchema,
    ) -> Result<(), SchemaResolverErr> {
        match node {
            AstNode::Call(call) => match call.lexeme.as_str() {
                SchemaFnLexeme::NULLABLE => {
                    self.resolve_modifier(ModifierKind::Nullable, call, resolved_schema)
                }
                SchemaFnLexeme::OPTIONAL => {
                    self.resolve_modifier(ModifierKind::Optional, call, resolved_schema)
                }
                SchemaFnLexeme::INT => self.resolve_primitive(
                    call,
                    SchemaDataType::Int,
                    SchemaFnLexeme::INT,
                    resolved_schema,
                ),
                SchemaFnLexeme::FLOAT => self.resolve_primitive(
                    call,
                    SchemaDataType::Float,
                    SchemaFnLexeme::FLOAT,
                    resolved_schema,
                ),
                SchemaFnLexeme::STRING => self.resolve_primitive(
                    call,
                    SchemaDataType::String,
                    SchemaFnLexeme::STRING,
                    resolved_schema,
                ),
                SchemaFnLexeme::BOOL => self.resolve_primitive(
                    call,
                    SchemaDataType::Bool,
                    SchemaFnLexeme::BOOL,
                    resolved_schema,
                ),
                SchemaFnLexeme::DICT => self.resolve_dict(call, resolved_schema),
                SchemaFnLexeme::LIST => self.resolve_list(call, resolved_schema),
                SchemaFnLexeme::UNION => self.resolve_union(call, resolved_schema),
                _ => Err(SchemaResolverErr::InvalTypeDef {
                    span: call.span.clone(),
                }),
            },
            node => Err(SchemaResolverErr::InvalTypeDef {
                span: node.span().clone(),
            }),
        }
    }

    fn appy_modifiers(&mut self, type_descriptor: &mut SchemaTypeDescriptor) {
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
    /// resolved schema.
    fn commit(&mut self, resolved_schema: &mut TResolvedSchema) -> Result<(), SchemaResolverErr> {
        if let Some(dtype) = &self.current_type {
            let mut type_descriptor = SchemaTypeDescriptor::with_defaults(dtype.clone());
            self.appy_modifiers(&mut type_descriptor);
            resolved_schema.insert(self.current_path.clone(), type_descriptor);
            return Ok(());
        }
        Err(SchemaResolverErr::UnresolvablePath {
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
        resolved_schema: &mut TResolvedSchema,
    ) -> Result<(), SchemaResolverErr> {
        let result = match modifier_kind {
            ModifierKind::Nullable => self.resolve_modifier_nullable(call, resolved_schema),
            ModifierKind::Optional => self.resolve_modifier_optional(call, resolved_schema),
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
        resolved_schema: &mut TResolvedSchema,
    ) -> Result<(), SchemaResolverErr> {
        let args_len = call.children.len();

        if args_len != ArgLen::NULLABLE {
            return Err(SchemaResolverErr::ArityMismatch {
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

        self.resolve_from_node(call.children.first().unwrap(), resolved_schema)?;

        // We do not pop path segment after resolving nullable since it's just a modifier.
        Ok(())
    }

    fn resolve_modifier_optional(
        &mut self,
        call: &AstCall,
        resolved_schema: &mut TResolvedSchema,
    ) -> Result<(), SchemaResolverErr> {
        let args_len = call.children.len();

        if args_len != ArgLen::OPTIONAL {
            return Err(SchemaResolverErr::ArityMismatch {
                fn_name: SchemaFnLexeme::OPTIONAL,
                kind: ArityMismatchKind::Eq(ArgLen::OPTIONAL),
                found: args_len,
                span: call.span.clone(),
            });
        }

        if let Some(ty) = &self.current_type
            && (matches!(ty, &SchemaDataType::ListFixed(..)) || ty == &SchemaDataType::ListAbstract)
        {
            return Err(SchemaResolverErr::InvalUseOfModifier {
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

        self.resolve_from_node(call.children.first().unwrap(), resolved_schema)?;
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
        dtype: SchemaDataType,
        lexeme: &'static str,
        resolved_schema: &mut TResolvedSchema,
    ) -> Result<(), SchemaResolverErr> {
        let args_len = call.children.len();
        self.current_type = Some(dtype);

        if args_len > 0 {
            return Err(SchemaResolverErr::ArityMismatch {
                fn_name: lexeme,
                kind: ArityMismatchKind::Eq(ArgLen::PRIMITIVE),
                found: args_len,
                span: call.span.clone(),
            });
        }

        self.commit(resolved_schema)?;
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
        resolved_schema: &mut TResolvedSchema,
    ) -> Result<(), SchemaResolverErr> {
        let args_len = call.children.len();

        // TODO: Might not be correct to enforce these semantics
        // here, but I don't know how to solve it for now.
        // Maybe we could run semantic analysis for schema AST
        // before type resolution?
        if !args_len.is_multiple_of(2) || args_len == 0 {
            return Err(SchemaResolverErr::InvalDict {
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
        self.current_type = Some(SchemaDataType::Dict);
        self.commit(resolved_schema)?;

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
                        .push(ResolutionPathSegment::Field(prim.value.clone()));
                    // Recurse into the key value type definition. This will commit
                    // new type definitions with path including the respective key.
                    self.resolve_from_node(value, resolved_schema)?;
                }
                node => {
                    return Err(SchemaResolverErr::InvalDict {
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

    fn resolve_list_size(ast_node: &AstNode) -> Result<usize, SchemaResolverErr> {
        match ast_node {
            AstNode::Int(prim) => {
                let size: usize = prim.value.parse().unwrap();
                Ok(size)
            }
            node => Err(SchemaResolverErr::UndexpType {
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
        resolved_schema: &mut TResolvedSchema,
    ) -> Result<(), SchemaResolverErr> {
        let args_len = call.children.len();

        if args_len > ArgLen::LIST.1 || args_len < ArgLen::LIST.0 {
            return Err(SchemaResolverErr::ArityMismatch {
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
            self.current_type = Some(SchemaDataType::ListFixed(size));
        } else {
            self.current_type = Some(SchemaDataType::ListAbstract);
        }
        self.commit(resolved_schema)?;

        // Pusing AbstractIndex since our list can have any number of
        // items of the same type.
        self.current_path.push(ResolutionPathSegment::AbstractIndex);
        self.resolve_from_node(first_arg, resolved_schema)?;

        self.current_path.pop();
        Ok(())
    }

    // ==================================================================
    // LIST END
    // ==================================================================

    // ==================================================================
    // UNION START
    // ==================================================================

    fn resolve_union(
        &mut self,
        call: &AstCall,
        _resolved_schema: &mut TResolvedSchema,
    ) -> Result<(), SchemaResolverErr> {
        let args_len = call.children.len();

        if args_len < ArgLen::UNION_MIN {
            return Err(SchemaResolverErr::ArityMismatch {
                fn_name: SchemaFnLexeme::UNION,
                kind: ArityMismatchKind::MoreEq(ArgLen::UNION_MIN),
                found: args_len,
                span: call.span.clone(),
            });
        }

        // Disallow direct usage of union inside the union.
        for child in &call.children {
            if let AstNode::Call(inner) = &**child
                && inner.lexeme == SchemaFnLexeme::UNION
            {
                return Err(SchemaResolverErr::NoUnionOfUnion {
                    span: inner.span.clone(),
                });
            }
        }

        Ok(())
    }

    // ==================================================================
    // UNION END
    // ==================================================================
}

// ==================================================================
//
// SCHEMA RESOLVER END
//
// ==================================================================
