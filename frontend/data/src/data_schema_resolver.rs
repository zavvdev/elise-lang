use std::collections::HashMap;

use elise_ast::{AstCall, AstNode, AstPrimitive};

use elise_shared::shared_errors::errors_schema_resolver::{
    SchemaResolverErr, SchemaResolverErr::*,
};
use elise_shared::shared_types::{ArityMismatchKind, Span};

use crate::data_config::ROOT_ARGS_LEN;
use crate::data_types::SchemaFnLexeme;
use crate::data_types::{DataType, ResolutionPath};

#[derive(Debug, PartialEq)]
pub struct TypeDescriptor {
    pub ty: DataType,
    pub opt: bool,
}

#[derive(Debug, PartialEq)]
pub struct ResolvedSchema {
    pub resolved_schema: HashMap<ResolutionPath, TypeDescriptor>,
}

pub struct SchemaResolver<'a> {
    schema_ast: &'a Vec<AstNode>,
}

impl<'a> SchemaResolver<'a> {
    pub fn new(schema_ast: &'a Vec<AstNode>) -> Self {
        Self { schema_ast }
    }

    pub fn resolve(&self) -> Result<ResolvedSchema, SchemaResolverErr> {
        let root_schema_call = self.get_root_schema_call()?;
        let parent = root_schema_call.children.first().unwrap();

        let mut resolved_schema = ResolvedSchema {
            resolved_schema: HashMap::new(),
        };

        match &**parent {
            AstNode::Call(call) => Self::resolve_types(call, &mut resolved_schema)?,
            node => {
                return Err(InvalTypeDef {
                    span: node.span().clone(),
                });
            }
        }

        Ok(resolved_schema)
    }

    // Ensures that the very top call is .schema function call.
    // We do this in case we want to provide any additional metadata
    // in future for schema.
    fn get_root_schema_call(&self) -> Result<&AstCall, SchemaResolverErr> {
        let root = self.schema_ast.first().ok_or_else(|| InvalRoot {
            span: Span { start: 1, end: 1 },
        })?;

        let root_call = match root {
            AstNode::Call(call) if call.lexeme == SchemaFnLexeme::ROOT => call,
            node => {
                return Err(InvalRoot {
                    span: node.span().clone(),
                });
            }
        };

        // Root call should have only one children.
        match root_call.children.len() {
            ROOT_ARGS_LEN => Ok(root_call),
            args_len => {
                return Err(SchemaResolverErr::ArityMismatch {
                    fn_name: SchemaFnLexeme::ROOT,
                    expected: ROOT_ARGS_LEN,
                    kind: ArityMismatchKind::Eq,
                    found: args_len,
                    span: root_call.span.clone(),
                });
            }
        }
    }

    fn resolve_types(
        parent: &AstCall,
        resolved_schema: &mut ResolvedSchema,
    ) -> Result<(), SchemaResolverErr> {
        let parent_lexeme = &parent.lexeme;
        match parent_lexeme.as_str() {
            SchemaFnLexeme::DICT  => Ok(()),
            SchemaFnLexeme::LIST => Ok(()),
            _ => Err(SchemaResolverErr::InvalTypeDef {
                span: parent.span.clone(),
            }),
        }
    }

    fn resolve_type(
        call_name: &str,
        start: usize,
        end: usize,
    ) -> Result<DataType, SchemaResolverErr> {
        match call_name {
            SchemaFnLexeme::BOOL => Ok(DataType::Bool),
            SchemaFnLexeme::INT => Ok(DataType::Int),
            SchemaFnLexeme::FLOAT => Ok(DataType::Float),
            SchemaFnLexeme::STRING => Ok(DataType::String),
            SchemaFnLexeme::OPT => Ok(DataType::Null),
            _ => Err(ColInvalType {
                span: Span { start, end },
            }),
        }
    }

    fn resolve_col_name(col: &AstNode) -> Result<String, SchemaResolverErr> {
        match col {
            // Column name must always be an identifier type.
            AstNode::Identifier(AstPrimitive { value, span: _ }) => Ok(value.clone()),
            node => Err(ColInvalName {
                span: node.span().clone(),
            }),
        }
    }

    fn resolve_literal_type(node: &AstNode) -> Result<DataType, SchemaResolverErr> {
        match node {
            AstNode::Call(AstCall {
                lexeme: name,
                children,
                span,
            }) => {
                if children.is_empty() {
                    return Self::resolve_type(name, span.start, span.end);
                }
                Err(ColTypeNoArgs {
                    span: span.clone(),
                })
            }
            node => Err(ColInvalType {
                span: node.span().clone(),
            }),
        }
    }

    fn resolve_col_type(ty: &AstNode) -> Result<(DataType, bool), SchemaResolverErr> {
        match ty {
            // Column type must always be a function call.
            AstNode::Call(AstCall {
                lexeme: name,
                children,
                span,
            }) => match name.as_str() {
                SchemaFnLexeme::OPT => {
                    if children.len() == 1 {
                        let literal_type = Self::resolve_literal_type(children.first().unwrap())?;
                        if literal_type == DataType::Null {
                            return Err(OptOpt {
                                span: span.clone(),
                            });
                        }
                        return Ok((literal_type, true));
                    }
                    Err(OptArgsLen {
                        span: span.clone(),
                    })
                }
                _ => Ok((Self::resolve_literal_type(ty)?, false)),
            },
            node => Err(ColInvalType {
                span: node.span().clone(),
            }),
        }
    }
}

// ==================================================================
//
//  TESTS START
//
// ==================================================================

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use elise_ast::{AstCall, AstNode, AstPrimitive};
    use elise_shared::shared_errors::errors_schema_resolver::SchemaResolverErr::*;
    use elise_shared::shared_types::Span;

    use crate::data_csv::data_csv_schema_resolver::{
        CsvColDescriptor, CsvResolvedSchema, CsvSchemaResolver,
    };
    use crate::data_types::{DataType, SchemaFnLexeme};

    // We don't care about Span values here since
    // we just need to make sure that they have the same
    // values as a node we're referring to. So in these tests
    // you can provide arbitrary span values.

    // ==================================================================
    // TESTS COMMON SEMANTICS START
    // ==================================================================

    // TESTS ROOT START

    #[test]
    fn root_should_return_error_if_file_empty() {
        let ast = vec![];
        let result = CsvSchemaResolver::new(&ast).resolve();
        let err = Err(RootInval {
            span: Span { start: 1, end: 1 },
        });
        assert_eq!(result, err);
    }

    #[test]
    fn root_should_return_error_if_invalid_call() {
        let ast = vec![AstNode::Call(AstCall {
            lexeme: "invalid".to_string(),
            span: Span { start: 0, end: 3 },
            children: vec![],
        })];
        let result = CsvSchemaResolver::new(&ast).resolve();
        let err = Err(RootInval {
            span: Span { start: 0, end: 3 },
        });
        assert_eq!(result, err);
    }

    #[test]
    fn root_should_return_error_if_not_a_call() {
        let ast = vec![AstNode::Int(AstPrimitive {
            span: Span { start: 0, end: 3 },
            value: "123".to_string(),
        })];
        let result = CsvSchemaResolver::new(&ast).resolve();
        let err = Err(RootInval {
            span: Span { start: 0, end: 3 },
        });
        assert_eq!(result, err);
    }

    #[test]
    fn root_should_return_error_if_no_args() {
        let ast = vec![AstNode::Call(AstCall {
            lexeme: SchemaFnLexeme::ROOT.to_string(),
            span: Span { start: 0, end: 3 },
            children: vec![],
        })];
        let result = CsvSchemaResolver::new(&ast).resolve();
        let err = Err(RootArgsLen {
            span: Span { start: 0, end: 3 },
        });
        assert_eq!(result, err);
    }

    #[test]
    fn root_should_return_error_if_more_than_one_arg() {
        let row_def = Box::new(AstNode::Call(AstCall {
            lexeme: SchemaFnLexeme::ROW.to_string(),
            span: Span { start: 3, end: 6 },
            children: vec![],
        }));
        let redundant_def = Box::new(AstNode::Call(AstCall {
            lexeme: "row2".to_string(),
            span: Span { start: 6, end: 9 },
            children: vec![],
        }));
        let ast = vec![AstNode::Call(AstCall {
            lexeme: SchemaFnLexeme::ROOT.to_string(),
            span: Span { start: 0, end: 11 },
            children: vec![row_def, redundant_def],
        })];
        let result = CsvSchemaResolver::new(&ast).resolve();
        let err = Err(RootArgsLen {
            span: Span { start: 0, end: 11 },
        });
        assert_eq!(result, err);
    }

    // TESTS ROOT END

    // TESTS ROW START

    #[test]
    fn row_should_return_error_if_not_a_call() {
        let row_def = Box::new(AstNode::Int(AstPrimitive {
            value: "2".to_string(),
            span: Span { start: 3, end: 6 },
        }));
        let ast = vec![AstNode::Call(AstCall {
            lexeme: SchemaFnLexeme::ROOT.to_string(),
            span: Span { start: 0, end: 8 },
            children: vec![row_def],
        })];
        let result = CsvSchemaResolver::new(&ast).resolve();
        let err = Err(RowInval {
            span: Span { start: 3, end: 6 },
        });
        assert_eq!(result, err);
    }

    #[test]
    fn row_should_return_error_if_invalid_call() {
        let row_def = Box::new(AstNode::Call(AstCall {
            lexeme: "invalid".to_string(),
            span: Span { start: 3, end: 6 },
            children: vec![],
        }));
        let ast = vec![AstNode::Call(AstCall {
            lexeme: SchemaFnLexeme::ROOT.to_string(),
            span: Span { start: 0, end: 8 },
            children: vec![row_def],
        })];
        let result = CsvSchemaResolver::new(&ast).resolve();
        let err = Err(RowInval {
            span: Span { start: 3, end: 6 },
        });
        assert_eq!(result, err);
    }

    #[test]
    fn row_should_return_error_if_no_args() {
        let row_def = Box::new(AstNode::Call(AstCall {
            lexeme: SchemaFnLexeme::ROW.to_string(),
            span: Span { start: 3, end: 6 },
            children: vec![],
        }));
        let ast = vec![AstNode::Call(AstCall {
            lexeme: SchemaFnLexeme::ROOT.to_string(),
            span: Span { start: 0, end: 3 },
            children: vec![row_def],
        })];
        let result = CsvSchemaResolver::new(&ast).resolve();
        let err = Err(RowArgsLen {
            span: Span { start: 3, end: 6 },
        });
        assert_eq!(result, err);
    }

    #[test]
    fn row_should_return_error_if_args_not_even() {
        let row_def = Box::new(AstNode::Call(AstCall {
            lexeme: SchemaFnLexeme::ROW.to_string(),
            span: Span { start: 3, end: 6 },
            children: vec![Box::new(AstNode::Identifier(AstPrimitive {
                value: "some_value".to_string(),
                span: Span { start: 9, end: 12 },
            }))],
        }));
        let ast = vec![AstNode::Call(AstCall {
            lexeme: SchemaFnLexeme::ROOT.to_string(),
            span: Span { start: 0, end: 3 },
            children: vec![row_def],
        })];
        let result = CsvSchemaResolver::new(&ast).resolve();
        let err = Err(RowArgsLen {
            span: Span { start: 3, end: 6 },
        });
        assert_eq!(result, err);
    }

    #[test]
    fn row_should_return_error_if_odd_args_not_identifiers() {
        let row_children = vec![
            Box::new(AstNode::Int(AstPrimitive {
                value: "4".to_string(),
                span: Span { start: 9, end: 12 },
            })),
            Box::new(AstNode::Call(AstCall {
                lexeme: SchemaFnLexeme::INT.to_string(),
                span: Span { start: 12, end: 15 },
                children: vec![],
            })),
        ];
        let row_def = Box::new(AstNode::Call(AstCall {
            lexeme: SchemaFnLexeme::ROW.to_string(),
            span: Span { start: 3, end: 6 },
            children: row_children,
        }));
        let ast = vec![AstNode::Call(AstCall {
            lexeme: SchemaFnLexeme::ROOT.to_string(),
            span: Span { start: 0, end: 3 },
            children: vec![row_def],
        })];
        let result = CsvSchemaResolver::new(&ast).resolve();
        let err = Err(ColInvalName {
            span: Span { start: 9, end: 12 },
        });
        assert_eq!(result, err);
    }

    #[test]
    fn row_should_return_error_if_even_args_not_known_calls() {
        let row_children = vec![
            Box::new(AstNode::Identifier(AstPrimitive {
                value: "name".to_string(),
                span: Span { start: 9, end: 12 },
            })),
            Box::new(AstNode::Call(AstCall {
                lexeme: "some".to_string(),
                span: Span { start: 12, end: 15 },
                children: vec![],
            })),
        ];
        let row_def = Box::new(AstNode::Call(AstCall {
            lexeme: SchemaFnLexeme::ROW.to_string(),
            span: Span { start: 3, end: 6 },
            children: row_children,
        }));
        let ast = vec![AstNode::Call(AstCall {
            lexeme: SchemaFnLexeme::ROOT.to_string(),
            span: Span { start: 0, end: 3 },
            children: vec![row_def],
        })];
        let result = CsvSchemaResolver::new(&ast).resolve();
        let err = Err(ColInvalType {
            span: Span { start: 12, end: 15 },
        });
        assert_eq!(result, err);
    }

    // TESTS ROW END

    // ==================================================================
    // TESTS COMMON SEMANTICS END
    // ==================================================================

    // ==================================================================
    // TESTS REQUIRED VALUE START
    // ==================================================================

    #[test]
    fn required_should_resolve() {
        let row_children = vec![
            Box::new(AstNode::Identifier(AstPrimitive {
                value: "name".to_string(),
                span: Span { start: 9, end: 12 },
            })),
            Box::new(AstNode::Call(AstCall {
                lexeme: SchemaFnLexeme::INT.to_string(),
                span: Span { start: 12, end: 15 },
                children: vec![],
            })),
        ];
        let row_def = Box::new(AstNode::Call(AstCall {
            lexeme: SchemaFnLexeme::ROW.to_string(),
            span: Span { start: 3, end: 6 },
            children: row_children,
        }));
        let ast = vec![AstNode::Call(AstCall {
            lexeme: SchemaFnLexeme::ROOT.to_string(),
            span: Span { start: 0, end: 3 },
            children: vec![row_def],
        })];
        let result = CsvSchemaResolver::new(&ast).resolve();
        let mut resolved_schema = HashMap::new();
        resolved_schema.insert(
            "name".to_string(),
            CsvColDescriptor {
                ty: DataType::Int,
                opt: false,
            },
        );
        let resolved = CsvResolvedSchema { resolved_schema };
        assert_eq!(result, Ok(resolved));
    }

    // ==================================================================
    // TESTS REQUIRED VALUE END
    // ==================================================================

    // ==================================================================
    // TESTS OPTIONAL VALUE START
    // ==================================================================

    #[test]
    fn optional_should_reject_empty_type() {
        let opt_children = vec![Box::new(AstNode::Call(AstCall {
            lexeme: SchemaFnLexeme::OPT.to_string(),
            span: Span { start: 12, end: 15 },
            children: vec![],
        }))];
        let type_opt = Box::new(AstNode::Call(AstCall {
            lexeme: SchemaFnLexeme::OPT.to_string(),
            span: Span { start: 15, end: 18 },
            children: opt_children,
        }));
        let row_children = vec![
            Box::new(AstNode::Identifier(AstPrimitive {
                value: "name".to_string(),
                span: Span { start: 9, end: 12 },
            })),
            type_opt,
        ];
        let row_def = Box::new(AstNode::Call(AstCall {
            lexeme: SchemaFnLexeme::ROW.to_string(),
            span: Span { start: 3, end: 6 },
            children: row_children,
        }));
        let ast = vec![AstNode::Call(AstCall {
            lexeme: SchemaFnLexeme::ROOT.to_string(),
            span: Span { start: 0, end: 3 },
            children: vec![row_def],
        })];
        let result = CsvSchemaResolver::new(&ast).resolve();
        let err = Err(OptOpt {
            span: Span { start: 15, end: 18 },
        });
        assert_eq!(result, err);
    }

    #[test]
    fn optional_should_resolve() {
        let opt_children = vec![Box::new(AstNode::Call(AstCall {
            lexeme: SchemaFnLexeme::INT.to_string(),
            span: Span { start: 12, end: 15 },
            children: vec![],
        }))];
        let type_opt = Box::new(AstNode::Call(AstCall {
            lexeme: SchemaFnLexeme::OPT.to_string(),
            span: Span { start: 15, end: 18 },
            children: opt_children,
        }));
        let row_children = vec![
            Box::new(AstNode::Identifier(AstPrimitive {
                value: "name".to_string(),
                span: Span { start: 9, end: 12 },
            })),
            type_opt,
        ];
        let row_def = Box::new(AstNode::Call(AstCall {
            lexeme: SchemaFnLexeme::ROW.to_string(),
            span: Span { start: 3, end: 6 },
            children: row_children,
        }));
        let ast = vec![AstNode::Call(AstCall {
            lexeme: SchemaFnLexeme::ROOT.to_string(),
            span: Span { start: 0, end: 3 },
            children: vec![row_def],
        })];
        let result = CsvSchemaResolver::new(&ast).resolve();
        let mut resolved_schema = HashMap::new();
        resolved_schema.insert(
            "name".to_string(),
            CsvColDescriptor {
                ty: DataType::Int,
                opt: true,
            },
        );
        assert_eq!(result, Ok(CsvResolvedSchema { resolved_schema }));
    }

    // ==================================================================
    // TESTS OPTIONAL VALUE END
    // ==================================================================

    // ==================================================================
    // TESTS INT START
    // ==================================================================

    #[test]
    fn int_should_resolve() {
        let row_children = vec![
            Box::new(AstNode::Identifier(AstPrimitive {
                value: "age".to_string(),
                span: Span { start: 9, end: 12 },
            })),
            Box::new(AstNode::Call(AstCall {
                lexeme: SchemaFnLexeme::INT.to_string(),
                span: Span { start: 12, end: 15 },
                children: vec![],
            })),
        ];
        let row_def = Box::new(AstNode::Call(AstCall {
            lexeme: SchemaFnLexeme::ROW.to_string(),
            span: Span { start: 3, end: 6 },
            children: row_children,
        }));
        let ast = vec![AstNode::Call(AstCall {
            lexeme: SchemaFnLexeme::ROOT.to_string(),
            span: Span { start: 0, end: 3 },
            children: vec![row_def],
        })];
        let result = CsvSchemaResolver::new(&ast).resolve();
        let mut resolved_schema = HashMap::new();
        resolved_schema.insert(
            "age".to_string(),
            CsvColDescriptor {
                ty: DataType::Int,
                opt: false,
            },
        );
        let resolved = CsvResolvedSchema { resolved_schema };
        assert_eq!(result, Ok(resolved));
    }

    #[test]
    fn int_should_return_error_if_has_args() {
        let row_children = vec![
            Box::new(AstNode::Identifier(AstPrimitive {
                value: "name".to_string(),
                span: Span { start: 9, end: 12 },
            })),
            Box::new(AstNode::Call(AstCall {
                lexeme: SchemaFnLexeme::INT.to_string(),
                span: Span { start: 12, end: 15 },
                children: vec![Box::new(AstNode::Int(AstPrimitive {
                    value: "1".to_string(),
                    span: Span { start: 0, end: 3 },
                }))],
            })),
        ];
        let row_def = Box::new(AstNode::Call(AstCall {
            lexeme: SchemaFnLexeme::ROW.to_string(),
            span: Span { start: 3, end: 6 },
            children: row_children,
        }));
        let ast = vec![AstNode::Call(AstCall {
            lexeme: SchemaFnLexeme::ROOT.to_string(),
            span: Span { start: 0, end: 3 },
            children: vec![row_def],
        })];
        let result = CsvSchemaResolver::new(&ast).resolve();
        let err = Err(ColTypeNoArgs {
            span: Span { start: 12, end: 15 },
        });
        assert_eq!(result, err);
    }

    // ==================================================================
    // TESTS INT END
    // ==================================================================

    // ==================================================================
    // TESTS FLOAT START
    // ==================================================================

    #[test]
    fn float_should_resolve() {
        let row_children = vec![
            Box::new(AstNode::Identifier(AstPrimitive {
                value: "age".to_string(),
                span: Span { start: 9, end: 12 },
            })),
            Box::new(AstNode::Call(AstCall {
                lexeme: SchemaFnLexeme::FLOAT.to_string(),
                span: Span { start: 12, end: 15 },
                children: vec![],
            })),
        ];
        let row_def = Box::new(AstNode::Call(AstCall {
            lexeme: SchemaFnLexeme::ROW.to_string(),
            span: Span { start: 3, end: 6 },
            children: row_children,
        }));
        let ast = vec![AstNode::Call(AstCall {
            lexeme: SchemaFnLexeme::ROOT.to_string(),
            span: Span { start: 0, end: 3 },
            children: vec![row_def],
        })];
        let result = CsvSchemaResolver::new(&ast).resolve();
        let mut resolved_schema = HashMap::new();
        resolved_schema.insert(
            "age".to_string(),
            CsvColDescriptor {
                ty: DataType::Float,
                opt: false,
            },
        );
        let resolved = CsvResolvedSchema { resolved_schema };
        assert_eq!(result, Ok(resolved));
    }

    #[test]
    fn float_should_return_error_if_has_args() {
        let row_children = vec![
            Box::new(AstNode::Identifier(AstPrimitive {
                value: "name".to_string(),
                span: Span { start: 9, end: 12 },
            })),
            Box::new(AstNode::Call(AstCall {
                lexeme: SchemaFnLexeme::FLOAT.to_string(),
                span: Span { start: 12, end: 15 },
                children: vec![Box::new(AstNode::Int(AstPrimitive {
                    value: "1".to_string(),
                    span: Span { start: 0, end: 3 },
                }))],
            })),
        ];
        let row_def = Box::new(AstNode::Call(AstCall {
            lexeme: SchemaFnLexeme::ROW.to_string(),
            span: Span { start: 3, end: 6 },
            children: row_children,
        }));
        let ast = vec![AstNode::Call(AstCall {
            lexeme: SchemaFnLexeme::ROOT.to_string(),
            span: Span { start: 0, end: 3 },
            children: vec![row_def],
        })];
        let result = CsvSchemaResolver::new(&ast).resolve();
        let err = Err(ColTypeNoArgs {
            span: Span { start: 12, end: 15 },
        });
        assert_eq!(result, err);
    }

    // ==================================================================
    // TESTS FLOAT END
    // ==================================================================

    // ==================================================================
    // TESTS STRING START
    // ==================================================================

    #[test]
    fn string_should_resolve() {
        let row_children = vec![
            Box::new(AstNode::Identifier(AstPrimitive {
                value: "name".to_string(),
                span: Span { start: 9, end: 12 },
            })),
            Box::new(AstNode::Call(AstCall {
                lexeme: SchemaFnLexeme::STRING.to_string(),
                span: Span { start: 12, end: 15 },
                children: vec![],
            })),
        ];
        let row_def = Box::new(AstNode::Call(AstCall {
            lexeme: SchemaFnLexeme::ROW.to_string(),
            span: Span { start: 3, end: 6 },
            children: row_children,
        }));
        let ast = vec![AstNode::Call(AstCall {
            lexeme: SchemaFnLexeme::ROOT.to_string(),
            span: Span { start: 0, end: 3 },
            children: vec![row_def],
        })];
        let result = CsvSchemaResolver::new(&ast).resolve();
        let mut resolved_schema = HashMap::new();
        resolved_schema.insert(
            "name".to_string(),
            CsvColDescriptor {
                ty: DataType::String,
                opt: false,
            },
        );
        let resolved = CsvResolvedSchema { resolved_schema };
        assert_eq!(result, Ok(resolved));
    }

    #[test]
    fn string_should_return_error_if_has_args() {
        let row_children = vec![
            Box::new(AstNode::Identifier(AstPrimitive {
                value: "name".to_string(),
                span: Span { start: 9, end: 12 },
            })),
            Box::new(AstNode::Call(AstCall {
                lexeme: SchemaFnLexeme::STRING.to_string(),
                span: Span { start: 12, end: 15 },
                children: vec![Box::new(AstNode::Int(AstPrimitive {
                    value: "1".to_string(),
                    span: Span { start: 0, end: 3 },
                }))],
            })),
        ];
        let row_def = Box::new(AstNode::Call(AstCall {
            lexeme: SchemaFnLexeme::ROW.to_string(),
            span: Span { start: 3, end: 6 },
            children: row_children,
        }));
        let ast = vec![AstNode::Call(AstCall {
            lexeme: SchemaFnLexeme::ROOT.to_string(),
            span: Span { start: 0, end: 3 },
            children: vec![row_def],
        })];
        let result = CsvSchemaResolver::new(&ast).resolve();
        let err = Err(ColTypeNoArgs {
            span: Span { start: 12, end: 15 },
        });
        assert_eq!(result, err);
    }

    // ==================================================================
    // TESTS STRING END
    // ==================================================================

    // ==================================================================
    // TESTS BOOLEAN START
    // ==================================================================

    #[test]
    fn boolean_should_resolve() {
        let row_children = vec![
            Box::new(AstNode::Identifier(AstPrimitive {
                value: "employed".to_string(),
                span: Span { start: 9, end: 12 },
            })),
            Box::new(AstNode::Call(AstCall {
                lexeme: SchemaFnLexeme::BOOL.to_string(),
                span: Span { start: 12, end: 15 },
                children: vec![],
            })),
        ];
        let row_def = Box::new(AstNode::Call(AstCall {
            lexeme: SchemaFnLexeme::ROW.to_string(),
            span: Span { start: 3, end: 6 },
            children: row_children,
        }));
        let ast = vec![AstNode::Call(AstCall {
            lexeme: SchemaFnLexeme::ROOT.to_string(),
            span: Span { start: 0, end: 3 },
            children: vec![row_def],
        })];
        let result = CsvSchemaResolver::new(&ast).resolve();
        let mut resolved_schema = HashMap::new();
        resolved_schema.insert(
            "employed".to_string(),
            CsvColDescriptor {
                ty: DataType::Bool,
                opt: false,
            },
        );
        let resolved = CsvResolvedSchema { resolved_schema };
        assert_eq!(result, Ok(resolved));
    }

    #[test]
    fn boolean_should_return_error_if_has_args() {
        let row_children = vec![
            Box::new(AstNode::Identifier(AstPrimitive {
                value: "name".to_string(),
                span: Span { start: 9, end: 12 },
            })),
            Box::new(AstNode::Call(AstCall {
                lexeme: SchemaFnLexeme::BOOL.to_string(),
                span: Span { start: 12, end: 15 },
                children: vec![Box::new(AstNode::Int(AstPrimitive {
                    value: "1".to_string(),
                    span: Span { start: 0, end: 3 },
                }))],
            })),
        ];
        let row_def = Box::new(AstNode::Call(AstCall {
            lexeme: SchemaFnLexeme::ROW.to_string(),
            span: Span { start: 3, end: 6 },
            children: row_children,
        }));
        let ast = vec![AstNode::Call(AstCall {
            lexeme: SchemaFnLexeme::ROOT.to_string(),
            span: Span { start: 0, end: 3 },
            children: vec![row_def],
        })];
        let result = CsvSchemaResolver::new(&ast).resolve();
        let err = Err(ColTypeNoArgs {
            span: Span { start: 12, end: 15 },
        });
        assert_eq!(result, err);
    }

    // ==================================================================
    // TESTS BOOLEAN END
    // ==================================================================
}

// ==================================================================
//
//  TESTS END
//
// ==================================================================
