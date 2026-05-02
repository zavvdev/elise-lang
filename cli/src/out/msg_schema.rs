use elise_shared::errors::errors_schema::SchemaErr;

use crate::out::utils::print_silent_err;

pub fn print_schema_err(schema_err: &SchemaErr) {
    use SchemaErr::*;

    let info: String = match schema_err {
        InvalDefFunc => {
            format!(
                "Invalid schema definition function.\nYou must define your schema inside 'schema'
            function"
            )
        }
    };

    print_silent_err(&info, Some("Schema error"));
}
