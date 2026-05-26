use elise_ast::{AstNode, CallKind::*, Compound, Primitive};

use elise_builtins::schema::{
    SCHEMA_FN_BOOL, SCHEMA_FN_NUMBER, SCHEMA_FN_OPTIONAL, SCHEMA_FN_ROOT, SCHEMA_FN_ROW,
    SCHEMA_FN_STRING,
};
use elise_errors::{
    LangErr,
    errors_csv_schema_resolver::{CsvSchemaResolverErr, CsvSchemaResolverErr::*},
};
use elise_types::Span;

use crate::types::CsvColType;

#[derive(Debug, PartialEq)]
struct CsvColDescriptor {
    name: String,
    ty: CsvColType,
    opt: bool,
}

#[derive(Debug, PartialEq)]
pub struct CsvResolvedSchema {
    row: Vec<CsvColDescriptor>,
}

pub struct CsvSchemaResolver<'a> {
    schema_ast: &'a Vec<AstNode>,
}

impl<'a> CsvSchemaResolver<'a> {
    pub fn new(schema_ast: &'a Vec<AstNode>) -> Self {
        Self { schema_ast }
    }

    fn err(e: CsvSchemaResolverErr) -> LangErr {
        LangErr::CsvSchemaResolver(e)
    }

    fn err_span(start: usize, end: usize) -> Span {
        Span { start, end }
    }

    fn resolve_type(call_name: &str, start: usize, end: usize) -> Result<CsvColType, LangErr> {
        match call_name {
            SCHEMA_FN_BOOL => Ok(CsvColType::Bool),
            SCHEMA_FN_NUMBER => Ok(CsvColType::Number),
            SCHEMA_FN_STRING => Ok(CsvColType::String),
            _ => Err(Self::err(ColInvalType {
                span: Self::err_span(start, end),
            })),
        }
    }

    fn resolve_col_name(col: &AstNode) -> Result<String, LangErr> {
        match col {
            // Column name must always be an identifier type.
            AstNode::Identifier(Primitive { value, span: _ }) => Ok(value.clone()),
            node => Err(Self::err(ColInvalName {
                span: Self::err_span(node.span().start, node.span().end),
            })),
        }
    }

    fn resolve_literal_type(node: &AstNode) -> Result<CsvColType, LangErr> {
        match node {
            AstNode::Call((Named(name), Compound { children, span })) => {
                if children.is_empty() {
                    return Self::resolve_type(name, span.start, span.end);
                }
                Err(Self::err(ColTypeNoArgs {
                    span: Self::err_span(span.start, span.end),
                }))
            }
            node => Err(Self::err(ColInvalType {
                span: Self::err_span(node.span().start, node.span().end),
            })),
        }
    }

    fn resolve_col_type(ty: &AstNode) -> Result<(CsvColType, bool), LangErr> {
        match ty {
            // Column type must always be a function call.
            AstNode::Call((Named(name), Compound { children, span })) => match name.as_str() {
                SCHEMA_FN_OPTIONAL => {
                    if children.len() == 1 {
                        return Ok((Self::resolve_literal_type(children.first().unwrap())?, true));
                    }
                    Err(Self::err(OptArgsLen {
                        span: Self::err_span(span.start, span.end),
                    }))
                }
                _ => Ok((Self::resolve_literal_type(ty)?, false)),
            },
            node => Err(Self::err(ColInvalType {
                span: Self::err_span(node.span().start, node.span().end),
            })),
        }
    }

    fn resolve_row(call: &Compound) -> Result<CsvResolvedSchema, LangErr> {
        let row_args_len = call.children.len();
        let start = call.span.start;
        let end = call.span.end;

        // Check if we have even number of arguments.
        if !row_args_len.is_multiple_of(2) || row_args_len == 0 {
            return Err(Self::err(RowArgsLen {
                span: Self::err_span(start, end),
            }));
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

    pub fn resolve(&self) -> Result<CsvResolvedSchema, LangErr> {
        // Root refers to a first function call that defines a schema.
        let root = self.schema_ast.first().ok_or_else(|| {
            Self::err(RootInval {
                span: Self::err_span(1, 1),
            })
        })?;

        // Extract root node descriptor if it matches type and name.
        let root_call = match root {
            AstNode::Call((Named(name), call)) if name == SCHEMA_FN_ROOT => call,
            node => {
                return Err(Self::err(RootInval {
                    span: Self::err_span(node.span().start, node.span().end),
                }));
            }
        };

        // Root call should have only one children.
        match root_call.children.len() {
            1 => {}
            _ => {
                return Err(Self::err(RootArgsLen {
                    span: Self::err_span(root_call.span.start, root_call.span.end),
                }));
            }
        }

        let row = root_call.children.first().unwrap();

        match &**row {
            AstNode::Call((Named(name), call)) if name == SCHEMA_FN_ROW => Self::resolve_row(call),
            node => Err(Self::err(RowInval {
                span: Self::err_span(node.span().start, node.span().end),
            })),
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
    use crate::schema_resolver::{
        CsvColDescriptor, CsvColType, CsvResolvedSchema, CsvSchemaResolver,
    };
    use elise_ast::{AstNode, CallKind::*, Compound, Primitive};
    use elise_builtins::schema::{
        SCHEMA_FN_BOOL, SCHEMA_FN_NUMBER, SCHEMA_FN_OPTIONAL, SCHEMA_FN_ROOT, SCHEMA_FN_ROW,
        SCHEMA_FN_STRING,
    };
    use elise_errors::{LangErr, errors_csv_schema_resolver::CsvSchemaResolverErr::*};
    use elise_types::Span;

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
        let err = Err(LangErr::CsvSchemaResolver(RootInval {
            span: Span { start: 1, end: 1 },
        }));
        assert_eq!(result, err);
    }

    #[test]
    fn root_should_return_error_if_invalid_call() {
        let ast = vec![AstNode::Call((
            Named("invalid".to_string()),
            Compound {
                span: Span { start: 0, end: 3 },
                children: vec![],
            },
        ))];
        let result = CsvSchemaResolver::new(&ast).resolve();
        let err = Err(LangErr::CsvSchemaResolver(RootInval {
            span: Span { start: 0, end: 3 },
        }));
        assert_eq!(result, err);
    }

    #[test]
    fn root_should_return_error_if_not_a_call() {
        let ast = vec![AstNode::Number(Primitive {
            span: Span { start: 0, end: 3 },
            value: "123".to_string(),
        })];
        let result = CsvSchemaResolver::new(&ast).resolve();
        let err = Err(LangErr::CsvSchemaResolver(RootInval {
            span: Span { start: 0, end: 3 },
        }));
        assert_eq!(result, err);
    }

    #[test]
    fn root_should_return_error_if_anon_call() {
        let ast = vec![AstNode::Call((
            Anon,
            Compound {
                span: Span { start: 0, end: 3 },
                children: vec![],
            },
        ))];
        let result = CsvSchemaResolver::new(&ast).resolve();
        let err = Err(LangErr::CsvSchemaResolver(RootInval {
            span: Span { start: 0, end: 3 },
        }));
        assert_eq!(result, err);
    }

    #[test]
    fn root_should_return_error_if_no_args() {
        let ast = vec![AstNode::Call((
            Named(SCHEMA_FN_ROOT.to_string()),
            Compound {
                span: Span { start: 0, end: 3 },
                children: vec![],
            },
        ))];
        let result = CsvSchemaResolver::new(&ast).resolve();
        let err = Err(LangErr::CsvSchemaResolver(RootArgsLen {
            span: Span { start: 0, end: 3 },
        }));
        assert_eq!(result, err);
    }

    #[test]
    fn root_should_return_error_if_more_than_one_arg() {
        let row_def = Box::new(AstNode::Call((
            Named(SCHEMA_FN_ROW.to_string()),
            Compound {
                span: Span { start: 3, end: 6 },
                children: vec![],
            },
        )));
        let redundant_def = Box::new(AstNode::Call((
            Named("row2".to_string()),
            Compound {
                span: Span { start: 6, end: 9 },
                children: vec![],
            },
        )));
        let ast = vec![AstNode::Call((
            Named(SCHEMA_FN_ROOT.to_string()),
            Compound {
                span: Span { start: 0, end: 11 },
                children: vec![row_def, redundant_def],
            },
        ))];
        let result = CsvSchemaResolver::new(&ast).resolve();
        let err = Err(LangErr::CsvSchemaResolver(RootArgsLen {
            span: Span { start: 0, end: 11 },
        }));
        assert_eq!(result, err);
    }

    // TESTS ROOT END

    // TESTS ROW START

    #[test]
    fn row_should_return_error_if_not_a_call() {
        let row_def = Box::new(AstNode::Number(Primitive {
            value: "2".to_string(),
            span: Span { start: 3, end: 6 },
        }));
        let ast = vec![AstNode::Call((
            Named(SCHEMA_FN_ROOT.to_string()),
            Compound {
                span: Span { start: 0, end: 8 },
                children: vec![row_def],
            },
        ))];
        let result = CsvSchemaResolver::new(&ast).resolve();
        let err = Err(LangErr::CsvSchemaResolver(RowInval {
            span: Span { start: 3, end: 6 },
        }));
        assert_eq!(result, err);
    }

    #[test]
    fn row_should_return_error_if_invalid_call() {
        let row_def = Box::new(AstNode::Call((
            Named("invalid".to_string()),
            Compound {
                span: Span { start: 3, end: 6 },
                children: vec![],
            },
        )));
        let ast = vec![AstNode::Call((
            Named(SCHEMA_FN_ROOT.to_string()),
            Compound {
                span: Span { start: 0, end: 8 },
                children: vec![row_def],
            },
        ))];
        let result = CsvSchemaResolver::new(&ast).resolve();
        let err = Err(LangErr::CsvSchemaResolver(RowInval {
            span: Span { start: 3, end: 6 },
        }));
        assert_eq!(result, err);
    }

    #[test]
    fn row_should_return_error_if_no_args() {
        let row_def = Box::new(AstNode::Call((
            Named(SCHEMA_FN_ROW.to_string()),
            Compound {
                span: Span { start: 3, end: 6 },
                children: vec![],
            },
        )));
        let ast = vec![AstNode::Call((
            Named(SCHEMA_FN_ROOT.to_string()),
            Compound {
                span: Span { start: 0, end: 3 },
                children: vec![row_def],
            },
        ))];
        let result = CsvSchemaResolver::new(&ast).resolve();
        let err = Err(LangErr::CsvSchemaResolver(RowArgsLen {
            span: Span { start: 3, end: 6 },
        }));
        assert_eq!(result, err);
    }

    #[test]
    fn row_should_return_error_if_args_not_even() {
        let row_def = Box::new(AstNode::Call((
            Named(SCHEMA_FN_ROW.to_string()),
            Compound {
                span: Span { start: 3, end: 6 },
                children: vec![Box::new(AstNode::Identifier(Primitive {
                    value: "some_value".to_string(),
                    span: Span { start: 9, end: 12 },
                }))],
            },
        )));
        let ast = vec![AstNode::Call((
            Named(SCHEMA_FN_ROOT.to_string()),
            Compound {
                span: Span { start: 0, end: 3 },
                children: vec![row_def],
            },
        ))];
        let result = CsvSchemaResolver::new(&ast).resolve();
        let err = Err(LangErr::CsvSchemaResolver(RowArgsLen {
            span: Span { start: 3, end: 6 },
        }));
        assert_eq!(result, err);
    }

    #[test]
    fn row_should_return_error_if_odd_args_not_identifiers() {
        let row_children = vec![
            Box::new(AstNode::Number(Primitive {
                value: "4".to_string(),
                span: Span { start: 9, end: 12 },
            })),
            Box::new(AstNode::Call((
                Named(SCHEMA_FN_NUMBER.to_string()),
                Compound {
                    children: vec![],
                    span: Span { start: 12, end: 15 },
                },
            ))),
        ];
        let row_def = Box::new(AstNode::Call((
            Named(SCHEMA_FN_ROW.to_string()),
            Compound {
                span: Span { start: 3, end: 6 },
                children: row_children,
            },
        )));
        let ast = vec![AstNode::Call((
            Named(SCHEMA_FN_ROOT.to_string()),
            Compound {
                span: Span { start: 0, end: 3 },
                children: vec![row_def],
            },
        ))];
        let result = CsvSchemaResolver::new(&ast).resolve();
        let err = Err(LangErr::CsvSchemaResolver(ColInvalName {
            span: Span { start: 9, end: 12 },
        }));
        assert_eq!(result, err);
    }

    #[test]
    fn row_should_return_error_if_even_args_not_known_calls() {
        let row_children = vec![
            Box::new(AstNode::Identifier(Primitive {
                value: "name".to_string(),
                span: Span { start: 9, end: 12 },
            })),
            Box::new(AstNode::Call((
                Named("some".to_string()),
                Compound {
                    children: vec![],
                    span: Span { start: 12, end: 15 },
                },
            ))),
        ];
        let row_def = Box::new(AstNode::Call((
            Named(SCHEMA_FN_ROW.to_string()),
            Compound {
                span: Span { start: 3, end: 6 },
                children: row_children,
            },
        )));
        let ast = vec![AstNode::Call((
            Named(SCHEMA_FN_ROOT.to_string()),
            Compound {
                span: Span { start: 0, end: 3 },
                children: vec![row_def],
            },
        ))];
        let result = CsvSchemaResolver::new(&ast).resolve();
        let err = Err(LangErr::CsvSchemaResolver(ColInvalType {
            span: Span { start: 12, end: 15 },
        }));
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
            Box::new(AstNode::Identifier(Primitive {
                value: "name".to_string(),
                span: Span { start: 9, end: 12 },
            })),
            Box::new(AstNode::Call((
                Named(SCHEMA_FN_NUMBER.to_string()),
                Compound {
                    children: vec![],
                    span: Span { start: 12, end: 15 },
                },
            ))),
        ];
        let row_def = Box::new(AstNode::Call((
            Named(SCHEMA_FN_ROW.to_string()),
            Compound {
                span: Span { start: 3, end: 6 },
                children: row_children,
            },
        )));
        let ast = vec![AstNode::Call((
            Named(SCHEMA_FN_ROOT.to_string()),
            Compound {
                span: Span { start: 0, end: 3 },
                children: vec![row_def],
            },
        ))];
        let result = CsvSchemaResolver::new(&ast).resolve();
        let resolved = CsvResolvedSchema {
            row: vec![CsvColDescriptor {
                name: "name".to_string(),
                ty: CsvColType::Number,
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
    fn optional_should_resolve() {
        let opt_children = vec![Box::new(AstNode::Call((
            Named(SCHEMA_FN_NUMBER.to_string()),
            Compound {
                children: vec![],
                span: Span { start: 12, end: 15 },
            },
        )))];
        let type_opt = Box::new(AstNode::Call((
            Named(SCHEMA_FN_OPTIONAL.to_string()),
            Compound {
                children: opt_children,
                span: Span { start: 15, end: 18 },
            },
        )));
        let row_children = vec![
            Box::new(AstNode::Identifier(Primitive {
                value: "name".to_string(),
                span: Span { start: 9, end: 12 },
            })),
            type_opt,
        ];
        let row_def = Box::new(AstNode::Call((
            Named(SCHEMA_FN_ROW.to_string()),
            Compound {
                span: Span { start: 3, end: 6 },
                children: row_children,
            },
        )));
        let ast = vec![AstNode::Call((
            Named(SCHEMA_FN_ROOT.to_string()),
            Compound {
                span: Span { start: 0, end: 3 },
                children: vec![row_def],
            },
        ))];
        let result = CsvSchemaResolver::new(&ast).resolve();
        let resolved = CsvResolvedSchema {
            row: vec![CsvColDescriptor {
                name: "name".to_string(),
                ty: CsvColType::Number,
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
            Box::new(AstNode::Identifier(Primitive {
                value: "age".to_string(),
                span: Span { start: 9, end: 12 },
            })),
            Box::new(AstNode::Call((
                Named(SCHEMA_FN_NUMBER.to_string()),
                Compound {
                    children: vec![],
                    span: Span { start: 12, end: 15 },
                },
            ))),
        ];
        let row_def = Box::new(AstNode::Call((
            Named(SCHEMA_FN_ROW.to_string()),
            Compound {
                span: Span { start: 3, end: 6 },
                children: row_children,
            },
        )));
        let ast = vec![AstNode::Call((
            Named(SCHEMA_FN_ROOT.to_string()),
            Compound {
                span: Span { start: 0, end: 3 },
                children: vec![row_def],
            },
        ))];
        let result = CsvSchemaResolver::new(&ast).resolve();
        let resolved = CsvResolvedSchema {
            row: vec![CsvColDescriptor {
                name: "age".to_string(),
                ty: CsvColType::Number,
                opt: false,
            }],
        };
        assert_eq!(result, Ok(resolved));
    }

    #[test]
    fn number_should_return_error_if_has_args() {
        let row_children = vec![
            Box::new(AstNode::Identifier(Primitive {
                value: "name".to_string(),
                span: Span { start: 9, end: 12 },
            })),
            Box::new(AstNode::Call((
                Named(SCHEMA_FN_NUMBER.to_string()),
                Compound {
                    children: vec![Box::new(AstNode::Number(Primitive {
                        value: "1".to_string(),
                        span: Span { start: 0, end: 3 },
                    }))],
                    span: Span { start: 12, end: 15 },
                },
            ))),
        ];
        let row_def = Box::new(AstNode::Call((
            Named(SCHEMA_FN_ROW.to_string()),
            Compound {
                span: Span { start: 3, end: 6 },
                children: row_children,
            },
        )));
        let ast = vec![AstNode::Call((
            Named(SCHEMA_FN_ROOT.to_string()),
            Compound {
                span: Span { start: 0, end: 3 },
                children: vec![row_def],
            },
        ))];
        let result = CsvSchemaResolver::new(&ast).resolve();
        let err = Err(LangErr::CsvSchemaResolver(ColTypeNoArgs {
            span: Span { start: 12, end: 15 },
        }));
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
            Box::new(AstNode::Identifier(Primitive {
                value: "name".to_string(),
                span: Span { start: 9, end: 12 },
            })),
            Box::new(AstNode::Call((
                Named(SCHEMA_FN_STRING.to_string()),
                Compound {
                    children: vec![],
                    span: Span { start: 12, end: 15 },
                },
            ))),
        ];
        let row_def = Box::new(AstNode::Call((
            Named(SCHEMA_FN_ROW.to_string()),
            Compound {
                span: Span { start: 3, end: 6 },
                children: row_children,
            },
        )));
        let ast = vec![AstNode::Call((
            Named(SCHEMA_FN_ROOT.to_string()),
            Compound {
                span: Span { start: 0, end: 3 },
                children: vec![row_def],
            },
        ))];
        let result = CsvSchemaResolver::new(&ast).resolve();
        let resolved = CsvResolvedSchema {
            row: vec![CsvColDescriptor {
                name: "name".to_string(),
                ty: CsvColType::String,
                opt: false,
            }],
        };
        assert_eq!(result, Ok(resolved));
    }

    #[test]
    fn string_should_return_error_if_has_args() {
        let row_children = vec![
            Box::new(AstNode::Identifier(Primitive {
                value: "name".to_string(),
                span: Span { start: 9, end: 12 },
            })),
            Box::new(AstNode::Call((
                Named(SCHEMA_FN_STRING.to_string()),
                Compound {
                    children: vec![Box::new(AstNode::Number(Primitive {
                        value: "1".to_string(),
                        span: Span { start: 0, end: 3 },
                    }))],
                    span: Span { start: 12, end: 15 },
                },
            ))),
        ];
        let row_def = Box::new(AstNode::Call((
            Named(SCHEMA_FN_ROW.to_string()),
            Compound {
                span: Span { start: 3, end: 6 },
                children: row_children,
            },
        )));
        let ast = vec![AstNode::Call((
            Named(SCHEMA_FN_ROOT.to_string()),
            Compound {
                span: Span { start: 0, end: 3 },
                children: vec![row_def],
            },
        ))];
        let result = CsvSchemaResolver::new(&ast).resolve();
        let err = Err(LangErr::CsvSchemaResolver(ColTypeNoArgs {
            span: Span { start: 12, end: 15 },
        }));
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
            Box::new(AstNode::Identifier(Primitive {
                value: "employed".to_string(),
                span: Span { start: 9, end: 12 },
            })),
            Box::new(AstNode::Call((
                Named(SCHEMA_FN_BOOL.to_string()),
                Compound {
                    children: vec![],
                    span: Span { start: 12, end: 15 },
                },
            ))),
        ];
        let row_def = Box::new(AstNode::Call((
            Named(SCHEMA_FN_ROW.to_string()),
            Compound {
                span: Span { start: 3, end: 6 },
                children: row_children,
            },
        )));
        let ast = vec![AstNode::Call((
            Named(SCHEMA_FN_ROOT.to_string()),
            Compound {
                span: Span { start: 0, end: 3 },
                children: vec![row_def],
            },
        ))];
        let result = CsvSchemaResolver::new(&ast).resolve();
        let resolved = CsvResolvedSchema {
            row: vec![CsvColDescriptor {
                name: "employed".to_string(),
                ty: CsvColType::Bool,
                opt: false,
            }],
        };
        assert_eq!(result, Ok(resolved));
    }

    #[test]
    fn boolean_should_return_error_if_has_args() {
        let row_children = vec![
            Box::new(AstNode::Identifier(Primitive {
                value: "name".to_string(),
                span: Span { start: 9, end: 12 },
            })),
            Box::new(AstNode::Call((
                Named(SCHEMA_FN_BOOL.to_string()),
                Compound {
                    children: vec![Box::new(AstNode::Number(Primitive {
                        value: "1".to_string(),
                        span: Span { start: 0, end: 3 },
                    }))],
                    span: Span { start: 12, end: 15 },
                },
            ))),
        ];
        let row_def = Box::new(AstNode::Call((
            Named(SCHEMA_FN_ROW.to_string()),
            Compound {
                span: Span { start: 3, end: 6 },
                children: row_children,
            },
        )));
        let ast = vec![AstNode::Call((
            Named(SCHEMA_FN_ROOT.to_string()),
            Compound {
                span: Span { start: 0, end: 3 },
                children: vec![row_def],
            },
        ))];
        let result = CsvSchemaResolver::new(&ast).resolve();
        let err = Err(LangErr::CsvSchemaResolver(ColTypeNoArgs {
            span: Span { start: 12, end: 15 },
        }));
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
