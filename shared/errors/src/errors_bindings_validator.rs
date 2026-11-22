#[derive(Debug, PartialEq)]
pub enum BindingsValidatorErr {
    TypeMismatch {
        data_path_info: String,
        expected: String,
        found: String,
    },

    MissingTypeDeclaration {
        data_path_info: String,
        data_type_info: String,
    },
}
