// - We start from empty key buffer Vec [];
//
// - When we encounter type definition we push Root PathSegment
//   into the buffer and insert it as key to resolved type;
//
// - If resolved type was primitive, we pop from buffer;
//
// - When we encounter compound type like list or dict,
//   we do not pop from buffer until we resolve inner types,
//   which allows us to build keys with prefix of prev buffer;
//
// - This gives us an ability to re-define fields which we want,
//   but for root we limit it deliberately to 1, although we could
//   just leave it.
//
// [Root] -> TDict
//
// [Root, Field("name")] -> TString
//
// [Root, Field("age")] -> TInt
//
// [Root, Field("employed")] -> TBool
//
// [Root, Field("nicknames")] -> TListOf(TString)
//
// [Root, Field("nicknames"), AbstractIndex] -> TString
//
// [Root, Field("address")] -> TDict
//
// [Root, Field("address"), Field("street")] -> TString
//
// [Root, Field("address"), Field("town")] -> TString
//
// [Root, Field("address"), Field("indexes")] -> TListOf(TInt)
//
// [Root, Field("address"), Field("some")] -> TList(TInt, TBool)
//
// [Root, Field("address"), Field("some"), Index(0)] -> TInt
//
// [Root, Field("address"), Field("some"), Index(1)] -> TBool
//
// .get("address", "indexes", 0)

use std::collections::HashMap;

use elise_ast::{AstCall, AstNode};

use elise_shared::shared_errors::errors_schema_resolver::{
    SchemaResolverErr, SchemaResolverErr::*,
};
use elise_shared::shared_types::{ArityMismatchKind, Span};

use crate::data_config::{OPT_ARGS_LEN, PRIMITIVE_ARGS_LEN, ROOT_ARGS_LEN};
use crate::data_types::{DataType, ResolutionPath};
use crate::data_types::{ResolutionPathSegment, SchemaFnLexeme};

#[derive(Debug, PartialEq)]
pub struct TypeDescriptor {
    pub dtype: DataType,
    pub nullable: bool,
}

type TResolvedSchema = HashMap<ResolutionPath, TypeDescriptor>;

#[derive(Debug, PartialEq)]
pub struct ResolvedSchema {
    pub resolved_schema: TResolvedSchema,
}

pub struct SchemaResolver<'a> {
    schema_ast: &'a Vec<AstNode>,
    current_path: ResolutionPath,
    current_type: Option<DataType>,
    current_nullable: bool,
}

impl<'a> SchemaResolver<'a> {
    pub fn new(schema_ast: &'a Vec<AstNode>) -> Self {
        Self {
            schema_ast,
            current_path: vec![ResolutionPathSegment::Root],
            current_type: None,
            current_nullable: false,
        }
    }

    pub fn resolve(&mut self) -> Result<ResolvedSchema, SchemaResolverErr> {
        let first_node = self.schema_ast.first().ok_or_else(|| InvalRoot {
            span: Span { start: 1, end: 1 },
        })?;

        let call = match first_node {
            AstNode::Call(call) if call.lexeme == SchemaFnLexeme::ROOT => call,
            node => {
                return Err(InvalRoot {
                    span: node.span().clone(),
                });
            }
        };

        match call.children.len() {
            ROOT_ARGS_LEN => {
                let root_node = call.children.first().unwrap();
                let mut resolved_schema: TResolvedSchema = HashMap::new();
                self.resolve_from_node(root_node, &mut resolved_schema)?;
                Ok(ResolvedSchema { resolved_schema })
            }
            args_len => {
                return Err(SchemaResolverErr::ArityMismatch {
                    fn_name: SchemaFnLexeme::ROOT,
                    expected: ROOT_ARGS_LEN,
                    kind: ArityMismatchKind::Eq,
                    found: args_len,
                    span: call.span.clone(),
                });
            }
        }
    }

    fn commit(&mut self, resolved_schema: &mut TResolvedSchema) -> Result<(), SchemaResolverErr> {
        if let Some(dtype) = &self.current_type {
            resolved_schema.insert(
                self.current_path.clone(),
                TypeDescriptor {
                    dtype: dtype.clone(),
                    nullable: self.current_nullable,
                },
            );
            return Ok(());
        }
        Err(SchemaResolverErr::Todo("Commit".to_string()))
    }

    fn backtrack(&mut self) {
        if self.current_path.len() > 1 {
            self.current_path.pop();
        }
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
                    DataType::Int,
                    SchemaFnLexeme::INT,
                    resolved_schema,
                ),
                SchemaFnLexeme::FLOAT => self.resolve_primitive(
                    call,
                    DataType::Float,
                    SchemaFnLexeme::FLOAT,
                    resolved_schema,
                ),
                SchemaFnLexeme::STRING => self.resolve_primitive(
                    call,
                    DataType::String,
                    SchemaFnLexeme::STRING,
                    resolved_schema,
                ),
                SchemaFnLexeme::BOOL => self.resolve_primitive(
                    call,
                    DataType::Bool,
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
                return Err(InvalTypeDef {
                    span: node.span().clone(),
                });
            }
        };

        self.backtrack();
        result
    }

    fn resolve_primitive(
        &mut self,
        call: &AstCall,
        dtype: DataType,
        lexeme: &'static str,
        resolved_schema: &mut TResolvedSchema,
    ) -> Result<(), SchemaResolverErr> {
        let args_len = call.children.len();
        self.current_type = Some(dtype);

        if args_len > 0 {
            return Err(SchemaResolverErr::ArityMismatch {
                fn_name: lexeme,
                expected: PRIMITIVE_ARGS_LEN,
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

        if args_len != OPT_ARGS_LEN {
            return Err(SchemaResolverErr::ArityMismatch {
                fn_name: SchemaFnLexeme::NULLABLE,
                expected: OPT_ARGS_LEN,
                kind: ArityMismatchKind::Eq,
                found: args_len,
                span: call.span.clone(),
            });
        }

        if self.current_nullable {
            return Err(SchemaResolverErr::Todo("optional of optional".to_string()));
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
            return Err(SchemaResolverErr::Todo("dict args not even".to_string()));
        }

        self.current_type = Some(DataType::Dict);
        self.commit(resolved_schema)?;

        let keys: Vec<_> = call.children.iter().step_by(2).collect();
        let values: Vec<_> = call.children.iter().skip(1).step_by(2).collect();

        let mut index = 0;

        while index < keys.len() {
            let key = *keys.get(index).unwrap();
            let value = *values.get(index).unwrap();

            match &**key {
                AstNode::String(prim) => {
                    println!("seg {:#?}", self.current_path);
                    self.current_path
                        .push(ResolutionPathSegment::Field(prim.value.clone()));
                    self.resolve_from_node(value, resolved_schema)?;
                }
                _ => {
                    return Err(SchemaResolverErr::Todo(
                        "dict key is not string".to_string(),
                    ));
                }
            };

            index += 1;
        }

        Ok(())
    }

    // TODO
    fn resolve_list(
        &mut self,
        call: &AstCall,
        resolved_schema: &mut TResolvedSchema,
    ) -> Result<(), SchemaResolverErr> {
        let args_len = call.children.len();

        if !args_len.is_multiple_of(2) || args_len == 0 {
            return Err(SchemaResolverErr::Todo("dict args not even".to_string()));
        }

        self.current_type = Some(DataType::Dict);
        self.commit(resolved_schema)?;

        let keys: Vec<_> = call.children.iter().step_by(2).collect();
        let values: Vec<_> = call.children.iter().skip(1).step_by(2).collect();

        let mut index = 0;

        while index < keys.len() {
            let key = *keys.get(index).unwrap();
            let value = *values.get(index).unwrap();

            match &**key {
                AstNode::String(prim) => {
                    println!("seg {:#?}", self.current_path);
                    self.current_path
                        .push(ResolutionPathSegment::Field(prim.value.clone()));
                    self.resolve_from_node(value, resolved_schema)?;
                }
                _ => {
                    return Err(SchemaResolverErr::Todo(
                        "dict key is not string".to_string(),
                    ));
                }
            };

            index += 1;
        }

        Ok(())
    }
}
