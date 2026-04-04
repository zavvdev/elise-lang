/**
 * A collection of all built-in functions names.
 */

// Identifiers
pub const FN_IDENTIFIER_DEFINE_GLOBAL: &str = "def";
pub const FN_IDENTIFIER_DEFINE_SCOPED: &str = "let";
pub const FN_IDENTIFIER_DEFINE_FUNCTION: &str = "fn";

// Math
pub const FN_NUMBER_ADD: &str = "add";
pub const FN_NUMBER_SUB: &str = "sub";
pub const FN_NUMBER_MUL: &str = "mul";
pub const FN_NUMBER_DIV: &str = "div";

// Branching
pub const FN_BRANCH_IF: &str = "if";

// Predicates
pub const FN_PRED_IS_NULL: &str = "null?";
pub const FN_PRED_NOT: &str = "not";
pub const FN_PRED_EQ: &str = "eq";
pub const FN_PRED_NOT_EQ: &str = "not-eq";
pub const FN_PRED_GREATER: &str = "greatr";
pub const FN_PRED_LESS: &str = "less";
pub const FN_PRED_GREATER_EQ: &str = "greatr-eq";
pub const FN_PRED_LESS_EQ: &str = "less-eq";

// Data manipulation
pub const FN_DATA_PIPE: &str = "pipe";
pub const FN_DATA_MAP: &str = "map";
pub const FN_DATA_REDUCE: &str = "reduce";
pub const FN_DATA_FILTER: &str = "filter";
pub const FN_DATA_SORT: &str = "sort";
pub const FN_DATA_IDENTITY: &str = "identity";

// Concurrency
pub const FN_CONC_PARALLELIZE: &str = "par";

// Files
pub const FN_FILE_READ_FULL: &str = "read";

// Strings
pub const FN_STRING_CONCAT: &str = "conc";
pub const FN_STRING_CAST: &str = "str";

// Numbers
pub const FN_NUMBER_CAST: &str = "num";

// Compounds
pub const FN_COMP_GET_VALUE: &str = "get";

// ...
