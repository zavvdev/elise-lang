pub mod test_utils {
    use std::collections::HashMap;

    use elise_aast::AAstNode;
    use elise_ast::AstNode;
    use elise_bindings::{
        BindingType, TypeBindingDesc, TypeBindingsMap,
        binding_path::{BindingPath, BindingPathSegment},
    };
    use elise_parser::Prelude;
    use elise_semanalyzer::Harmony;
    use elise_shared::shared_types::Span;

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
        global_type_bindings: &mut TypeBindingsMap,
    ) -> Vec<AAstNode> {
        let ast = Prelude::new(source_code.as_bytes()).parse().unwrap();
        Harmony::new(&ast, global_type_bindings, false)
            .analyze()
            .unwrap()
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
    pub fn type_bindings_map() -> TypeBindingsMap {
        let mut map: TypeBindingsMap = HashMap::new();

        let mut single_type_bindings = HashMap::new();
        single_type_bindings.insert(
            BindingPath::new(),
            TypeBindingDesc {
                dtype: BindingType::Int,
                span: Span { start: 0, end: 0 },
            },
        );

        let mut record_type_bindings = HashMap::new();
        record_type_bindings.insert(
            BindingPath::new(),
            TypeBindingDesc {
                dtype: BindingType::Record,
                span: Span { start: 0, end: 0 },
            },
        );
        record_type_bindings.insert(
            BindingPath::with_segments(vec![BindingPathSegment::Field("id".to_string())]),
            TypeBindingDesc {
                dtype: BindingType::Int,
                span: Span { start: 0, end: 0 },
            },
        );

        let mut list_type_bindings = HashMap::new();
        list_type_bindings.insert(
            BindingPath::new(),
            TypeBindingDesc {
                dtype: BindingType::List,
                span: Span { start: 0, end: 0 },
            },
        );
        list_type_bindings.insert(
            BindingPath::with_segments(vec![BindingPathSegment::AbstractIndex]),
            TypeBindingDesc {
                dtype: BindingType::Int,
                span: Span { start: 0, end: 0 },
            },
        );

        map.insert("SingleType".to_string(), single_type_bindings);
        map.insert("RecordType".to_string(), record_type_bindings);
        map.insert("ListType".to_string(), list_type_bindings);

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

    //pub mod csv {
    //    use elise_data::{
    //        csv::{csv_data_binder::CsvDataBinder, csv_data_parser::CsvDataParser},
    //        data_binder::{DataBinder, DataBindings},
    //    };

    //    pub fn build_header(index: usize) -> String {
    //        format!("n{}", index)
    //    }

    //    pub fn build(rows: &Vec<&Vec<&str>>) -> String {
    //        let head: Vec<String> = (0..rows[0].len()).map(build_header).collect();
    //        let mut final_str = head.join(",");
    //        for row in rows {
    //            final_str = format!("{}\n{}", final_str, row.join(","));
    //        }
    //        final_str
    //    }

    //    pub fn bind(rows: &Vec<&Vec<&str>>) -> DataBindings {
    //        let parsed = CsvDataParser::new(&build(rows)).parse().unwrap();
    //        CsvDataBinder::new(&parsed).bind().unwrap()
    //    }
    //}

    // ==================================================================
    //
    // DATA UTILS END
    //
    // ==================================================================
}
