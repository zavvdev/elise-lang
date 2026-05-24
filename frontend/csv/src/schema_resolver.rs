use elise_ast::{AstNode, CallKind::*, Compound, Primitive};

use elise_builtins::schema::{
    SCHEMA_FN_BOOL, SCHEMA_FN_NUMBER, SCHEMA_FN_OPTIONAL, SCHEMA_FN_ROOT, SCHEMA_FN_ROW,
    SCHEMA_FN_STRING,
};
use elise_errors::{
    LangErr,
    errors_csv_schema_resolver::{
        CsvSchemaResolverErr, CsvSchemaResolverErr::*, CsvSchemaResolverErrPos,
    },
};

#[derive(Debug, PartialEq)]
enum CsvColType {
    Number,
    String,
    Bool,
}

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

    fn err_pos(start: usize, end: usize) -> CsvSchemaResolverErrPos {
        CsvSchemaResolverErrPos { start, end }
    }

    fn resolve_type(call_name: &str, start: usize, end: usize) -> Result<CsvColType, LangErr> {
        match call_name {
            SCHEMA_FN_BOOL => Ok(CsvColType::Bool),
            SCHEMA_FN_NUMBER => Ok(CsvColType::Number),
            SCHEMA_FN_STRING => Ok(CsvColType::String),
            _ => Err(Self::err(ColInvalType {
                pos: Self::err_pos(start, end),
            })),
        }
    }

    fn resolve_col_name(col: &AstNode) -> Result<String, LangErr> {
        match col {
            // Column name must always be an identifier type.
            AstNode::Identifier(Primitive { value, span: _ }) => Ok(value.clone()),
            node => Err(Self::err(ColInvalName {
                pos: Self::err_pos(node.span().start, node.span().end),
            })),
        }
    }

    fn resolve_literal_type(node: &AstNode) -> Result<CsvColType, LangErr> {
        match node {
            AstNode::Call((Named(name), Compound { children, span })) => {
                if children.is_empty() {
                    return Self::resolve_type(name, span.start, span.end);
                }
                Err(Self::err(TypeNoArgs {
                    pos: Self::err_pos(span.start, span.end),
                }))
            }
            node => Err(Self::err(ColInvalType {
                pos: Self::err_pos(node.span().start, node.span().end),
            })),
        }
    }

    fn resolve_col_type(ty: &AstNode) -> Result<(CsvColType, bool), LangErr> {
        match ty {
            // Column type must always be a function call.
            AstNode::Call((Named(name), Compound { children, span })) => match name.as_str() {
                SCHEMA_FN_OPTIONAL => {
                    if children.len() != 1 {
                        return Err(Self::err(OptInvalArgsLen {
                            pos: Self::err_pos(span.start, span.end),
                        }));
                    }
                    Ok((
                        Self::resolve_literal_type(children.first().unwrap())?,
                        true,
                    ))
                }
                _ => Ok((Self::resolve_literal_type(ty)?, false)),
            },
            node => Err(Self::err(ColInvalType {
                pos: Self::err_pos(node.span().start, node.span().end),
            })),
        }
    }

    fn resolve_row(call: &Compound) -> Result<CsvResolvedSchema, LangErr> {
        let row_args_len = call.children.len();
        let start = call.span.start;
        let end = call.span.end;

        if row_args_len == 0 {
            return Err(Self::err(RowEmpty {
                pos: Self::err_pos(start, end),
            }));
        }

        // Check if we have even number of arguments.
        if !row_args_len.is_multiple_of(2) {
            return Err(Self::err(RowInvalArgsLen {
                pos: Self::err_pos(start, end),
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
            let col = *cols.get(index).ok_or_else(|| {
                Self::err(RowInvalArgs {
                    pos: Self::err_pos(start, end),
                })
            })?;

            let ty = *types.get(index).ok_or_else(|| {
                Self::err(RowInvalArgs {
                    pos: Self::err_pos(start, end),
                })
            })?;

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
        let root = self
            .schema_ast
            .first()
            .ok_or_else(|| Self::err(RootMissing))?;

        // Extract root node descriptor if it matches type and name.
        let root_call = match root {
            AstNode::Call((Named(name), call)) if name == SCHEMA_FN_ROOT => call,
            node => {
                return Err(Self::err(RootInval {
                    pos: Self::err_pos(node.span().start, node.span().end),
                }));
            }
        };

        // Root call should have only one children.
        match root_call.children.len() {
            0 => {
                return Err(Self::err(RootNoArgs {
                    pos: Self::err_pos(root_call.span.start, root_call.span.end),
                }));
            }
            2.. => {
                return Err(Self::err(RootTooManyArgs {
                    pos: Self::err_pos(root_call.span.start, root_call.span.end),
                }));
            }
            _ => {}
        }

        // Extract the first argument of the root call.
        let row = root_call.children.first().ok_or_else(|| {
            Self::err(RootNoArgs {
                pos: Self::err_pos(root_call.span.start, root_call.span.end),
            })
        })?;

        // Match row call name and resolve it.
        match &**row {
            AstNode::Call((Named(name), call)) if name == SCHEMA_FN_ROW => Self::resolve_row(call),
            node => Err(Self::err(RowInval {
                pos: Self::err_pos(node.span().start, node.span().end),
            })),
        }
    }
}

// ==========================
//
//  TESTS START
//
// ==========================

#[cfg(test)]
mod tests {
    use crate::schema_resolver::{
        CsvColDescriptor, CsvColType, CsvResolvedSchema, CsvSchemaResolver,
    };
    use elise_ast::{AstNode, CallKind::*, Compound, Primitive, TokSpan};
    use elise_builtins::schema::{
        SCHEMA_FN_BOOL, SCHEMA_FN_NUMBER, SCHEMA_FN_OPTIONAL, SCHEMA_FN_ROOT, SCHEMA_FN_ROW,
        SCHEMA_FN_STRING,
    };
    use elise_errors::{
        LangErr,
        errors_csv_schema_resolver::{CsvSchemaResolverErr, CsvSchemaResolverErrPos},
    };

    #[test]
    fn should_return_error_if_schema_file_is_empty() {
        let ast = vec![];
        let result = CsvSchemaResolver::new(&ast).resolve();
        assert_eq!(
            result,
            Err(LangErr::CsvSchemaResolver(
                CsvSchemaResolverErr::RootMissing
            ))
        );
    }

    #[test]
    fn should_return_error_if_root_is_invalid() {
        let ast = vec![AstNode::Call((
            Named("invalid".to_string()),
            Compound {
                // We don't care about TokSpan values here since
                // we just need to make sure that CsvSchemaResolvedErrPos
                // has the same values.
                span: TokSpan { start: 0, end: 3 },
                children: vec![],
            },
        ))];
        let result = CsvSchemaResolver::new(&ast).resolve();
        assert_eq!(
            result,
            Err(LangErr::CsvSchemaResolver(
                CsvSchemaResolverErr::RootInval {
                    pos: CsvSchemaResolverErrPos { start: 0, end: 3 }
                }
            ))
        );
    }

    #[test]
    fn should_return_error_if_root_is_not_a_call() {
        let ast = vec![AstNode::Number(Primitive {
            span: TokSpan { start: 0, end: 3 },
            value: "123".to_string(),
        })];
        let result = CsvSchemaResolver::new(&ast).resolve();
        assert_eq!(
            result,
            Err(LangErr::CsvSchemaResolver(
                CsvSchemaResolverErr::RootInval {
                    pos: CsvSchemaResolverErrPos { start: 0, end: 3 }
                }
            ))
        );
    }

    #[test]
    fn should_return_error_if_root_is_anon() {
        let ast = vec![AstNode::Call((
            Anon,
            Compound {
                span: TokSpan { start: 0, end: 3 },
                children: vec![],
            },
        ))];
        let result = CsvSchemaResolver::new(&ast).resolve();
        assert_eq!(
            result,
            Err(LangErr::CsvSchemaResolver(
                CsvSchemaResolverErr::RootInval {
                    pos: CsvSchemaResolverErrPos { start: 0, end: 3 }
                }
            ))
        );
    }

    #[test]
    fn should_return_error_if_root_has_no_args() {
        let ast = vec![AstNode::Call((
            Named(SCHEMA_FN_ROOT.to_string()),
            Compound {
                span: TokSpan { start: 0, end: 3 },
                children: vec![],
            },
        ))];
        let result = CsvSchemaResolver::new(&ast).resolve();
        assert_eq!(
            result,
            Err(LangErr::CsvSchemaResolver(
                CsvSchemaResolverErr::RootNoArgs {
                    pos: CsvSchemaResolverErrPos { start: 0, end: 3 }
                }
            ))
        );
    }

    #[test]
    fn should_return_error_if_root_has_more_than_one_arg() {
        let row_def = Box::new(AstNode::Call((
            Named(SCHEMA_FN_ROW.to_string()),
            Compound {
                span: TokSpan { start: 3, end: 6 },
                children: vec![],
            },
        )));
        let redundant_def = Box::new(AstNode::Call((
            Named("row2".to_string()),
            Compound {
                span: TokSpan { start: 6, end: 9 },
                children: vec![],
            },
        )));
        let ast = vec![AstNode::Call((
            Named(SCHEMA_FN_ROOT.to_string()),
            Compound {
                span: TokSpan { start: 0, end: 11 },
                children: vec![row_def, redundant_def],
            },
        ))];
        let result = CsvSchemaResolver::new(&ast).resolve();
        let err = CsvSchemaResolverErr::RootTooManyArgs {
            pos: CsvSchemaResolverErrPos { start: 0, end: 11 },
        };
        assert_eq!(result, Err(LangErr::CsvSchemaResolver(err)));
    }

    #[test]
    fn should_return_error_if_row_is_not_a_call() {
        let row_def = Box::new(AstNode::Number(Primitive {
            value: "2".to_string(),
            span: TokSpan { start: 3, end: 6 },
        }));
        let ast = vec![AstNode::Call((
            Named(SCHEMA_FN_ROOT.to_string()),
            Compound {
                span: TokSpan { start: 0, end: 8 },
                children: vec![row_def],
            },
        ))];
        let result = CsvSchemaResolver::new(&ast).resolve();
        let err = CsvSchemaResolverErr::RowInval {
            pos: CsvSchemaResolverErrPos { start: 3, end: 6 },
        };
        assert_eq!(result, Err(LangErr::CsvSchemaResolver(err)));
    }

    #[test]
    fn should_return_error_if_row_is_invalid_call() {
        let row_def = Box::new(AstNode::Call((
            Named("invalid".to_string()),
            Compound {
                span: TokSpan { start: 3, end: 6 },
                children: vec![],
            },
        )));
        let ast = vec![AstNode::Call((
            Named(SCHEMA_FN_ROOT.to_string()),
            Compound {
                span: TokSpan { start: 0, end: 8 },
                children: vec![row_def],
            },
        ))];
        let result = CsvSchemaResolver::new(&ast).resolve();
        let err = CsvSchemaResolverErr::RowInval {
            pos: CsvSchemaResolverErrPos { start: 3, end: 6 },
        };
        assert_eq!(result, Err(LangErr::CsvSchemaResolver(err)));
    }

    #[test]
    fn should_return_error_if_row_has_no_args() {
        let row_def = Box::new(AstNode::Call((
            Named(SCHEMA_FN_ROW.to_string()),
            Compound {
                span: TokSpan { start: 3, end: 6 },
                children: vec![],
            },
        )));
        let ast = vec![AstNode::Call((
            Named(SCHEMA_FN_ROOT.to_string()),
            Compound {
                span: TokSpan { start: 0, end: 3 },
                children: vec![row_def],
            },
        ))];
        let result = CsvSchemaResolver::new(&ast).resolve();
        let err = CsvSchemaResolverErr::RowEmpty {
            pos: CsvSchemaResolverErrPos { start: 3, end: 6 },
        };
        assert_eq!(result, Err(LangErr::CsvSchemaResolver(err)));
    }

    #[test]
    fn should_return_error_if_number_of_row_def_args_is_not_even() {
        let row_def = Box::new(AstNode::Call((
            Named(SCHEMA_FN_ROW.to_string()),
            Compound {
                span: TokSpan { start: 3, end: 6 },
                children: vec![Box::new(AstNode::Identifier(Primitive {
                    value: "some_value".to_string(),
                    span: TokSpan { start: 9, end: 12 },
                }))],
            },
        )));
        let ast = vec![AstNode::Call((
            Named(SCHEMA_FN_ROOT.to_string()),
            Compound {
                span: TokSpan { start: 0, end: 3 },
                children: vec![row_def],
            },
        ))];
        let result = CsvSchemaResolver::new(&ast).resolve();
        let err = CsvSchemaResolverErr::RowInvalArgsLen {
            pos: CsvSchemaResolverErrPos { start: 3, end: 6 },
        };
        assert_eq!(result, Err(LangErr::CsvSchemaResolver(err)));
    }

    #[test]
    fn should_return_error_if_odd_row_args_are_not_identifiers() {
        let row_children = vec![
            Box::new(AstNode::Number(Primitive {
                value: "4".to_string(),
                span: TokSpan { start: 9, end: 12 },
            })),
            Box::new(AstNode::Call((
                Named(SCHEMA_FN_NUMBER.to_string()),
                Compound {
                    children: vec![],
                    span: TokSpan { start: 12, end: 15 },
                },
            ))),
        ];
        let row_def = Box::new(AstNode::Call((
            Named(SCHEMA_FN_ROW.to_string()),
            Compound {
                span: TokSpan { start: 3, end: 6 },
                children: row_children,
            },
        )));
        let ast = vec![AstNode::Call((
            Named(SCHEMA_FN_ROOT.to_string()),
            Compound {
                span: TokSpan { start: 0, end: 3 },
                children: vec![row_def],
            },
        ))];
        let result = CsvSchemaResolver::new(&ast).resolve();
        let err = CsvSchemaResolverErr::ColInvalName {
            pos: CsvSchemaResolverErrPos { start: 9, end: 12 },
        };
        assert_eq!(result, Err(LangErr::CsvSchemaResolver(err)));
    }

    #[test]
    fn should_return_error_if_even_row_args_are_not_known_fn_calls() {
        let row_children = vec![
            Box::new(AstNode::Identifier(Primitive {
                value: "name".to_string(),
                span: TokSpan { start: 9, end: 12 },
            })),
            Box::new(AstNode::Call((
                Named("some".to_string()),
                Compound {
                    children: vec![],
                    span: TokSpan { start: 12, end: 15 },
                },
            ))),
        ];
        let row_def = Box::new(AstNode::Call((
            Named(SCHEMA_FN_ROW.to_string()),
            Compound {
                span: TokSpan { start: 3, end: 6 },
                children: row_children,
            },
        )));
        let ast = vec![AstNode::Call((
            Named(SCHEMA_FN_ROOT.to_string()),
            Compound {
                span: TokSpan { start: 0, end: 3 },
                children: vec![row_def],
            },
        ))];
        let result = CsvSchemaResolver::new(&ast).resolve();
        let err = CsvSchemaResolverErr::ColInvalType {
            pos: CsvSchemaResolverErrPos { start: 12, end: 15 },
        };
        assert_eq!(result, Err(LangErr::CsvSchemaResolver(err)));
    }

    #[test]
    fn should_return_error_if_type_fns_have_args() {
        let row_children = vec![
            Box::new(AstNode::Identifier(Primitive {
                value: "name".to_string(),
                span: TokSpan { start: 9, end: 12 },
            })),
            Box::new(AstNode::Call((
                Named("number".to_string()),
                Compound {
                    children: vec![Box::new(AstNode::Number(Primitive {
                        value: "1".to_string(),
                        span: TokSpan { start: 0, end: 3 },
                    }))],
                    span: TokSpan { start: 12, end: 15 },
                },
            ))),
        ];
        let row_def = Box::new(AstNode::Call((
            Named(SCHEMA_FN_ROW.to_string()),
            Compound {
                span: TokSpan { start: 3, end: 6 },
                children: row_children,
            },
        )));
        let ast = vec![AstNode::Call((
            Named(SCHEMA_FN_ROOT.to_string()),
            Compound {
                span: TokSpan { start: 0, end: 3 },
                children: vec![row_def],
            },
        ))];
        let result = CsvSchemaResolver::new(&ast).resolve();
        let err = CsvSchemaResolverErr::TypeNoArgs {
            pos: CsvSchemaResolverErrPos { start: 12, end: 15 },
        };
        assert_eq!(result, Err(LangErr::CsvSchemaResolver(err)));
    }

    #[test]
    fn should_resolve_schema_req_value() {
        let row_children = vec![
            Box::new(AstNode::Identifier(Primitive {
                value: "name".to_string(),
                span: TokSpan { start: 9, end: 12 },
            })),
            Box::new(AstNode::Call((
                Named(SCHEMA_FN_NUMBER.to_string()),
                Compound {
                    children: vec![],
                    span: TokSpan { start: 12, end: 15 },
                },
            ))),
        ];
        let row_def = Box::new(AstNode::Call((
            Named(SCHEMA_FN_ROW.to_string()),
            Compound {
                span: TokSpan { start: 3, end: 6 },
                children: row_children,
            },
        )));
        let ast = vec![AstNode::Call((
            Named(SCHEMA_FN_ROOT.to_string()),
            Compound {
                span: TokSpan { start: 0, end: 3 },
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

    #[test]
    fn should_resolve_schema_opt_value() {
        let type_opt = Box::new(AstNode::Call((
            Named(SCHEMA_FN_OPTIONAL.to_string()),
            Compound {
                children: vec![Box::new(AstNode::Call((
                    Named(SCHEMA_FN_NUMBER.to_string()),
                    Compound {
                        children: vec![],
                        span: TokSpan { start: 12, end: 15 },
                    },
                )))],
                span: TokSpan { start: 15, end: 18 },
            },
        )));
        let row_children = vec![
            Box::new(AstNode::Identifier(Primitive {
                value: "name".to_string(),
                span: TokSpan { start: 9, end: 12 },
            })),
            type_opt,
        ];
        let row_def = Box::new(AstNode::Call((
            Named(SCHEMA_FN_ROW.to_string()),
            Compound {
                span: TokSpan { start: 3, end: 6 },
                children: row_children,
            },
        )));
        let ast = vec![AstNode::Call((
            Named(SCHEMA_FN_ROOT.to_string()),
            Compound {
                span: TokSpan { start: 0, end: 3 },
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

    #[test]
    fn should_resolve_number() {
        let row_children = vec![
            Box::new(AstNode::Identifier(Primitive {
                value: "age".to_string(),
                span: TokSpan { start: 9, end: 12 },
            })),
            Box::new(AstNode::Call((
                Named(SCHEMA_FN_NUMBER.to_string()),
                Compound {
                    children: vec![],
                    span: TokSpan { start: 12, end: 15 },
                },
            ))),
        ];
        let row_def = Box::new(AstNode::Call((
            Named(SCHEMA_FN_ROW.to_string()),
            Compound {
                span: TokSpan { start: 3, end: 6 },
                children: row_children,
            },
        )));
        let ast = vec![AstNode::Call((
            Named(SCHEMA_FN_ROOT.to_string()),
            Compound {
                span: TokSpan { start: 0, end: 3 },
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
    fn should_resolve_string() {
        let row_children = vec![
            Box::new(AstNode::Identifier(Primitive {
                value: "name".to_string(),
                span: TokSpan { start: 9, end: 12 },
            })),
            Box::new(AstNode::Call((
                Named(SCHEMA_FN_STRING.to_string()),
                Compound {
                    children: vec![],
                    span: TokSpan { start: 12, end: 15 },
                },
            ))),
        ];
        let row_def = Box::new(AstNode::Call((
            Named(SCHEMA_FN_ROW.to_string()),
            Compound {
                span: TokSpan { start: 3, end: 6 },
                children: row_children,
            },
        )));
        let ast = vec![AstNode::Call((
            Named(SCHEMA_FN_ROOT.to_string()),
            Compound {
                span: TokSpan { start: 0, end: 3 },
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
    fn should_resolve_boolean() {
        let row_children = vec![
            Box::new(AstNode::Identifier(Primitive {
                value: "employed".to_string(),
                span: TokSpan { start: 9, end: 12 },
            })),
            Box::new(AstNode::Call((
                Named(SCHEMA_FN_BOOL.to_string()),
                Compound {
                    children: vec![],
                    span: TokSpan { start: 12, end: 15 },
                },
            ))),
        ];
        let row_def = Box::new(AstNode::Call((
            Named(SCHEMA_FN_ROW.to_string()),
            Compound {
                span: TokSpan { start: 3, end: 6 },
                children: row_children,
            },
        )));
        let ast = vec![AstNode::Call((
            Named(SCHEMA_FN_ROOT.to_string()),
            Compound {
                span: TokSpan { start: 0, end: 3 },
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
}

// ==========================
//
//  TESTS END
//
// ==========================
