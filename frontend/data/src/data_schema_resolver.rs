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
    pub optional: bool,
}

type TResolvedSchema = HashMap<ResolutionPath, TypeDescriptor>;

#[derive(Debug, PartialEq)]
pub struct ResolvedSchema {
    pub resolved_schema: TResolvedSchema,
}

pub struct SchemaResolver<'a> {
    schema_ast: &'a Vec<AstNode>,
    current_path: ResolutionPath,
    // For tracking current data type being resolved.
    // This is needed for cases like if we want to check
    // if some type definition is allowed to be used as the
    // child of the current type only. Like .of must be used
    // only as a child of .list definition.
    current_type: Option<DataType>,
    // Track optional state for current type definition.
    current_optional: bool,
}

impl<'a> SchemaResolver<'a> {
    pub fn new(schema_ast: &'a Vec<AstNode>) -> Self {
        Self {
            schema_ast,
            current_path: vec![ResolutionPathSegment::Root],
            current_type: None,
            current_optional: false,
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
                    optional: self.current_optional,
                },
            );
            self.current_type = None;
            return Ok(());
        }
        Err(SchemaResolverErr::Todo)
    }

    fn backtrack(&mut self) {
        self.current_path.pop();
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
                SchemaFnLexeme::OPT => self.resolve_optional(call, resolved_schema),
                SchemaFnLexeme::DICT => self.resolve_dict(call, resolved_schema),
                SchemaFnLexeme::LIST => Ok(()),
                SchemaFnLexeme::LIST_OF => Ok(()),
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

    fn resolve_optional(
        &mut self,
        call: &AstCall,
        resolved_schema: &mut TResolvedSchema,
    ) -> Result<(), SchemaResolverErr> {
        let args_len = call.children.len();

        if args_len != OPT_ARGS_LEN {
            return Err(SchemaResolverErr::ArityMismatch {
                fn_name: SchemaFnLexeme::OPT,
                expected: OPT_ARGS_LEN,
                kind: ArityMismatchKind::Eq,
                found: args_len,
                span: call.span.clone(),
            });
        }

        if self.current_optional {
            return Err(SchemaResolverErr::Todo);
        }

        self.current_optional = true;
        self.resolve_from_node(call.children.first().unwrap(), resolved_schema)?;
        self.current_optional = false;

        Ok(())
    }

    fn resolve_dict(
        &mut self,
        call: &AstCall,
        resolved_schema: &mut TResolvedSchema,
    ) -> Result<(), SchemaResolverErr> {
        let args_len = call.children.len();

        if !args_len.is_multiple_of(2) || args_len == 0 {
            return Err(SchemaResolverErr::Todo);
        }

        // TODO: We either need to include a full data type descriptor
        // into DataType::N or just rely on the expanded tree type hints
        // that we're building now.
        //self.current_type = Some(DataType::?);
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
                _ => {
                    return Err(SchemaResolverErr::Todo);
                }
            };

            index += 1;
        }

        Ok(())
    }
}

// ==================================================================
//
//  TESTS START
//
// ==================================================================

//#[cfg(test)]
//mod tests {
//    use std::collections::HashMap;
//
//    use elise_ast::{AstCall, AstNode, AstPrimitive};
//    use elise_shared::shared_errors::errors_schema_resolver::SchemaResolverErr::*;
//    use elise_shared::shared_types::Span;
//
//    use crate::data_csv::data_csv_schema_resolver::{
//        CsvColDescriptor, CsvResolvedSchema, CsvSchemaResolver,
//    };
//    use crate::data_types::{DataType, SchemaFnLexeme};
//
//    // We don't care about Span values here since
//    // we just need to make sure that they have the same
//    // values as a node we're referring to. So in these tests
//    // you can provide arbitrary span values.
//
//    // ==================================================================
//    // TESTS COMMON SEMANTICS START
//    // ==================================================================
//
//    // TESTS ROOT START
//
//    #[test]
//    fn root_should_return_error_if_file_empty() {
//        let ast = vec![];
//        let result = CsvSchemaResolver::new(&ast).resolve();
//        let err = Err(RootInval {
//            span: Span { start: 1, end: 1 },
//        });
//        assert_eq!(result, err);
//    }
//
//    #[test]
//    fn root_should_return_error_if_invalid_call() {
//        let ast = vec![AstNode::Call(AstCall {
//            lexeme: "invalid".to_string(),
//            span: Span { start: 0, end: 3 },
//            children: vec![],
//        })];
//        let result = CsvSchemaResolver::new(&ast).resolve();
//        let err = Err(RootInval {
//            span: Span { start: 0, end: 3 },
//        });
//        assert_eq!(result, err);
//    }
//
//    #[test]
//    fn root_should_return_error_if_not_a_call() {
//        let ast = vec![AstNode::Int(AstPrimitive {
//            span: Span { start: 0, end: 3 },
//            value: "123".to_string(),
//        })];
//        let result = CsvSchemaResolver::new(&ast).resolve();
//        let err = Err(RootInval {
//            span: Span { start: 0, end: 3 },
//        });
//        assert_eq!(result, err);
//    }
//
//    #[test]
//    fn root_should_return_error_if_no_args() {
//        let ast = vec![AstNode::Call(AstCall {
//            lexeme: SchemaFnLexeme::ROOT.to_string(),
//            span: Span { start: 0, end: 3 },
//            children: vec![],
//        })];
//        let result = CsvSchemaResolver::new(&ast).resolve();
//        let err = Err(RootArgsLen {
//            span: Span { start: 0, end: 3 },
//        });
//        assert_eq!(result, err);
//    }
//
//    #[test]
//    fn root_should_return_error_if_more_than_one_arg() {
//        let row_def = Box::new(AstNode::Call(AstCall {
//            lexeme: SchemaFnLexeme::ROW.to_string(),
//            span: Span { start: 3, end: 6 },
//            children: vec![],
//        }));
//        let redundant_def = Box::new(AstNode::Call(AstCall {
//            lexeme: "row2".to_string(),
//            span: Span { start: 6, end: 9 },
//            children: vec![],
//        }));
//        let ast = vec![AstNode::Call(AstCall {
//            lexeme: SchemaFnLexeme::ROOT.to_string(),
//            span: Span { start: 0, end: 11 },
//            children: vec![row_def, redundant_def],
//        })];
//        let result = CsvSchemaResolver::new(&ast).resolve();
//        let err = Err(RootArgsLen {
//            span: Span { start: 0, end: 11 },
//        });
//        assert_eq!(result, err);
//    }
//
//    // TESTS ROOT END
//
//    // TESTS ROW START
//
//    #[test]
//    fn row_should_return_error_if_not_a_call() {
//        let row_def = Box::new(AstNode::Int(AstPrimitive {
//            value: "2".to_string(),
//            span: Span { start: 3, end: 6 },
//        }));
//        let ast = vec![AstNode::Call(AstCall {
//            lexeme: SchemaFnLexeme::ROOT.to_string(),
//            span: Span { start: 0, end: 8 },
//            children: vec![row_def],
//        })];
//        let result = CsvSchemaResolver::new(&ast).resolve();
//        let err = Err(RowInval {
//            span: Span { start: 3, end: 6 },
//        });
//        assert_eq!(result, err);
//    }
//
//    #[test]
//    fn row_should_return_error_if_invalid_call() {
//        let row_def = Box::new(AstNode::Call(AstCall {
//            lexeme: "invalid".to_string(),
//            span: Span { start: 3, end: 6 },
//            children: vec![],
//        }));
//        let ast = vec![AstNode::Call(AstCall {
//            lexeme: SchemaFnLexeme::ROOT.to_string(),
//            span: Span { start: 0, end: 8 },
//            children: vec![row_def],
//        })];
//        let result = CsvSchemaResolver::new(&ast).resolve();
//        let err = Err(RowInval {
//            span: Span { start: 3, end: 6 },
//        });
//        assert_eq!(result, err);
//    }
//
//    #[test]
//    fn row_should_return_error_if_no_args() {
//        let row_def = Box::new(AstNode::Call(AstCall {
//            lexeme: SchemaFnLexeme::ROW.to_string(),
//            span: Span { start: 3, end: 6 },
//            children: vec![],
//        }));
//        let ast = vec![AstNode::Call(AstCall {
//            lexeme: SchemaFnLexeme::ROOT.to_string(),
//            span: Span { start: 0, end: 3 },
//            children: vec![row_def],
//        })];
//        let result = CsvSchemaResolver::new(&ast).resolve();
//        let err = Err(RowArgsLen {
//            span: Span { start: 3, end: 6 },
//        });
//        assert_eq!(result, err);
//    }
//
//    #[test]
//    fn row_should_return_error_if_args_not_even() {
//        let row_def = Box::new(AstNode::Call(AstCall {
//            lexeme: SchemaFnLexeme::ROW.to_string(),
//            span: Span { start: 3, end: 6 },
//            children: vec![Box::new(AstNode::Identifier(AstPrimitive {
//                value: "some_value".to_string(),
//                span: Span { start: 9, end: 12 },
//            }))],
//        }));
//        let ast = vec![AstNode::Call(AstCall {
//            lexeme: SchemaFnLexeme::ROOT.to_string(),
//            span: Span { start: 0, end: 3 },
//            children: vec![row_def],
//        })];
//        let result = CsvSchemaResolver::new(&ast).resolve();
//        let err = Err(RowArgsLen {
//            span: Span { start: 3, end: 6 },
//        });
//        assert_eq!(result, err);
//    }
//
//    #[test]
//    fn row_should_return_error_if_odd_args_not_identifiers() {
//        let row_children = vec![
//            Box::new(AstNode::Int(AstPrimitive {
//                value: "4".to_string(),
//                span: Span { start: 9, end: 12 },
//            })),
//            Box::new(AstNode::Call(AstCall {
//                lexeme: SchemaFnLexeme::INT.to_string(),
//                span: Span { start: 12, end: 15 },
//                children: vec![],
//            })),
//        ];
//        let row_def = Box::new(AstNode::Call(AstCall {
//            lexeme: SchemaFnLexeme::ROW.to_string(),
//            span: Span { start: 3, end: 6 },
//            children: row_children,
//        }));
//        let ast = vec![AstNode::Call(AstCall {
//            lexeme: SchemaFnLexeme::ROOT.to_string(),
//            span: Span { start: 0, end: 3 },
//            children: vec![row_def],
//        })];
//        let result = CsvSchemaResolver::new(&ast).resolve();
//        let err = Err(ColInvalName {
//            span: Span { start: 9, end: 12 },
//        });
//        assert_eq!(result, err);
//    }
//
//    #[test]
//    fn row_should_return_error_if_even_args_not_known_calls() {
//        let row_children = vec![
//            Box::new(AstNode::Identifier(AstPrimitive {
//                value: "name".to_string(),
//                span: Span { start: 9, end: 12 },
//            })),
//            Box::new(AstNode::Call(AstCall {
//                lexeme: "some".to_string(),
//                span: Span { start: 12, end: 15 },
//                children: vec![],
//            })),
//        ];
//        let row_def = Box::new(AstNode::Call(AstCall {
//            lexeme: SchemaFnLexeme::ROW.to_string(),
//            span: Span { start: 3, end: 6 },
//            children: row_children,
//        }));
//        let ast = vec![AstNode::Call(AstCall {
//            lexeme: SchemaFnLexeme::ROOT.to_string(),
//            span: Span { start: 0, end: 3 },
//            children: vec![row_def],
//        })];
//        let result = CsvSchemaResolver::new(&ast).resolve();
//        let err = Err(ColInvalType {
//            span: Span { start: 12, end: 15 },
//        });
//        assert_eq!(result, err);
//    }
//
//    // TESTS ROW END
//
//    // ==================================================================
//    // TESTS COMMON SEMANTICS END
//    // ==================================================================
//
//    // ==================================================================
//    // TESTS REQUIRED VALUE START
//    // ==================================================================
//
//    #[test]
//    fn required_should_resolve() {
//        let row_children = vec![
//            Box::new(AstNode::Identifier(AstPrimitive {
//                value: "name".to_string(),
//                span: Span { start: 9, end: 12 },
//            })),
//            Box::new(AstNode::Call(AstCall {
//                lexeme: SchemaFnLexeme::INT.to_string(),
//                span: Span { start: 12, end: 15 },
//                children: vec![],
//            })),
//        ];
//        let row_def = Box::new(AstNode::Call(AstCall {
//            lexeme: SchemaFnLexeme::ROW.to_string(),
//            span: Span { start: 3, end: 6 },
//            children: row_children,
//        }));
//        let ast = vec![AstNode::Call(AstCall {
//            lexeme: SchemaFnLexeme::ROOT.to_string(),
//            span: Span { start: 0, end: 3 },
//            children: vec![row_def],
//        })];
//        let result = CsvSchemaResolver::new(&ast).resolve();
//        let mut resolved_schema = HashMap::new();
//        resolved_schema.insert(
//            "name".to_string(),
//            CsvColDescriptor {
//                ty: DataType::Int,
//                opt: false,
//            },
//        );
//        let resolved = CsvResolvedSchema { resolved_schema };
//        assert_eq!(result, Ok(resolved));
//    }
//
//    // ==================================================================
//    // TESTS REQUIRED VALUE END
//    // ==================================================================
//
//    // ==================================================================
//    // TESTS OPTIONAL VALUE START
//    // ==================================================================
//
//    #[test]
//    fn optional_should_reject_empty_type() {
//        let opt_children = vec![Box::new(AstNode::Call(AstCall {
//            lexeme: SchemaFnLexeme::OPT.to_string(),
//            span: Span { start: 12, end: 15 },
//            children: vec![],
//        }))];
//        let type_opt = Box::new(AstNode::Call(AstCall {
//            lexeme: SchemaFnLexeme::OPT.to_string(),
//            span: Span { start: 15, end: 18 },
//            children: opt_children,
//        }));
//        let row_children = vec![
//            Box::new(AstNode::Identifier(AstPrimitive {
//                value: "name".to_string(),
//                span: Span { start: 9, end: 12 },
//            })),
//            type_opt,
//        ];
//        let row_def = Box::new(AstNode::Call(AstCall {
//            lexeme: SchemaFnLexeme::ROW.to_string(),
//            span: Span { start: 3, end: 6 },
//            children: row_children,
//        }));
//        let ast = vec![AstNode::Call(AstCall {
//            lexeme: SchemaFnLexeme::ROOT.to_string(),
//            span: Span { start: 0, end: 3 },
//            children: vec![row_def],
//        })];
//        let result = CsvSchemaResolver::new(&ast).resolve();
//        let err = Err(OptOpt {
//            span: Span { start: 15, end: 18 },
//        });
//        assert_eq!(result, err);
//    }
//
//    #[test]
//    fn optional_should_resolve() {
//        let opt_children = vec![Box::new(AstNode::Call(AstCall {
//            lexeme: SchemaFnLexeme::INT.to_string(),
//            span: Span { start: 12, end: 15 },
//            children: vec![],
//        }))];
//        let type_opt = Box::new(AstNode::Call(AstCall {
//            lexeme: SchemaFnLexeme::OPT.to_string(),
//            span: Span { start: 15, end: 18 },
//            children: opt_children,
//        }));
//        let row_children = vec![
//            Box::new(AstNode::Identifier(AstPrimitive {
//                value: "name".to_string(),
//                span: Span { start: 9, end: 12 },
//            })),
//            type_opt,
//        ];
//        let row_def = Box::new(AstNode::Call(AstCall {
//            lexeme: SchemaFnLexeme::ROW.to_string(),
//            span: Span { start: 3, end: 6 },
//            children: row_children,
//        }));
//        let ast = vec![AstNode::Call(AstCall {
//            lexeme: SchemaFnLexeme::ROOT.to_string(),
//            span: Span { start: 0, end: 3 },
//            children: vec![row_def],
//        })];
//        let result = CsvSchemaResolver::new(&ast).resolve();
//        let mut resolved_schema = HashMap::new();
//        resolved_schema.insert(
//            "name".to_string(),
//            CsvColDescriptor {
//                ty: DataType::Int,
//                opt: true,
//            },
//        );
//        assert_eq!(result, Ok(CsvResolvedSchema { resolved_schema }));
//    }
//
//    // ==================================================================
//    // TESTS OPTIONAL VALUE END
//    // ==================================================================
//
//    // ==================================================================
//    // TESTS INT START
//    // ==================================================================
//
//    #[test]
//    fn int_should_resolve() {
//        let row_children = vec![
//            Box::new(AstNode::Identifier(AstPrimitive {
//                value: "age".to_string(),
//                span: Span { start: 9, end: 12 },
//            })),
//            Box::new(AstNode::Call(AstCall {
//                lexeme: SchemaFnLexeme::INT.to_string(),
//                span: Span { start: 12, end: 15 },
//                children: vec![],
//            })),
//        ];
//        let row_def = Box::new(AstNode::Call(AstCall {
//            lexeme: SchemaFnLexeme::ROW.to_string(),
//            span: Span { start: 3, end: 6 },
//            children: row_children,
//        }));
//        let ast = vec![AstNode::Call(AstCall {
//            lexeme: SchemaFnLexeme::ROOT.to_string(),
//            span: Span { start: 0, end: 3 },
//            children: vec![row_def],
//        })];
//        let result = CsvSchemaResolver::new(&ast).resolve();
//        let mut resolved_schema = HashMap::new();
//        resolved_schema.insert(
//            "age".to_string(),
//            CsvColDescriptor {
//                ty: DataType::Int,
//                opt: false,
//            },
//        );
//        let resolved = CsvResolvedSchema { resolved_schema };
//        assert_eq!(result, Ok(resolved));
//    }
//
//    #[test]
//    fn int_should_return_error_if_has_args() {
//        let row_children = vec![
//            Box::new(AstNode::Identifier(AstPrimitive {
//                value: "name".to_string(),
//                span: Span { start: 9, end: 12 },
//            })),
//            Box::new(AstNode::Call(AstCall {
//                lexeme: SchemaFnLexeme::INT.to_string(),
//                span: Span { start: 12, end: 15 },
//                children: vec![Box::new(AstNode::Int(AstPrimitive {
//                    value: "1".to_string(),
//                    span: Span { start: 0, end: 3 },
//                }))],
//            })),
//        ];
//        let row_def = Box::new(AstNode::Call(AstCall {
//            lexeme: SchemaFnLexeme::ROW.to_string(),
//            span: Span { start: 3, end: 6 },
//            children: row_children,
//        }));
//        let ast = vec![AstNode::Call(AstCall {
//            lexeme: SchemaFnLexeme::ROOT.to_string(),
//            span: Span { start: 0, end: 3 },
//            children: vec![row_def],
//        })];
//        let result = CsvSchemaResolver::new(&ast).resolve();
//        let err = Err(ColTypeNoArgs {
//            span: Span { start: 12, end: 15 },
//        });
//        assert_eq!(result, err);
//    }
//
//    // ==================================================================
//    // TESTS INT END
//    // ==================================================================
//
//    // ==================================================================
//    // TESTS FLOAT START
//    // ==================================================================
//
//    #[test]
//    fn float_should_resolve() {
//        let row_children = vec![
//            Box::new(AstNode::Identifier(AstPrimitive {
//                value: "age".to_string(),
//                span: Span { start: 9, end: 12 },
//            })),
//            Box::new(AstNode::Call(AstCall {
//                lexeme: SchemaFnLexeme::FLOAT.to_string(),
//                span: Span { start: 12, end: 15 },
//                children: vec![],
//            })),
//        ];
//        let row_def = Box::new(AstNode::Call(AstCall {
//            lexeme: SchemaFnLexeme::ROW.to_string(),
//            span: Span { start: 3, end: 6 },
//            children: row_children,
//        }));
//        let ast = vec![AstNode::Call(AstCall {
//            lexeme: SchemaFnLexeme::ROOT.to_string(),
//            span: Span { start: 0, end: 3 },
//            children: vec![row_def],
//        })];
//        let result = CsvSchemaResolver::new(&ast).resolve();
//        let mut resolved_schema = HashMap::new();
//        resolved_schema.insert(
//            "age".to_string(),
//            CsvColDescriptor {
//                ty: DataType::Float,
//                opt: false,
//            },
//        );
//        let resolved = CsvResolvedSchema { resolved_schema };
//        assert_eq!(result, Ok(resolved));
//    }
//
//    #[test]
//    fn float_should_return_error_if_has_args() {
//        let row_children = vec![
//            Box::new(AstNode::Identifier(AstPrimitive {
//                value: "name".to_string(),
//                span: Span { start: 9, end: 12 },
//            })),
//            Box::new(AstNode::Call(AstCall {
//                lexeme: SchemaFnLexeme::FLOAT.to_string(),
//                span: Span { start: 12, end: 15 },
//                children: vec![Box::new(AstNode::Int(AstPrimitive {
//                    value: "1".to_string(),
//                    span: Span { start: 0, end: 3 },
//                }))],
//            })),
//        ];
//        let row_def = Box::new(AstNode::Call(AstCall {
//            lexeme: SchemaFnLexeme::ROW.to_string(),
//            span: Span { start: 3, end: 6 },
//            children: row_children,
//        }));
//        let ast = vec![AstNode::Call(AstCall {
//            lexeme: SchemaFnLexeme::ROOT.to_string(),
//            span: Span { start: 0, end: 3 },
//            children: vec![row_def],
//        })];
//        let result = CsvSchemaResolver::new(&ast).resolve();
//        let err = Err(ColTypeNoArgs {
//            span: Span { start: 12, end: 15 },
//        });
//        assert_eq!(result, err);
//    }
//
//    // ==================================================================
//    // TESTS FLOAT END
//    // ==================================================================
//
//    // ==================================================================
//    // TESTS STRING START
//    // ==================================================================
//
//    #[test]
//    fn string_should_resolve() {
//        let row_children = vec![
//            Box::new(AstNode::Identifier(AstPrimitive {
//                value: "name".to_string(),
//                span: Span { start: 9, end: 12 },
//            })),
//            Box::new(AstNode::Call(AstCall {
//                lexeme: SchemaFnLexeme::STRING.to_string(),
//                span: Span { start: 12, end: 15 },
//                children: vec![],
//            })),
//        ];
//        let row_def = Box::new(AstNode::Call(AstCall {
//            lexeme: SchemaFnLexeme::ROW.to_string(),
//            span: Span { start: 3, end: 6 },
//            children: row_children,
//        }));
//        let ast = vec![AstNode::Call(AstCall {
//            lexeme: SchemaFnLexeme::ROOT.to_string(),
//            span: Span { start: 0, end: 3 },
//            children: vec![row_def],
//        })];
//        let result = CsvSchemaResolver::new(&ast).resolve();
//        let mut resolved_schema = HashMap::new();
//        resolved_schema.insert(
//            "name".to_string(),
//            CsvColDescriptor {
//                ty: DataType::String,
//                opt: false,
//            },
//        );
//        let resolved = CsvResolvedSchema { resolved_schema };
//        assert_eq!(result, Ok(resolved));
//    }
//
//    #[test]
//    fn string_should_return_error_if_has_args() {
//        let row_children = vec![
//            Box::new(AstNode::Identifier(AstPrimitive {
//                value: "name".to_string(),
//                span: Span { start: 9, end: 12 },
//            })),
//            Box::new(AstNode::Call(AstCall {
//                lexeme: SchemaFnLexeme::STRING.to_string(),
//                span: Span { start: 12, end: 15 },
//                children: vec![Box::new(AstNode::Int(AstPrimitive {
//                    value: "1".to_string(),
//                    span: Span { start: 0, end: 3 },
//                }))],
//            })),
//        ];
//        let row_def = Box::new(AstNode::Call(AstCall {
//            lexeme: SchemaFnLexeme::ROW.to_string(),
//            span: Span { start: 3, end: 6 },
//            children: row_children,
//        }));
//        let ast = vec![AstNode::Call(AstCall {
//            lexeme: SchemaFnLexeme::ROOT.to_string(),
//            span: Span { start: 0, end: 3 },
//            children: vec![row_def],
//        })];
//        let result = CsvSchemaResolver::new(&ast).resolve();
//        let err = Err(ColTypeNoArgs {
//            span: Span { start: 12, end: 15 },
//        });
//        assert_eq!(result, err);
//    }
//
//    // ==================================================================
//    // TESTS STRING END
//    // ==================================================================
//
//    // ==================================================================
//    // TESTS BOOLEAN START
//    // ==================================================================
//
//    #[test]
//    fn boolean_should_resolve() {
//        let row_children = vec![
//            Box::new(AstNode::Identifier(AstPrimitive {
//                value: "employed".to_string(),
//                span: Span { start: 9, end: 12 },
//            })),
//            Box::new(AstNode::Call(AstCall {
//                lexeme: SchemaFnLexeme::BOOL.to_string(),
//                span: Span { start: 12, end: 15 },
//                children: vec![],
//            })),
//        ];
//        let row_def = Box::new(AstNode::Call(AstCall {
//            lexeme: SchemaFnLexeme::ROW.to_string(),
//            span: Span { start: 3, end: 6 },
//            children: row_children,
//        }));
//        let ast = vec![AstNode::Call(AstCall {
//            lexeme: SchemaFnLexeme::ROOT.to_string(),
//            span: Span { start: 0, end: 3 },
//            children: vec![row_def],
//        })];
//        let result = CsvSchemaResolver::new(&ast).resolve();
//        let mut resolved_schema = HashMap::new();
//        resolved_schema.insert(
//            "employed".to_string(),
//            CsvColDescriptor {
//                ty: DataType::Bool,
//                opt: false,
//            },
//        );
//        let resolved = CsvResolvedSchema { resolved_schema };
//        assert_eq!(result, Ok(resolved));
//    }
//
//    #[test]
//    fn boolean_should_return_error_if_has_args() {
//        let row_children = vec![
//            Box::new(AstNode::Identifier(AstPrimitive {
//                value: "name".to_string(),
//                span: Span { start: 9, end: 12 },
//            })),
//            Box::new(AstNode::Call(AstCall {
//                lexeme: SchemaFnLexeme::BOOL.to_string(),
//                span: Span { start: 12, end: 15 },
//                children: vec![Box::new(AstNode::Int(AstPrimitive {
//                    value: "1".to_string(),
//                    span: Span { start: 0, end: 3 },
//                }))],
//            })),
//        ];
//        let row_def = Box::new(AstNode::Call(AstCall {
//            lexeme: SchemaFnLexeme::ROW.to_string(),
//            span: Span { start: 3, end: 6 },
//            children: row_children,
//        }));
//        let ast = vec![AstNode::Call(AstCall {
//            lexeme: SchemaFnLexeme::ROOT.to_string(),
//            span: Span { start: 0, end: 3 },
//            children: vec![row_def],
//        })];
//        let result = CsvSchemaResolver::new(&ast).resolve();
//        let err = Err(ColTypeNoArgs {
//            span: Span { start: 12, end: 15 },
//        });
//        assert_eq!(result, err);
//    }
//
//    // ==================================================================
//    // TESTS BOOLEAN END
//    // ==================================================================
//}

// ==================================================================
//
//  TESTS END
//
// ==================================================================
