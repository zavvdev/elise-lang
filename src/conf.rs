use regex::Regex;
use std::collections::HashMap;
use std::path::Path;

use crate::out;

// ===============================
// #source_file
// ===============================

const FILE_EXT: &str = ".eli";

// ===============================
// #argument_names
// ===============================

const ARG_K_FILE_PATH: &str = "file-path";

const ARG_K_PRINT_BYTECODE: &str = "print-bytecode";

// ===============================
// #argument_values
// ===============================

const ARG_V_TRUE: &str = "true";

const ARG_V_FALSE: &str = "false";

// ===============================
// #argument_types
// ===============================

enum AType {
    SourceFile(&'static str),
    Boolean,
}

struct Arg {
    name: &'static str,
    t: AType,
    req: bool,                 // required or not
    def: Option<&'static str>, // default argument value
}

// ===============================
// #available_arguments
// ===============================

const ARGS: [Arg; 2] = [
    Arg {
        name: ARG_K_FILE_PATH,
        t: AType::SourceFile(FILE_EXT),
        req: true,
        def: None,
    },
    Arg {
        name: ARG_K_PRINT_BYTECODE,
        t: AType::Boolean,
        req: false,
        def: Some(ARG_V_FALSE),
    },
];

// ===============================
// #config_struct
// ===============================

#[derive(Debug)]
pub struct Conf {
    pub file_path: String,
    pub print_bytecode: bool,
}

impl Conf {
    // Validators. Must be used for argument validation in build_cli_args function.

    fn validate_source_file(path: String, ext: &str) -> String {
        if !Path::new(&path).exists() {
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

    // Take the list of raw CLI arguments provided by user (args)
    // and the list of argument names that we want to parse (names).
    // Returns HashMap where keys are values from names argument
    // and values are Option<String>.
    // This function should not perform any validation. It just
    // extract values from the input.
    fn parse_cli_args(args: &Vec<String>, names: &[&str]) -> HashMap<String, String> {
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
    fn build_cli_args(user_args: &HashMap<String, String>) -> HashMap<String, String> {
        let mut res: HashMap<String, String> = HashMap::new();

        for arg in ARGS {
            let mut user_arg: Option<String> = None;

            if user_args.contains_key(arg.name) {
                let value = user_args.get(arg.name).unwrap();
                user_arg = Some(value.to_string());
            }

            if user_arg.is_none() && arg.req {
                out::crash(&format!("\"{}\" argument is required.", arg.name));
            }

            if user_arg.is_none() && arg.def.is_some() {
                res.insert(arg.name.to_string(), arg.def.unwrap().to_string());
            } else if user_arg.is_some() {
                let user_arg = user_arg.unwrap();
                match arg.t {
                    AType::SourceFile(ext) => {
                        res.insert(
                            arg.name.to_string(),
                            Self::validate_source_file(user_arg, ext),
                        );
                    }
                    AType::Boolean => {
                        if user_arg.is_empty() {
                            res.insert(arg.name.to_string(), ARG_V_TRUE.to_string());
                        } else {
                            res.insert(arg.name.to_string(), (ARG_V_TRUE == user_arg).to_string());
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
        let parsed_args = Self::parse_cli_args(&args, &ARGS.map(|arg| arg.name));

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

// ===============================
// #tests
// ===============================

#[cfg(test)]
mod tests {
    use crate::conf::Conf;

    // ===============================
    // #file_path
    // ===============================

    #[test]
    #[should_panic(expected = "\"file-path\" argument is required.")]
    fn should_require_filepath_arg_if_nothing_provided() {
        Conf::from_cli(vec![]);
    }

    #[test]
    #[should_panic(expected = "Path does not exist: \"some/path/file.eli\"")]
    fn should_reject_if_path_to_file_does_not_exist() {
        Conf::from_cli(vec!["--file-path=some/path/file.eli".to_string()]);
    }

    #[test]
    #[should_panic(expected = "File must have \".eli\" extension")]
    fn should_reject_files_with_invalid_ext() {
        Conf::from_cli(vec!["--file-path=test.rs".to_string()]);
    }

    #[test]
    fn should_not_panic_if_path_exists_and_file_has_correct_ext() {
        let file_path = "test.eli";
        let config = Conf::from_cli(vec![format!("--file-path={}", file_path)]);
        assert_eq!(config.file_path, file_path);
    }

    // ===============================
    // #print_bytecode
    // ===============================

    #[test]
    fn should_set_print_bytecode_to_true_if_no_value_provided() {
        let config = Conf::from_cli(vec![
            "--file-path=test.eli".to_string(),
            "--print-bytecode".to_string(),
        ]);
        assert_eq!(config.print_bytecode, true);
    }

    #[test]
    fn should_set_print_bytecode_to_true_if_explicitly_provided() {
        let config = Conf::from_cli(vec![
            "--file-path=test.eli".to_string(),
            "--print-bytecode=true".to_string(),
        ]);
        assert_eq!(config.print_bytecode, true);
    }

    #[test]
    fn should_set_print_bytecode_to_false_if_explicitly_provided() {
        let config = Conf::from_cli(vec![
            "--file-path=test.eli".to_string(),
            "--print-bytecode=false".to_string(),
        ]);
        assert_eq!(config.print_bytecode, false);
    }

    #[test]
    fn should_set_print_bytecode_to_false_if_invalid_value_provided() {
        let config = Conf::from_cli(vec![
            "--file-path=test.eli".to_string(),
            "--print-bytecode=123".to_string(),
        ]);
        assert_eq!(config.print_bytecode, false);
    }
}
