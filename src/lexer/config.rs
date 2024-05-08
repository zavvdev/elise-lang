// Allowed set of characters: a-z A-Z - ? ! _ 0-9
// Not allowed at start: 0-9 - ? ! @
// Not allowed: whitespace and all others
pub const IDENTIFIER_REGEX: &str = r"^([^\d\-?!@\s+])([a-zA-Z\-\?!_\d])+$";
