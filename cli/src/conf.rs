use std::collections::HashMap;
use std::path::Path;

use crate::out;

// ==========================
//
// FILE EXT START
//
// ==========================

// Program code
const FILE_SOURCE_EXT: &str = ".eli";

// File with pre-compiled bytecode
const FILE_EXECUTABLE_EXT: &str = ".elc";

// Schema file with type definitions
// for input data
const FILE_SCHEMA_EXT: &str = ".elt";

// Supported data to be processed
const FILE_DATA_SUPPORTED_EXT: [&str; 1] = [".csv"];

// ==========================
//
// FILE EXT END
//
// ==========================

// ==========================
//
// ARGUMENT NAMES START
//
// ==========================

/**
 * Execution modes
 */

// Plain run with full data validation + execution.
const ARG_EXEC_MODE_RUN: &str = "run";

// Build an executable.
const ARG_EXEC_MODE_BUILD: &str = "build";

// Exec pre-compiled executable.
const ARG_EXEC_MODE_EXEC: &str = "exec";

// Validate data against schema.
const ARG_EXEC_MODE_VALIDATE: &str = "validate";

/**
 * Flags
 */

// Path to the file with program code
const ARG_FLAG_SOURCE_CODE: &str = "source-code";

// Path to the file with data to be processed
const ARG_FLAG_DATA: &str = "data";

// Path to the file with type definitions for data
const ARG_FLAG_DATA_SCHEMA: &str = "data-schema";

// Path to the file with pre-compiled bytecode to be executed
const ARG_FLAG_EXECUTABLE: &str = "executable";

// Path to the executable file that will be generated after the build
const ARG_FLAG_EXECUTABLE_OUTPUT: &str = "o";

// Disable data validation when program runs
const ARG_FLAG_UNSAFE_ASSUME_VALID: &str = "unsafe-assume-valid";

// Print bytecode after execution
const ARG_FLAG_PRINT_BYTECODE: &str = "print-bytecode";

// ==========================
//
// ARGUMENT NAMES END
//
// ==========================

// ==========================
//
// ARGUMENT VALUES START
//
// ==========================

const ARG_V_TRUE: &str = "true";

const ARG_V_FALSE: &str = "false";

// ==========================
//
// ARGUMENT VALUES END
//
// ==========================

// ==========================
//
// ARGUMENT TYPES START
//
// ==========================

enum ExecMode {
    RunPlain,
    Compile,
    RunCompiled,
    Validate,
}

enum ArgType {
    File(&'static str),
    Boolean,
    Mode,
}

struct Arg {
    name: &'static str,
    ty: ArgType,
    req: bool,                 // required or not
    def: Option<&'static str>, // default argument value
}

// ==========================
//
// ARGUMENT TYPES END
//
// ==========================

// ==========================
//
// ARGUMENT LIST START
//
// ==========================

const RUN_ARGS: [Arg; 2] = [
    Arg {
        name: ARG_FLAG_SOURCE_CODE,
        ty: ArgType::File(FILE_SOURCE_EXT),
        req: true,
        def: None,
    },
    Arg {
        name: ARG_FLAG_DATA,
        ty: ArgType::File(FILE_SOURCE_EXT),
        req: true,
        def: None,
    },
];

// ==========================
//
// ARGUMENT LIST END
//
// ==========================

// ==========================
//
// CONFIG START
//
// ==========================

#[derive(Debug)]
pub struct Conf {
    // We use String here because we don't want to depend on the
    // lifetime of the string source. Conf must own all arguments.
    pub file_path: String,
    pub print_bytecode: bool,
}

impl Conf {
    // Validators. Must be used for argument validation in build_cli_args function.

    fn validate_source_file<'a>(path: &'a str, ext: &'_ str) -> &'a str {
        if !Path::new(path).exists() {
            out::crash(&format!("Path does not exist: \"{}\"", path));
        }
        if !path.ends_with(ext) {
            out::crash(&format!("File must have \"{}\" extension", ext));
        }
        path
    }

    // Adapters. Must be used for adapting argument values
    // in order to construct Conf struct.

    fn boolean(value: &str) -> bool {
        value == ARG_V_TRUE
    }

    // Takes a reference to the list of original argument strings
    // and returns a hash-map of parsed data. Lifetimes tied to the
    // original owned array of arguments that was created when
    // from_cli was called, so we don't re-allocate but keeping
    // original arguments alive until we construct Conf from them.
    // This function should not perform any validation. It just
    // extract values from the input.
    // If value of the argument wasn't provided, an empty string
    // has to be inserted as value.
    fn parse_cli_args<'a>(args: &'a [String]) -> HashMap<&'a str, &'a str> {
        let mut res = HashMap::new();

        for arg in args {
            if let Some(stripped) = arg.strip_prefix("--") {
                if let Some((key, value)) = stripped.split_once('=') {
                    res.insert(key, value);
                } else {
                    res.insert(stripped, "");
                }
            }
        }

        res
    }

    // Takes a reference to the hash-map created by parse_cli_args
    // and returns a new hash-map. Both hash-maps are tied to the
    // lifetime of the original vector of strings that is owned by
    // the from_cli caller.
    // This must build the final HashMap of arguments that must
    // contain all possible arguments as key-value pairs with valid values.
    // If any argument is required but not provided, this function must panic.
    // If any argument is provided but has invalid value, this function must panic.
    // If any argument is not provided and not required it must be set to
    // the respective default value.
    fn build_cli_args<'a>(user_args: &HashMap<&'a str, &'a str>) -> HashMap<&'a str, &'a str> {
        let mut res: HashMap<&str, &str> = HashMap::new();

        for arg in ARGS {
            let mut user_arg: Option<&str> = None;

            if user_args.contains_key(arg.name) {
                let value = user_args.get(arg.name).unwrap();
                user_arg = Some(value);
            }

            if user_arg.is_none() && arg.req {
                out::crash(&format!("\"{}\" argument is required.", arg.name));
            }

            if user_arg.is_none() && arg.def.is_some() {
                res.insert(arg.name, arg.def.unwrap());
            } else if user_arg.is_some() {
                let user_arg = user_arg.unwrap();
                match arg.t {
                    AType::SourceFile(ext) => {
                        res.insert(arg.name, Self::validate_source_file(user_arg, ext));
                    }
                    AType::Boolean => {
                        if user_arg.is_empty() {
                            res.insert(arg.name, ARG_V_TRUE);
                        } else {
                            let value = if ARG_V_TRUE == user_arg {
                                ARG_V_TRUE
                            } else {
                                ARG_V_FALSE
                            };
                            res.insert(arg.name, value);
                        }
                    }
                }
            }
        }

        res
    }

    // Takes a reference to the array of strings that are raw arguments from CLI.
    // We don't own the data here, so the original owned data just reused and not
    // copied.
    pub fn from_cli(args: &[String]) -> Self {
        // Parse raw CLI arguments. This variable contains only arguments that were provided by
        // user. So if some argument is not provided, it won't be present in this data structure.
        let parsed_args = Self::parse_cli_args(args);

        // At this point we have the full list of arguments with their values.
        // If some argument was not provided by user, it must be present in this variable with
        // default value. Or if some argument was required but not provided, this function must
        // panic.
        let args = Self::build_cli_args(&parsed_args);

        Self {
            // Allocate here because Conf must own its arguments.
            file_path: args.get(ARG_K_FILE_PATH).unwrap().to_string(),
            print_bytecode: Self::boolean(args.get(ARG_K_PRINT_BYTECODE).unwrap()),
        }
    }
}

// ==========================
//
// CONFIG END
//
// ==========================

// ==========================
//
// TESTS START
//
// ==========================

#[cfg(test)]
mod tests {
    use crate::conf::Conf;

    // File path

    #[test]
    #[should_panic(expected = "\"file-path\" argument is required.")]
    fn should_require_filepath_arg_if_nothing_provided() {
        Conf::from_cli(&[]);
    }

    #[test]
    #[should_panic(expected = "Path does not exist: \"some/path/file.eli\"")]
    fn should_reject_if_path_to_file_does_not_exist() {
        Conf::from_cli(&["--file-path=some/path/file.eli".to_string()]);
    }

    #[test]
    #[should_panic(expected = "File must have \".eli\" extension")]
    fn should_reject_files_with_invalid_ext() {
        Conf::from_cli(&["--file-path=mock/test.rs".to_string()]);
    }

    #[test]
    fn should_not_panic_if_path_exists_and_file_has_correct_ext() {
        let file_path = "mock/test.eli";
        let config = Conf::from_cli(&[format!("--file-path={}", file_path)]);
        assert_eq!(config.file_path, file_path);
    }

    // Print bytecode

    #[test]
    fn should_set_print_bytecode_to_true_if_no_value_provided() {
        let config = Conf::from_cli(&[
            "--file-path=mock/test.eli".to_string(),
            "--print-bytecode".to_string(),
        ]);
        assert_eq!(config.print_bytecode, true);
    }

    #[test]
    fn should_set_print_bytecode_to_true_if_explicitly_provided() {
        let config = Conf::from_cli(&[
            "--file-path=mock/test.eli".to_string(),
            "--print-bytecode=true".to_string(),
        ]);
        assert_eq!(config.print_bytecode, true);
    }

    #[test]
    fn should_set_print_bytecode_to_false_if_explicitly_provided() {
        let config = Conf::from_cli(&[
            "--file-path=mock/test.eli".to_string(),
            "--print-bytecode=false".to_string(),
        ]);
        assert_eq!(config.print_bytecode, false);
    }

    #[test]
    fn should_set_print_bytecode_to_false_if_invalid_value_provided() {
        let config = Conf::from_cli(&[
            "--file-path=mock/test.eli".to_string(),
            "--print-bytecode=123".to_string(),
        ]);
        assert_eq!(config.print_bytecode, false);
    }
}

// ==========================
//
// TESTS END
//
// ==========================
