pub mod test_utils {
    use std::collections::HashMap;

    use elise_frontend_parser::Prelude;
    use elise_frontend_semanalyzer::Harmony;
    use elise_ir_aast::AAstNode;
    use elise_ir_ast::AstNode;
    use elise_ir_bindings::{
        binding_path::{BindingPath, BindingPathSegment},
        type_bindings::{AliasedTypeBindings, TypeBinding, TypeBindingDataType, TypeBindingDesc},
    };
    use elise_shared_errors::errors_semanalyzer::SemanalyzerErr;
    use elise_shared_types::Span;

    // ==================================================================
    //
    // PARSER UTILS START
    //
    // ==================================================================

    pub fn parse(source_code: &str) -> Vec<AstNode> {
        Prelude::new(source_code.as_bytes()).parse().unwrap()
    }

    // ==================================================================
    //
    // PARSER UTILS END
    //
    // ==================================================================

    // ==================================================================
    //
    // SEMANALYZER START
    //
    // ==================================================================

    pub fn semanalyze(
        source_code: &str,
        global_type_bindings: &mut AliasedTypeBindings,
        typedef_mode: bool,
    ) -> Result<Vec<AAstNode>, SemanalyzerErr> {
        let ast = Prelude::new(source_code.as_bytes()).parse().unwrap();
        Harmony::new(&ast, global_type_bindings, typedef_mode).annotate()
    }

    // ==================================================================
    //
    // SEMANALYZER END
    //
    // ==================================================================

    // ==================================================================
    //
    // TYPE BINDER START
    //
    // ==================================================================

    /// Creates a test bindings map with three values:
    /// SingleType, RecordType and ListType. We can reference
    /// these aliases in tests and pass the result into
    /// Harmony or TypeBinder globals.
    pub fn type_bindings_map() -> AliasedTypeBindings {
        let mut map: AliasedTypeBindings = HashMap::new();

        let mut single_type_bindings = HashMap::new();
        single_type_bindings.insert(
            BindingPath::new(),
            TypeBindingDesc {
                dtype: TypeBindingDataType::Int,
                span: Span { start: 0, end: 0 },
            },
        );

        let mut record_type_bindings = HashMap::new();
        record_type_bindings.insert(
            BindingPath::new(),
            TypeBindingDesc {
                dtype: TypeBindingDataType::Record,
                span: Span { start: 0, end: 0 },
            },
        );
        record_type_bindings.insert(
            BindingPath::with_segments(vec![BindingPathSegment::Field("id".to_string())]),
            TypeBindingDesc {
                dtype: TypeBindingDataType::Int,
                span: Span { start: 0, end: 0 },
            },
        );

        let mut list_type_bindings = HashMap::new();
        list_type_bindings.insert(
            BindingPath::new(),
            TypeBindingDesc {
                dtype: TypeBindingDataType::List,
                span: Span { start: 0, end: 0 },
            },
        );
        list_type_bindings.insert(
            BindingPath::with_segments(vec![BindingPathSegment::AbstractIndex]),
            TypeBindingDesc {
                dtype: TypeBindingDataType::Int,
                span: Span { start: 0, end: 0 },
            },
        );

        let mut data_slot_bindings = HashMap::new();
        data_slot_bindings.insert(
            BindingPath::new(),
            TypeBindingDesc {
                dtype: TypeBindingDataType::List,
                span: Span { start: 0, end: 0 },
            },
        );
        data_slot_bindings.insert(
            BindingPath::with_segments(vec![BindingPathSegment::AbstractIndex]),
            TypeBindingDesc {
                dtype: TypeBindingDataType::Record,
                span: Span { start: 0, end: 0 },
            },
        );
        data_slot_bindings.insert(
            BindingPath::with_segments(vec![
                BindingPathSegment::AbstractIndex,
                BindingPathSegment::Field("id".to_string()),
            ]),
            TypeBindingDesc {
                dtype: TypeBindingDataType::Int,
                span: Span { start: 0, end: 0 },
            },
        );
        data_slot_bindings.insert(
            BindingPath::with_segments(vec![
                BindingPathSegment::AbstractIndex,
                BindingPathSegment::Field("name".to_string()),
            ]),
            TypeBindingDesc {
                dtype: TypeBindingDataType::Str,
                span: Span { start: 0, end: 0 },
            },
        );

        map.insert(
            "SingleType".to_string(),
            TypeBinding::with(&single_type_bindings),
        );
        map.insert(
            "RecordType".to_string(),
            TypeBinding::with(&record_type_bindings),
        );
        map.insert(
            "ListType".to_string(),
            TypeBinding::with(&list_type_bindings),
        );
        map.insert("Data".to_string(), TypeBinding::with(&data_slot_bindings));

        map
    }

    // ==================================================================
    //
    // TYPE BINDER END
    //
    // ==================================================================

    // ==================================================================
    //
    // DATA UTILS START
    //
    // ==================================================================

    pub mod csv {
        pub fn build_header(index: usize) -> String {
            format!("n{}", index)
        }

        pub fn build(rows: &Vec<&Vec<&str>>) -> String {
            let head: Vec<String> = (0..rows[0].len()).map(build_header).collect();
            let mut final_str = head.join(",");
            for row in rows {
                final_str = format!("{}\n{}", final_str, row.join(","));
            }
            final_str
        }
    }

    // ==================================================================
    //
    // DATA UTILS END
    //
    // ==================================================================

    // ==================================================================
    //
    // MISC START
    //
    // ==================================================================

    pub fn span() -> Span {
        Span { start: 0, end: 0 }
    }

    // ==================================================================
    //
    // MISC END
    //
    // ==================================================================
}
