pub mod test_utils {
    use elise_aast::AAstNode;
    use elise_ast::AstNode;
    use elise_bindings::TypeBindingsMap;
    use elise_parser::Prelude;
    use elise_semanalyzer::Harmony;

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
        Harmony::new(&ast, global_type_bindings).analyze().unwrap()
    }

    // ==================================================================
    //
    // SEMANALYZER END
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
