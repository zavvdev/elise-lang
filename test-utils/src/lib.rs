pub mod test_utils {
    use elise_ast::AstNode;
    use elise_data::schema_binder::{SchemaBinder, SchemaBindings};
    use elise_parser::Prelude;

    // =====================================================================================
    //
    // PARSER UTILS START
    //
    // =====================================================================================

    pub fn parse(source_code: &str) -> Vec<AstNode> {
        Prelude::new(source_code.as_bytes()).parse().unwrap()
    }

    // =====================================================================================
    //
    // PARSER UTILS END
    //
    // =====================================================================================

    // =====================================================================================
    //
    // SCHEMA BINDER START
    //
    // =====================================================================================

    pub fn bind_schema(source_code: &str) -> SchemaBindings {
        let parsed = parse(source_code);
        SchemaBinder::new(&parsed).bind().unwrap()
    }

    // =====================================================================================
    //
    // SCHEMA BINDER END
    //
    // =====================================================================================

    // =====================================================================================
    //
    // DATA UTILS START
    //
    // =====================================================================================

    pub mod csv {
        use elise_data::{
            csv::{csv_data_binder::CsvDataBinder, csv_data_parser::CsvDataParser},
            data_binder::DataBinder,
            data_binder::DataBindings,
        };

        pub fn build_header(index: usize) -> String {
            format!("n{}", index)
        }

        pub fn build(row: &Vec<&str>) -> String {
            let head: Vec<String> = (0..row.len()).map(build_header).collect();
            format!("{}\n{}", head.join(","), row.join(","))
        }

        pub fn bind(row: &Vec<&str>) -> DataBindings {
            let parsed = CsvDataParser::new(&build(row)).parse().unwrap();
            CsvDataBinder::new(&parsed).bind().unwrap()
        }
    }

    // =====================================================================================
    //
    // DATA UTILS END
    //
    // =====================================================================================
}
