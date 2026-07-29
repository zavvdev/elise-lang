use elise_ast::{AstCall, AstNode, AstPrimitive};

use elise_shared::shared_errors::errors_csv_schema_resolver::{
    CsvSchemaResolverErr, CsvSchemaResolverErr::*,
};
use elise_shared::shared_types::Span;

use crate::data_config::{
    SchemaFnBool, SchemaFnFloat, SchemaFnInt, SchemaFnOptional, SchemaFnRoot, SchemaFnRow,
    SchemaFnString,
};
use crate::data_types::DataType;

#[derive(Debug, PartialEq)]
pub struct CsvColDescriptor {
    pub name: String,
    pub ty: DataType,
    pub opt: bool,
}

#[derive(Debug, PartialEq)]
pub struct CsvResolvedSchema {
    pub row: Vec<CsvColDescriptor>,
}

pub struct CsvSchemaResolver<'a> {
    schema_ast: &'a Vec<AstNode>,
}

impl<'a> CsvSchemaResolver<'a> {
    pub fn new(schema_ast: &'a Vec<AstNode>) -> Self {
        Self { schema_ast }
    }

    fn err_span(start: usize, end: usize) -> Span {
        Span { start, end }
    }

    fn resolve_type(
        call_name: &str,
        start: usize,
        end: usize,
    ) -> Result<DataType, CsvSchemaResolverErr> {
        match call_name {
            SchemaFnBool::LEXEME => Ok(DataType::Bool),
            SchemaFnInt::LEXEME => Ok(DataType::Int),
            SchemaFnFloat::LEXEME => Ok(DataType::Float),
            SchemaFnString::LEXEME => Ok(DataType::String),
            SchemaFnOptional::LEXEME => Ok(DataType::Null),
            _ => Err(ColInvalType {
                span: Self::err_span(start, end),
            }),
        }
    }

    fn resolve_col_name(col: &AstNode) -> Result<String, CsvSchemaResolverErr> {
        match col {
            // Column name must always be an identifier type.
            AstNode::Identifier(AstPrimitive { value, span: _ }) => Ok(value.clone()),
            node => Err(ColInvalName {
                span: Self::err_span(node.span().start, node.span().end),
            }),
        }
    }

    fn resolve_literal_type(node: &AstNode) -> Result<DataType, CsvSchemaResolverErr> {
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
                    span: Self::err_span(span.start, span.end),
                })
            }
            node => Err(ColInvalType {
                span: Self::err_span(node.span().start, node.span().end),
            }),
        }
    }

    fn resolve_col_type(ty: &AstNode) -> Result<(DataType, bool), CsvSchemaResolverErr> {
        match ty {
            // Column type must always be a function call.
            AstNode::Call(AstCall {
                lexeme: name,
                children,
                span,
            }) => match name.as_str() {
                SchemaFnOptional::LEXEME => {
                    if children.len() == 1 {
                        let literal_type = Self::resolve_literal_type(children.first().unwrap())?;
                        if literal_type != DataType::Null {
                            return Ok((literal_type, true));
                        }
                        return Err(OptEmpty {
                            span: Self::err_span(span.start, span.end),
                        });
                    }
                    Err(OptArgsLen {
                        span: Self::err_span(span.start, span.end),
                    })
                }
                _ => Ok((Self::resolve_literal_type(ty)?, false)),
            },
            node => Err(ColInvalType {
                span: Self::err_span(node.span().start, node.span().end),
            }),
        }
    }

    fn resolve_row(call: &AstCall) -> Result<CsvResolvedSchema, CsvSchemaResolverErr> {
        let row_args_len = call.children.len();
        let start = call.span.start;
        let end = call.span.end;

        // Check if we have even number of arguments.
        if !row_args_len.is_multiple_of(2) || row_args_len == 0 {
            return Err(RowArgsLen {
                span: Self::err_span(start, end),
            });
        }

        // Since we know here that number of arguments is even,
        // then we can extract each odd and even argument.
        let cols: Vec<_> = call.children.iter().step_by(2).collect();
        let types: Vec<_> = call.children.iter().skip(1).step_by(2).collect();

        let mut index = 0;
        let mut resolved_row: Vec<CsvColDescriptor> = vec![];

        while index < cols.len() {
            // Since we split arguments to cols and types and the number
            // of arguments is even, then we have 2 vectors with the same
            // length where items on the same index represent a key-value pair
            // (column name -> column type).
            let col = *cols.get(index).unwrap();
            let ty = *types.get(index).unwrap();

            let col_name = Self::resolve_col_name(col)?;
            let (col_type, optional) = Self::resolve_col_type(ty)?;

            resolved_row.push(CsvColDescriptor {
                name: col_name,
                ty: col_type,
                opt: optional,
            });

            index += 1;
        }

        Ok(CsvResolvedSchema { row: resolved_row })
    }

    pub fn resolve(&self) -> Result<CsvResolvedSchema, CsvSchemaResolverErr> {
        // Root refers to a first function call that defines a schema.
        let root = self.schema_ast.first().ok_or_else(|| RootInval {
            span: Self::err_span(1, 1),
        })?;

        // Extract root node descriptor if it matches type and name.
        let root_call = match root {
            AstNode::Call(call) if call.lexeme == SchemaFnRoot::LEXEME => call,
            node => {
                return Err(RootInval {
                    span: Self::err_span(node.span().start, node.span().end),
                });
            }
        };

        // Root call should have only one children.
        match root_call.children.len() {
            1 => {}
            _ => {
                return Err(RootArgsLen {
                    span: Self::err_span(root_call.span.start, root_call.span.end),
                });
            }
        }

        let row = root_call.children.first().unwrap();

        match &**row {
            AstNode::Call(call) if call.lexeme == SchemaFnRow::LEXEME => Self::resolve_row(call),
            node => Err(RowInval {
                span: Self::err_span(node.span().start, node.span().end),
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
    use elise_ast::{AstCall, AstNode, AstPrimitive};
    use elise_shared::shared_errors::errors_csv_schema_resolver::CsvSchemaResolverErr::*;
    use elise_shared::shared_types::Span;

    use crate::data_config::{
        SchemaFnBool, SchemaFnInt, SchemaFnOptional, SchemaFnRoot, SchemaFnRow, SchemaFnString,
    };
    use crate::data_csv::data_csv_schema_resolver::{
        CsvColDescriptor, CsvResolvedSchema, CsvSchemaResolver,
    };
    use crate::data_types::DataType;

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
            lexeme: SchemaFnRoot::LEXEME.to_string(),
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
            lexeme: SchemaFnRow::LEXEME.to_string(),
            span: Span { start: 3, end: 6 },
            children: vec![],
        }));
        let redundant_def = Box::new(AstNode::Call(AstCall {
            lexeme: "row2".to_string(),
            span: Span { start: 6, end: 9 },
            children: vec![],
        }));
        let ast = vec![AstNode::Call(AstCall {
            lexeme: SchemaFnRoot::LEXEME.to_string(),
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
            lexeme: SchemaFnRoot::LEXEME.to_string(),
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
            lexeme: SchemaFnRoot::LEXEME.to_string(),
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
            lexeme: SchemaFnRow::LEXEME.to_string(),
            span: Span { start: 3, end: 6 },
            children: vec![],
        }));
        let ast = vec![AstNode::Call(AstCall {
            lexeme: SchemaFnRoot::LEXEME.to_string(),
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
            lexeme: SchemaFnRow::LEXEME.to_string(),
            span: Span { start: 3, end: 6 },
            children: vec![Box::new(AstNode::Identifier(AstPrimitive {
                value: "some_value".to_string(),
                span: Span { start: 9, end: 12 },
            }))],
        }));
        let ast = vec![AstNode::Call(AstCall {
            lexeme: SchemaFnRoot::LEXEME.to_string(),
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
                lexeme: SchemaFnInt::LEXEME.to_string(),
                span: Span { start: 12, end: 15 },
                children: vec![],
            })),
        ];
        let row_def = Box::new(AstNode::Call(AstCall {
            lexeme: SchemaFnRow::LEXEME.to_string(),
            span: Span { start: 3, end: 6 },
            children: row_children,
        }));
        let ast = vec![AstNode::Call(AstCall {
            lexeme: SchemaFnRoot::LEXEME.to_string(),
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
            lexeme: SchemaFnRow::LEXEME.to_string(),
            span: Span { start: 3, end: 6 },
            children: row_children,
        }));
        let ast = vec![AstNode::Call(AstCall {
            lexeme: SchemaFnRoot::LEXEME.to_string(),
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
                lexeme: SchemaFnInt::LEXEME.to_string(),
                span: Span { start: 12, end: 15 },
                children: vec![],
            })),
        ];
        let row_def = Box::new(AstNode::Call(AstCall {
            lexeme: SchemaFnRow::LEXEME.to_string(),
            span: Span { start: 3, end: 6 },
            children: row_children,
        }));
        let ast = vec![AstNode::Call(AstCall {
            lexeme: SchemaFnRoot::LEXEME.to_string(),
            span: Span { start: 0, end: 3 },
            children: vec![row_def],
        })];
        let result = CsvSchemaResolver::new(&ast).resolve();
        let resolved = CsvResolvedSchema {
            row: vec![CsvColDescriptor {
                name: "name".to_string(),
                ty: DataType::Int,
                opt: false,
            }],
        };
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
            lexeme: SchemaFnOptional::LEXEME.to_string(),
            span: Span { start: 12, end: 15 },
            children: vec![],
        }))];
        let type_opt = Box::new(AstNode::Call(AstCall {
            lexeme: SchemaFnOptional::LEXEME.to_string(),
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
            lexeme: SchemaFnRow::LEXEME.to_string(),
            span: Span { start: 3, end: 6 },
            children: row_children,
        }));
        let ast = vec![AstNode::Call(AstCall {
            lexeme: SchemaFnRoot::LEXEME.to_string(),
            span: Span { start: 0, end: 3 },
            children: vec![row_def],
        })];
        let result = CsvSchemaResolver::new(&ast).resolve();
        let err = Err(OptEmpty {
            span: Span { start: 15, end: 18 },
        });
        assert_eq!(result, err);
    }

    #[test]
    fn optional_should_resolve() {
        let opt_children = vec![Box::new(AstNode::Call(AstCall {
            lexeme: SchemaFnInt::LEXEME.to_string(),
            span: Span { start: 12, end: 15 },
            children: vec![],
        }))];
        let type_opt = Box::new(AstNode::Call(AstCall {
            lexeme: SchemaFnOptional::LEXEME.to_string(),
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
            lexeme: SchemaFnRow::LEXEME.to_string(),
            span: Span { start: 3, end: 6 },
            children: row_children,
        }));
        let ast = vec![AstNode::Call(AstCall {
            lexeme: SchemaFnRoot::LEXEME.to_string(),
            span: Span { start: 0, end: 3 },
            children: vec![row_def],
        })];
        let result = CsvSchemaResolver::new(&ast).resolve();
        let resolved = CsvResolvedSchema {
            row: vec![CsvColDescriptor {
                name: "name".to_string(),
                ty: DataType::Int,
                opt: true,
            }],
        };
        assert_eq!(result, Ok(resolved));
    }

    // ==================================================================
    // TESTS OPTIONAL VALUE END
    // ==================================================================

    // ==================================================================
    // TESTS NUMBER START
    // ==================================================================

    #[test]
    fn number_should_resolve() {
        let row_children = vec![
            Box::new(AstNode::Identifier(AstPrimitive {
                value: "age".to_string(),
                span: Span { start: 9, end: 12 },
            })),
            Box::new(AstNode::Call(AstCall {
                lexeme: SchemaFnInt::LEXEME.to_string(),
                span: Span { start: 12, end: 15 },
                children: vec![],
            })),
        ];
        let row_def = Box::new(AstNode::Call(AstCall {
            lexeme: SchemaFnRow::LEXEME.to_string(),
            span: Span { start: 3, end: 6 },
            children: row_children,
        }));
        let ast = vec![AstNode::Call(AstCall {
            lexeme: SchemaFnRoot::LEXEME.to_string(),
            span: Span { start: 0, end: 3 },
            children: vec![row_def],
        })];
        let result = CsvSchemaResolver::new(&ast).resolve();
        let resolved = CsvResolvedSchema {
            row: vec![CsvColDescriptor {
                name: "age".to_string(),
                ty: DataType::Int,
                opt: false,
            }],
        };
        assert_eq!(result, Ok(resolved));
    }

    #[test]
    fn number_should_return_error_if_has_args() {
        let row_children = vec![
            Box::new(AstNode::Identifier(AstPrimitive {
                value: "name".to_string(),
                span: Span { start: 9, end: 12 },
            })),
            Box::new(AstNode::Call(AstCall {
                lexeme: SchemaFnInt::LEXEME.to_string(),
                span: Span { start: 12, end: 15 },
                children: vec![Box::new(AstNode::Int(AstPrimitive {
                    value: "1".to_string(),
                    span: Span { start: 0, end: 3 },
                }))],
            })),
        ];
        let row_def = Box::new(AstNode::Call(AstCall {
            lexeme: SchemaFnRow::LEXEME.to_string(),
            span: Span { start: 3, end: 6 },
            children: row_children,
        }));
        let ast = vec![AstNode::Call(AstCall {
            lexeme: SchemaFnRoot::LEXEME.to_string(),
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
    // TESTS NUMBER END
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
                lexeme: SchemaFnString::LEXEME.to_string(),
                span: Span { start: 12, end: 15 },
                children: vec![],
            })),
        ];
        let row_def = Box::new(AstNode::Call(AstCall {
            lexeme: SchemaFnRow::LEXEME.to_string(),
            span: Span { start: 3, end: 6 },
            children: row_children,
        }));
        let ast = vec![AstNode::Call(AstCall {
            lexeme: SchemaFnRoot::LEXEME.to_string(),
            span: Span { start: 0, end: 3 },
            children: vec![row_def],
        })];
        let result = CsvSchemaResolver::new(&ast).resolve();
        let resolved = CsvResolvedSchema {
            row: vec![CsvColDescriptor {
                name: "name".to_string(),
                ty: DataType::String,
                opt: false,
            }],
        };
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
                lexeme: SchemaFnString::LEXEME.to_string(),
                span: Span { start: 12, end: 15 },
                children: vec![Box::new(AstNode::Int(AstPrimitive {
                    value: "1".to_string(),
                    span: Span { start: 0, end: 3 },
                }))],
            })),
        ];
        let row_def = Box::new(AstNode::Call(AstCall {
            lexeme: SchemaFnRow::LEXEME.to_string(),
            span: Span { start: 3, end: 6 },
            children: row_children,
        }));
        let ast = vec![AstNode::Call(AstCall {
            lexeme: SchemaFnRoot::LEXEME.to_string(),
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
                lexeme: SchemaFnBool::LEXEME.to_string(),
                span: Span { start: 12, end: 15 },
                children: vec![],
            })),
        ];
        let row_def = Box::new(AstNode::Call(AstCall {
            lexeme: SchemaFnRow::LEXEME.to_string(),
            span: Span { start: 3, end: 6 },
            children: row_children,
        }));
        let ast = vec![AstNode::Call(AstCall {
            lexeme: SchemaFnRoot::LEXEME.to_string(),
            span: Span { start: 0, end: 3 },
            children: vec![row_def],
        })];
        let result = CsvSchemaResolver::new(&ast).resolve();
        let resolved = CsvResolvedSchema {
            row: vec![CsvColDescriptor {
                name: "employed".to_string(),
                ty: DataType::Bool,
                opt: false,
            }],
        };
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
                lexeme: SchemaFnBool::LEXEME.to_string(),
                span: Span { start: 12, end: 15 },
                children: vec![Box::new(AstNode::Int(AstPrimitive {
                    value: "1".to_string(),
                    span: Span { start: 0, end: 3 },
                }))],
            })),
        ];
        let row_def = Box::new(AstNode::Call(AstCall {
            lexeme: SchemaFnRow::LEXEME.to_string(),
            span: Span { start: 3, end: 6 },
            children: row_children,
        }));
        let ast = vec![AstNode::Call(AstCall {
            lexeme: SchemaFnRoot::LEXEME.to_string(),
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
