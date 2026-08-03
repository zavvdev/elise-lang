use std::collections::HashMap;

use elise_ast::{AstCall, AstNode};

use elise_shared::shared_errors::errors_schema_resolver::SchemaResolverErr;
use elise_shared::shared_types::ArityMismatchKind;

use elise_shared::shared_node_names::NodeName;

use crate::data_resolution_path::{ResolutionPath, ResolutionPathSegment};

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
    schema_ast: &'a Vec<AstNode>,
    current_path: ResolutionPath,
    current_type: Option<SchemaDataType>,
    current_nullable: bool,
}

impl<'a> SchemaResolver<'a> {
    pub fn new(schema_ast: &'a Vec<AstNode>) -> Self {
        Self {
            schema_ast,
            current_path: ResolutionPath::new(),
            current_type: None,
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
                return Err(SchemaResolverErr::UnexpCall {
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

    fn resolve_from_node(
        &mut self,
        node: &AstNode,
        resolved_schema: &mut TResolvedSchema,
    ) -> Result<(), SchemaResolverErr> {
        let result = match node {
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
                SchemaFnLexeme::NULLABLE => self.resolve_nullable(call, resolved_schema),
                SchemaFnLexeme::DICT => self.resolve_dict(call, resolved_schema),
                SchemaFnLexeme::LIST => self.resolve_list(call, resolved_schema),
                _ => Err(SchemaResolverErr::InvalTypeDef {
                    span: call.span.clone(),
                }),
            },
            node => {
                return Err(SchemaResolverErr::InvalTypeDef {
                    span: node.span().clone(),
                });
            }
        };

        self.current_path.pop();
        result
    }

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

        self.commit(resolved_schema)
    }

    fn resolve_nullable(
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

        if self.current_nullable {
            return Err(SchemaResolverErr::NullableNullable {
                span: call.span.clone(),
            });
        }

        self.current_nullable = true;
        self.resolve_from_node(call.children.first().unwrap(), resolved_schema)?;
        self.current_nullable = false;

        Ok(())
    }

    fn resolve_dict(
        &mut self,
        call: &AstCall,
        resolved_schema: &mut TResolvedSchema,
    ) -> Result<(), SchemaResolverErr> {
        let args_len = call.children.len();

        if !args_len.is_multiple_of(2) || args_len == 0 {
            return Err(SchemaResolverErr::InvalDict {
                span: call.span.clone(),
            });
        }

        self.current_type = Some(SchemaDataType::Dict);
        self.commit(resolved_schema)?;

        let keys: Vec<_> = call.children.iter().step_by(2).collect();
        let values: Vec<_> = call.children.iter().skip(1).step_by(2).collect();

        let mut index = 0;

        while index < keys.len() {
            let key = *keys.get(index).unwrap();
            let value = *values.get(index).unwrap();

            match &**key {
                AstNode::String(prim) => {
                    self.current_path
                        .push(ResolutionPathSegment::Field(prim.value.clone()));
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

        self.current_type = Some(SchemaDataType::List);
        self.commit(resolved_schema)?;
        self.current_path.push(ResolutionPathSegment::AbstractIndex);
        self.resolve_from_node(first_arg, resolved_schema)?;

        Ok(())
    }
}
