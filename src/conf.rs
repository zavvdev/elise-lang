use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;

use crate::out;

// ===============================
// Constants
// ===============================

const FILE_EXT: &str = ".eli";

const ARG_K_FILE_PATH: &str = "file-path";

const ARG_K_PRINT_BYTECODE: &str = "print-bytecode";

const ARG_V_TRUE: &str = "true";

const ARG_V_FALSE: &str = "false";

// ==============================

#[derive(Clone)]
enum AType {
    SourceFile(&'static str),
    Boolean,
}

#[derive(Clone)] // TODO: Remove clone
struct Arg {
    name: &'static str,
    t: AType,
    req: bool,
    def: Option<String>,
}

// We use lazy evaluation for static variables in order to keep
// all arguments in a since data structure, so we can easily use
// keys and values.
lazy_static! {
    // List of all possible arguments that we want to parse from CLI.

    static ref ARGS_MAP: HashMap<&'static str, Arg> = {
        let mut m = HashMap::new();

        m.insert(
            ARG_K_FILE_PATH,
            Arg {
                name: ARG_K_FILE_PATH,
                t: AType::SourceFile(FILE_EXT),
                req: true,
                def: None,
            },
        );

        m.insert(
            ARG_K_PRINT_BYTECODE,
            Arg {
                name: ARG_K_PRINT_BYTECODE,
                t: AType::Boolean,
                req: false,
                def: Some(ARG_V_FALSE.to_string()),
            },
        );

        m
    };
}

// ===============================

#[derive(Debug, Clone)]
pub struct Conf {
    pub file_path: String,
    pub print_bytecode: bool,
}

impl Conf {
    // Validators. Must be used for argument validation in build_cli_args function.

    fn validate_source_file(path: String, ext: &str) -> String {
        if path.ends_with(ext) {
            return path;
        } else {
            out::crash(&format!("File must have {} extension", ext));
        }
    }

    // Adapters. Must be used for adapting argument values
    // in order to construct Conf struct.

    fn boolean(value: &str) -> bool {
        value == ARG_V_TRUE
    }

    // Take the list of raw CLI arguments provided by user (args)
    // and the list of argument names that we want to parse (names).
    // Returns HashMap where keys are values from names argument
    // and values are Option<String>.
    // This function should not perform any validation. It just
    // extract values from the input.
    pub fn parse_cli_args(args: &Vec<String>, names: &Vec<&str>) -> HashMap<String, String> {
        let mut res: HashMap<String, String> = HashMap::new();

        let args = format!("{} ", args.join(" "));

        for name in names {
            let pattern = format!(r"--{}(=?)(.*?)(\s+)", name);
            let re = Regex::new(&pattern).unwrap();
            if let Some(caps) = re.captures(&args) {
                let m = &caps[2];
                res.insert(name.to_string(), m.to_string());
            }
        }

        res
    }

    // This should take the result of parse_cli_args and build
    // the final HashMap of arguments that must contain all possible
    // arguments as key-value pairs with valid values.
    // If any argument is required but not provided, this function must panic.
    // If any argument is provided but has invalid value, this function must panic.
    // If any argument is not provided and not required it must be set to
    // the respective default value.
    pub fn build_cli_args(args: &HashMap<String, String>) -> HashMap<String, String> {
        let mut res: HashMap<String, String> = HashMap::new();

        // TODO: remove clone
        for (key, value) in ARGS_MAP.clone().into_iter() {
            let mut arg: Option<String> = None;

            if args.contains_key(key) {
                let arg_value = args.get(key).unwrap();
                arg = Some(arg_value.to_string());
            }

            let key = key.to_string();

            if arg.is_none() && value.req {
                out::crash(&format!("\"{}\" argument is required.", value.name));
            }

            if arg.is_none() && value.def.is_some() {
                res.insert(key, value.def.unwrap());
            } else {
                let arg = arg.unwrap();
                match value.t {
                    AType::SourceFile(ext) => {
                        res.insert(key, Self::validate_source_file(arg, ext));
                    }
                    AType::Boolean => {
                        if arg.is_empty() {
                            res.insert(key, ARG_V_TRUE.to_string());
                        } else {
                            res.insert(key, (ARG_V_TRUE == arg).to_string());
                        }
                    }
                }
            }
        }

        return res;
    }

    // Takes the vector of strings that are raw arguments from CLI.
    // Should return a valid Conf struct or panic if any arguments are invalid.
    pub fn from_cli(args: Vec<String>) -> Self {
        // Parse raw CLI arguments. This variable contains only arguments that were provided by
        // user. So if some argument is not provided, it won't be present in this variable.
        let parsed_args = Self::parse_cli_args(&args, &ARGS_MAP.keys().cloned().collect());

        // At this point we have the full list of arguments with their values.
        // If some argument was not provided by user, it must be present in this variable with
        // default value. Or if some argument was required but not provided, this function must
        // panic.
        let args = Self::build_cli_args(&parsed_args);

        Self {
            file_path: args.get(ARG_K_FILE_PATH).unwrap().to_string(),
            print_bytecode: Self::boolean(args.get(ARG_K_PRINT_BYTECODE).unwrap()),
        }
    }
}

/**
 * ===============================
 * Tests
 * ===============================
 */

#[cfg(test)]
mod tests {
    #[test]
    fn should_require_filepath_arg_if_nothing_provided() {
        panic!("TODO");
    }

    #[test]
    fn should_reject_files_with_invalid_ext() {
        panic!("TODO");
    }

    #[test]
    fn should_not_accept_invalid_path() {}

    #[test]
    fn should_set_print_bytecode_to_true_if_no_value_provided() {
        panic!("TODO");
    }

    #[test]
    fn should_set_print_bytecode_to_true_if_explicitly_provided() {
        panic!("TODO");
    }

    #[test]
    fn should_set_print_bytecode_to_false_if_explicitly_provided() {
        panic!("TODO");
    }
}
