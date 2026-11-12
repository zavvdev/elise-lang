#[derive(Debug, PartialEq)]
pub enum BindingsValidatorErr {
    // TODO: Add metadata.
    TypeMismatch,
    // TODO: Add metadata.
    MissingTypeDeclaration,
}
