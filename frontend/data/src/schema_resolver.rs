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
    Null,
    List,
    Dict,
}

// TODO: Check if we need this.
impl SchemaDataType {
    pub fn as_str(&self) -> &'static str {
        match self {
            SchemaDataType::Int => NodeName::INT,
            SchemaDataType::Float => NodeName::FLOAT,
            SchemaDataType::String => NodeName::STRING,
            SchemaDataType::Bool => NodeName::BOOL,
            SchemaDataType::Null => NodeName::NULL,
            SchemaDataType::List => NodeName::LIST,
            SchemaDataType::Dict => NodeName::DICT,
        }
    }
}

/// Set of known function calls that are used for
/// type definitions.
pub struct SchemaFnLexeme;
impl SchemaFnLexeme {
    // Top level call. Technically we don't need this
    // but I left it in case we need to provide some
    // specific metadata for schema being defined
    // in the future.
    pub const ROOT: &'static str = "schema";
    pub const INT: &'static str = "int";
    pub const FLOAT: &'static str = "float";
    pub const STRING: &'static str = "string";
    pub const BOOL: &'static str = "bool";
    pub const NULLABLE: &'static str = "nullable";
    pub const DICT: &'static str = "dict";
    pub const LIST: &'static str = "list";
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
    pub const NULLABLE: usize = 1;
    // We support only a list of one data type for now.
    pub const LIST: usize = 1;
}

/// Data type descriptor that is a value each resolution
/// path resolves to.
#[derive(Debug, PartialEq)]
pub struct SchemaTypeDescriptor {
    pub dtype: SchemaDataType,
    pub nullable: bool,
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
    current_nullable: bool,
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
            // Whenever we enter .nullable type we set it to true in order
            // to make all nested types nullable, since if parent is nullable,
            // then accessing any nested items might give you null value.
            current_nullable: false,
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
                expected: ArgLen::ROOT,
                kind: ArityMismatchKind::Eq,
                found: args_len,
                span: call.span.clone(),
            }),
        }
    }

    /// Captures the current state and inserts a new record into the
    /// resolved schema.
    fn commit(&mut self, resolved_schema: &mut TResolvedSchema) -> Result<(), SchemaResolverErr> {
        if let Some(dtype) = &self.current_type {
            resolved_schema.insert(
                self.current_path.clone(),
                SchemaTypeDescriptor {
                    dtype: dtype.clone(),
                    nullable: self.current_nullable,
                },
            );
            return Ok(());
        }
        Err(SchemaResolverErr::UnresolvablePath {
            path: self.current_path.as_str(),
        })
    }

    /// Main function for resolving type from AST nodes.
    fn resolve_from_node(
        &mut self,
        node: &AstNode,
        resolved_schema: &mut TResolvedSchema,
    ) -> Result<(), SchemaResolverErr> {
        match node {
            AstNode::Call(call) => match call.lexeme.as_str() {
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
                SchemaFnLexeme::NULLABLE => self.resolve_modifier_nullable(call, resolved_schema),
                SchemaFnLexeme::DICT => self.resolve_dict(call, resolved_schema),
                SchemaFnLexeme::LIST => self.resolve_list(call, resolved_schema),
                _ => Err(SchemaResolverErr::InvalTypeDef {
                    span: call.span.clone(),
                }),
            },
            node => Err(SchemaResolverErr::InvalTypeDef {
                span: node.span().clone(),
            }),
        }
    }

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
                expected: ArgLen::PRIMITIVE,
                kind: ArityMismatchKind::Eq,
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
                expected: ArgLen::NULLABLE,
                kind: ArityMismatchKind::Eq,
                found: args_len,
                span: call.span.clone(),
            });
        }

        // Set nullable flag to true before recursing deeper.
        self.current_nullable = true;

        // After this stage, all nested type definitions will be
        // resolved as nullable. For now this is intentional since
        // accessing any data of the parent might give you null because
        // parent might not be accessible.
        self.resolve_from_node(call.children.first().unwrap(), resolved_schema)?;

        // Reset nullable state after we out of nullable definition scope.
        self.current_nullable = false;

        // We do not pop path segment after resolving nullable since it's just a modifier.
        Ok(())
    }

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

    fn resolve_list(
        &mut self,
        call: &AstCall,
        resolved_schema: &mut TResolvedSchema,
    ) -> Result<(), SchemaResolverErr> {
        let args_len = call.children.len();

        if args_len != ArgLen::LIST {
            return Err(SchemaResolverErr::ArityMismatch {
                fn_name: SchemaFnLexeme::LIST,
                expected: ArgLen::LIST,
                kind: ArityMismatchKind::Eq,
                found: args_len,
                span: call.span.clone(),
            });
        }

        let first_arg = call.children.first().unwrap();

        // Capture current type and commit it before recursing
        // in order to prevent committing parent type with invalid
        // path segments since recursing will alter current_path.
        self.current_type = Some(SchemaDataType::List);
        self.commit(resolved_schema)?;

        // Pusing AbstractIndex since our list can have any number of
        // items of the same type.
        self.current_path.push(ResolutionPathSegment::AbstractIndex);
        self.resolve_from_node(first_arg, resolved_schema)?;

        self.current_path.pop();
        Ok(())
    }
}
