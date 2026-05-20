use elise_ast::{AstNode, CallKind};

use elise_builtins::schema::{SCH_FN_DEF, SCH_FN_ROW};
use elise_errors::{
    LangErr,
    errors_csv_schema_resolver::{CsvSchemaResolverErr, CsvSchemaResolverErrPos},
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

//.schema(
//    .row(
//        name     .string()
//        age      .number()
//        salary   .optional(.number())
//        employed .bool()
//    ))

// 1. [x] Check if schema ast is empty. Otherwise return EmptySchema error
// 2. [x] Check if we have .schema function call at the top level. Otherwise return InvalDef error
// 3. [x] Check if we have a single child which is a .row function call. Otherwise return InvaliRowDef error
// 4. [x] Get the number of columns from .row call. If empty then return EmptyRow error
// 5. [ ] Allocate a vector with length that equals to number of culumns
// 6. [ ] Walk through .row function call children and check:
//      - [x] Number of arguments should be even
//      - [ ] Each odd argument should be an identifier. Otherwise return InvalColName  error
//      - [ ] Each even argument should be a known schema function call (SCH_FN_*). Otherwise return
//           InvalColTypeDef error
//      - [ ] If two prev steps pass then create CsvColDescriptor and insert it into previously
//           created vector

pub struct CsvSchemaResolver<'a> {
    schema_ast: &'a Vec<AstNode>,
}

impl<'a> CsvSchemaResolver<'a> {
    pub fn new(schema_ast: &'a Vec<AstNode>) -> Self {
        Self { schema_ast }
    }

    fn err_unknown() -> LangErr {
        LangErr::CsvSchemaResolver(CsvSchemaResolverErr::Unknown)
    }

    fn err_root_missing() -> LangErr {
        LangErr::CsvSchemaResolver(CsvSchemaResolverErr::RootMissing)
    }

    fn err_root_no_args(start: usize, end: usize) -> LangErr {
        LangErr::CsvSchemaResolver(CsvSchemaResolverErr::RootNoArgs {
            pos: CsvSchemaResolverErrPos { start, end },
        })
    }

    fn err_root_inval(start: usize, end: usize) -> LangErr {
        LangErr::CsvSchemaResolver(CsvSchemaResolverErr::RootInval {
            pos: CsvSchemaResolverErrPos { start, end },
        })
    }

    fn err_root_too_many_args(start: usize, end: usize) -> LangErr {
        LangErr::CsvSchemaResolver(CsvSchemaResolverErr::RootTooManyArgs {
            pos: CsvSchemaResolverErrPos { start, end },
        })
    }

    fn error_row_inval(start: usize, end: usize) -> LangErr {
        LangErr::CsvSchemaResolver(CsvSchemaResolverErr::RowInval {
            pos: CsvSchemaResolverErrPos { start, end },
        })
    }

    fn error_row_empty(start: usize, end: usize) -> LangErr {
        LangErr::CsvSchemaResolver(CsvSchemaResolverErr::RowEmpty {
            pos: CsvSchemaResolverErrPos { start, end },
        })
    }

    fn error_row_inval_args_len(start: usize, end: usize) -> LangErr {
        LangErr::CsvSchemaResolver(CsvSchemaResolverErr::RowInvalArgsLen {
            pos: CsvSchemaResolverErrPos { start, end },
        })
    }

    pub fn resolve(&self) -> Result<CsvResolvedSchema, LangErr> {
        let schema_root = self.schema_ast.first().ok_or_else(Self::err_root_missing)?;

        let root_call = match schema_root {
            AstNode::Call((CallKind::Named(name), root_call)) if name == SCH_FN_DEF => root_call,
            _ => {
                return Err(Self::err_root_inval(
                    schema_root.span().start,
                    schema_root.span().end,
                ));
            }
        };

        let span = &root_call.span;
        match root_call.children.len() {
            0 => return Err(Self::err_root_no_args(span.start, span.end)),
            2.. => return Err(Self::err_root_too_many_args(span.start, span.end)),
            _ => {}
        }

        let row = root_call
            .children
            .first()
            .ok_or_else(|| Self::err_root_no_args(span.start, span.end))?;

        match &**row {
            AstNode::Call((CallKind::Named(row_call_name), row_compound))
                if row_call_name == SCH_FN_ROW =>
            {
                let row_args_len = row_compound.children.len();
                if row_args_len == 0 {
                    return Err(Self::error_row_empty(
                        row_compound.span.start,
                        row_compound.span.end,
                    ));
                }

                if row_args_len % 2 != 0 {
                    return Err(Self::error_row_inval_args_len(
                        row_compound.span.start,
                        row_compound.span.end,
                    ));
                }
            }
            _ => return Err(Self::error_row_inval(row.span().start, row.span().end)),
        }

        Err(Self::err_unknown())
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
    use elise_ast::{AstNode, CallKind, Compound, Primitive, TokSpan};
    use elise_builtins::schema::{
        SCH_FN_BOOL, SCH_FN_DEF, SCH_FN_NUMBER, SCH_FN_OPTIONAL, SCH_FN_ROW, SCH_FN_STRING,
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
            CallKind::Named("invalid".to_string()),
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
            CallKind::Anon,
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
            CallKind::Named(SCH_FN_DEF.to_string()),
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
            CallKind::Named(SCH_FN_ROW.to_string()),
            Compound {
                span: TokSpan { start: 3, end: 6 },
                children: vec![],
            },
        )));
        let redundant_def = Box::new(AstNode::Call((
            CallKind::Named("row2".to_string()),
            Compound {
                span: TokSpan { start: 6, end: 9 },
                children: vec![],
            },
        )));
        let ast = vec![AstNode::Call((
            CallKind::Named(SCH_FN_DEF.to_string()),
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
            CallKind::Named(SCH_FN_DEF.to_string()),
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
            CallKind::Named("invalid".to_string()),
            Compound {
                span: TokSpan { start: 3, end: 6 },
                children: vec![],
            },
        )));
        let ast = vec![AstNode::Call((
            CallKind::Named(SCH_FN_DEF.to_string()),
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
            CallKind::Named(SCH_FN_ROW.to_string()),
            Compound {
                span: TokSpan { start: 3, end: 6 },
                children: vec![],
            },
        )));
        let ast = vec![AstNode::Call((
            CallKind::Named(SCH_FN_DEF.to_string()),
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
            CallKind::Named(SCH_FN_ROW.to_string()),
            Compound {
                span: TokSpan { start: 3, end: 6 },
                children: vec![Box::new(AstNode::Identifier(Primitive {
                    value: "some_value".to_string(),
                    span: TokSpan { start: 9, end: 12 },
                }))],
            },
        )));
        let ast = vec![AstNode::Call((
            CallKind::Named(SCH_FN_DEF.to_string()),
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

    // #[test]
    // fn should_return_error_if_odd_row_args_are_not_identifiers() {
    //     let row_children = vec![
    //         Box::new(AstNode::Number(Primitive {
    //             value: "4".to_string(),
    //             span: TokSpan { start: 9, end: 12 },
    //         })),
    //         Box::new(AstNode::Call((
    //             CallKind::Named(SCH_FN_NUMBER.to_string()),
    //             Compound {
    //                 children: vec![],
    //                 span: TokSpan { start: 12, end: 15 },
    //             },
    //         ))),
    //     ];
    //     let row_def = Box::new(AstNode::Call((
    //         CallKind::Named(SCH_FN_ROW.to_string()),
    //         Compound {
    //             span: TokSpan { start: 3, end: 6 },
    //             children: row_children,
    //         },
    //     )));
    //     let ast = vec![AstNode::Call((
    //         CallKind::Named(SCH_FN_DEF.to_string()),
    //         Compound {
    //             span: TokSpan { start: 0, end: 3 },
    //             children: vec![row_def],
    //         },
    //     ))];
    //     let result = CsvSchemaResolver::new(&ast).resolve();
    //     let err = CsvSchemaResolverErr::InvalColName {
    //         pos: CsvSchemaResolverErrPos { start: 9, end: 12 },
    //     };
    //     assert_eq!(result, Err(LangErr::CsvSchemaResolver(err)));
    // }
    //
    // #[test]
    // fn should_return_error_if_even_row_args_are_not_known_fn_calls() {
    //     let row_children = vec![
    //         Box::new(AstNode::Identifier(Primitive {
    //             value: "name".to_string(),
    //             span: TokSpan { start: 9, end: 12 },
    //         })),
    //         Box::new(AstNode::Call((
    //             CallKind::Named("some".to_string()),
    //             Compound {
    //                 children: vec![],
    //                 span: TokSpan { start: 12, end: 15 },
    //             },
    //         ))),
    //     ];
    //     let row_def = Box::new(AstNode::Call((
    //         CallKind::Named(SCH_FN_ROW.to_string()),
    //         Compound {
    //             span: TokSpan { start: 3, end: 6 },
    //             children: row_children,
    //         },
    //     )));
    //     let ast = vec![AstNode::Call((
    //         CallKind::Named(SCH_FN_DEF.to_string()),
    //         Compound {
    //             span: TokSpan { start: 0, end: 3 },
    //             children: vec![row_def],
    //         },
    //     ))];
    //     let result = CsvSchemaResolver::new(&ast).resolve();
    //     let err = CsvSchemaResolverErr::InvalColTypeDef {
    //         pos: CsvSchemaResolverErrPos { start: 12, end: 15 },
    //     };
    //     assert_eq!(result, Err(LangErr::CsvSchemaResolver(err)));
    // }
    //
    // #[test]
    // fn should_resolve_schema_req_value() {
    //     let row_children = vec![
    //         Box::new(AstNode::Identifier(Primitive {
    //             value: "name".to_string(),
    //             span: TokSpan { start: 9, end: 12 },
    //         })),
    //         Box::new(AstNode::Call((
    //             CallKind::Named(SCH_FN_NUMBER.to_string()),
    //             Compound {
    //                 children: vec![],
    //                 span: TokSpan { start: 12, end: 15 },
    //             },
    //         ))),
    //     ];
    //     let row_def = Box::new(AstNode::Call((
    //         CallKind::Named(SCH_FN_ROW.to_string()),
    //         Compound {
    //             span: TokSpan { start: 3, end: 6 },
    //             children: row_children,
    //         },
    //     )));
    //     let ast = vec![AstNode::Call((
    //         CallKind::Named(SCH_FN_DEF.to_string()),
    //         Compound {
    //             span: TokSpan { start: 0, end: 3 },
    //             children: vec![row_def],
    //         },
    //     ))];
    //     let result = CsvSchemaResolver::new(&ast).resolve();
    //     let resolved = CsvResolvedSchema {
    //         row: vec![CsvColDescriptor {
    //             name: "name".to_string(),
    //             ty: CsvColType::Number,
    //             opt: false,
    //         }],
    //     };
    //     assert_eq!(result, Ok(resolved));
    // }
    //
    // #[test]
    // fn should_resolve_schema_opt_value() {
    //     let type_opt = Box::new(AstNode::Call((
    //         CallKind::Named(SCH_FN_OPTIONAL.to_string()),
    //         Compound {
    //             children: vec![Box::new(AstNode::Call((
    //                 CallKind::Named(SCH_FN_NUMBER.to_string()),
    //                 Compound {
    //                     children: vec![],
    //                     span: TokSpan { start: 12, end: 15 },
    //                 },
    //             )))],
    //             span: TokSpan { start: 15, end: 18 },
    //         },
    //     )));
    //     let row_children = vec![
    //         Box::new(AstNode::Identifier(Primitive {
    //             value: "name".to_string(),
    //             span: TokSpan { start: 9, end: 12 },
    //         })),
    //         type_opt,
    //     ];
    //     let row_def = Box::new(AstNode::Call((
    //         CallKind::Named(SCH_FN_ROW.to_string()),
    //         Compound {
    //             span: TokSpan { start: 3, end: 6 },
    //             children: row_children,
    //         },
    //     )));
    //     let ast = vec![AstNode::Call((
    //         CallKind::Named(SCH_FN_DEF.to_string()),
    //         Compound {
    //             span: TokSpan { start: 0, end: 3 },
    //             children: vec![row_def],
    //         },
    //     ))];
    //     let result = CsvSchemaResolver::new(&ast).resolve();
    //     let resolved = CsvResolvedSchema {
    //         row: vec![CsvColDescriptor {
    //             name: "name".to_string(),
    //             ty: CsvColType::Number,
    //             opt: true,
    //         }],
    //     };
    //     assert_eq!(result, Ok(resolved));
    // }
    //
    // #[test]
    // fn should_resolve_number() {
    //     let row_children = vec![
    //         Box::new(AstNode::Identifier(Primitive {
    //             value: "age".to_string(),
    //             span: TokSpan { start: 9, end: 12 },
    //         })),
    //         Box::new(AstNode::Call((
    //             CallKind::Named(SCH_FN_NUMBER.to_string()),
    //             Compound {
    //                 children: vec![],
    //                 span: TokSpan { start: 12, end: 15 },
    //             },
    //         ))),
    //     ];
    //     let row_def = Box::new(AstNode::Call((
    //         CallKind::Named(SCH_FN_ROW.to_string()),
    //         Compound {
    //             span: TokSpan { start: 3, end: 6 },
    //             children: row_children,
    //         },
    //     )));
    //     let ast = vec![AstNode::Call((
    //         CallKind::Named(SCH_FN_DEF.to_string()),
    //         Compound {
    //             span: TokSpan { start: 0, end: 3 },
    //             children: vec![row_def],
    //         },
    //     ))];
    //     let result = CsvSchemaResolver::new(&ast).resolve();
    //     let resolved = CsvResolvedSchema {
    //         row: vec![CsvColDescriptor {
    //             name: "age".to_string(),
    //             ty: CsvColType::Number,
    //             opt: false,
    //         }],
    //     };
    //     assert_eq!(result, Ok(resolved));
    // }
    //
    // #[test]
    // fn should_resolve_string() {
    //     let row_children = vec![
    //         Box::new(AstNode::Identifier(Primitive {
    //             value: "name".to_string(),
    //             span: TokSpan { start: 9, end: 12 },
    //         })),
    //         Box::new(AstNode::Call((
    //             CallKind::Named(SCH_FN_STRING.to_string()),
    //             Compound {
    //                 children: vec![],
    //                 span: TokSpan { start: 12, end: 15 },
    //             },
    //         ))),
    //     ];
    //     let row_def = Box::new(AstNode::Call((
    //         CallKind::Named(SCH_FN_ROW.to_string()),
    //         Compound {
    //             span: TokSpan { start: 3, end: 6 },
    //             children: row_children,
    //         },
    //     )));
    //     let ast = vec![AstNode::Call((
    //         CallKind::Named(SCH_FN_DEF.to_string()),
    //         Compound {
    //             span: TokSpan { start: 0, end: 3 },
    //             children: vec![row_def],
    //         },
    //     ))];
    //     let result = CsvSchemaResolver::new(&ast).resolve();
    //     let resolved = CsvResolvedSchema {
    //         row: vec![CsvColDescriptor {
    //             name: "name".to_string(),
    //             ty: CsvColType::String,
    //             opt: false,
    //         }],
    //     };
    //     assert_eq!(result, Ok(resolved));
    // }
    //
    // #[test]
    // fn should_resolve_boolean() {
    //     let row_children = vec![
    //         Box::new(AstNode::Identifier(Primitive {
    //             value: "employed".to_string(),
    //             span: TokSpan { start: 9, end: 12 },
    //         })),
    //         Box::new(AstNode::Call((
    //             CallKind::Named(SCH_FN_BOOL.to_string()),
    //             Compound {
    //                 children: vec![],
    //                 span: TokSpan { start: 12, end: 15 },
    //             },
    //         ))),
    //     ];
    //     let row_def = Box::new(AstNode::Call((
    //         CallKind::Named(SCH_FN_ROW.to_string()),
    //         Compound {
    //             span: TokSpan { start: 3, end: 6 },
    //             children: row_children,
    //         },
    //     )));
    //     let ast = vec![AstNode::Call((
    //         CallKind::Named(SCH_FN_DEF.to_string()),
    //         Compound {
    //             span: TokSpan { start: 0, end: 3 },
    //             children: vec![row_def],
    //         },
    //     ))];
    //     let result = CsvSchemaResolver::new(&ast).resolve();
    //     let resolved = CsvResolvedSchema {
    //         row: vec![CsvColDescriptor {
    //             name: "employed".to_string(),
    //             ty: CsvColType::Bool,
    //             opt: false,
    //         }],
    //     };
    //     assert_eq!(result, Ok(resolved));
    // }
}

// ==========================
//
//  TESTS END
//
// ==========================
