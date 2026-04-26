/**
 * This file represents configuration type
 * that must include all necessary information
 * for the program in order to start execution.
 */
pub mod config;

use std::collections::HashMap;

use config::{
    ARG_FLAG_DATA, ARG_FLAG_DATA_SCHEMA, ARG_FLAG_EXECUTABLE, ARG_FLAG_MODE, ARG_FLAG_OUTPUT,
    ARG_FLAG_PRINT_BYTECODE, ARG_FLAG_SOURCE_CODE, ARG_V_BOOL_FALSE,
    ARG_V_BOOL_TRUE, ARG_V_MODE_BUILD, ARG_V_MODE_EXEC, ARG_V_MODE_RUN, ARG_V_MODE_VALIDATE,
    ARG_V_MODES, ArgType, BUILD_ARGS, EXEC_ARGS, RUN_ARGS, VALIDATE_ARGS,
};

#[derive(Debug, PartialEq)]
pub struct InvalidArg {
    pub arg_name: String,
    pub provided: String,
}

#[derive(Debug, PartialEq)]
pub enum ConfErr {
    ExtInvalid(String),
    ArgInvalid(InvalidArg),
    ArgRequired(String),
}

#[derive(Debug, PartialEq)]
pub struct ModeRunConf {
    pub source_code_path: String,
    pub data_path: String,
    pub data_schema_path: String,
    pub print_bytecode: bool,
    pub output_path: Option<String>,
}

#[derive(Debug, PartialEq)]
pub struct ModeBuildConf {
    pub source_code_path: String,
    pub data_schema_path: String,
    pub executable_output_path: String,
}

#[derive(Debug, PartialEq)]
pub struct ModeExecConf {
    pub executable_path: String,
    pub data_path: String,
}

#[derive(Debug, PartialEq)]
pub struct ModeValidateConf {
    pub data_path: String,
    pub data_schema_path: String,
}

#[derive(Debug, PartialEq)]
pub enum Conf {
    Run(ModeRunConf),
    Build(ModeBuildConf),
    Exec(ModeExecConf),
    Validate(ModeValidateConf),
}

impl Conf {
    // Validators. Must be used for argument validation in build_args function.

    fn validate_source_file<'a>(path: &'a str, exts: &[&'_ str]) -> Result<&'a str, ConfErr> {
        if !exts.iter().any(|e| path.ends_with(*e)) {
            return Err(ConfErr::ExtInvalid(path.to_string()));
        }
        Ok(path)
    }

    fn validate_mode<'a>(mode: Option<&'a str>) -> Result<&'a str, ConfErr> {
        match mode {
            Some(mode) if ARG_V_MODES.contains(&mode) => Ok(mode),
            Some(mode) => Err(ConfErr::ArgInvalid(InvalidArg {
                provided: mode.to_string(),
                arg_name: ARG_FLAG_MODE.to_string(),
            })),
            None => Err(ConfErr::ArgRequired(ARG_FLAG_MODE.to_string())),
        }
    }

    // Adapters. Must be used for adapting argument values
    // in order to construct Conf structs.

    fn arg_bool(value: Option<&&str>) -> bool {
        value.is_some() && *value.unwrap() == ARG_V_BOOL_TRUE
    }

    fn arg_str<'a>(value: Option<&&str>) -> String {
        if value.is_some() {
            value.unwrap().to_string()
        } else {
            "".to_string()
        }
    }

    fn arg_any(value: Option<&&str>) -> Option<String> {
        if value.is_some() {
            let val = *value.unwrap();
            Some(val.to_string())
        } else {
            None
        }
    }

    // Takes a reference to the list of original argument strings
    // and returns a hash-map of parsed data. Lifetimes tied to the
    // original owned array of arguments that was created when
    // Conf::new was called, so we don't re-allocate but keeping
    // original arguments alive until we construct Conf struct from them.
    // This function should not perform any validation. It just
    // extract values from the input.
    // If value of some argument wasn't provided, an empty string
    // has to be inserted as a value.
    fn parse_args<'a>(args: &'a [String]) -> HashMap<&'a str, &'a str> {
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

    // Takes a reference to the hash-map created by parse_args
    // and returns a new hash-map. Both hash-maps are tied to the
    // lifetime of the original vector of strings that is owned by
    // the Conf::new caller.
    // This must build the final HashMap of arguments that must
    // contain all possible arguments as key-value pairs with valid values.
    //
    // If any argument is required but not provided, this function must return an error.
    // If any argument is provided but has invalid value, this function must return an error.
    // If any argument is not provided and not required it must be set to
    // the respective default value if available.
    fn build_valid_args<'a>(
        user_args: &HashMap<&'a str, &'a str>,
        mode: &str,
    ) -> Result<HashMap<&'a str, &'a str>, ConfErr> {
        let mut res: HashMap<&str, &str> = HashMap::new();

        let args = match mode {
            ARG_V_MODE_RUN => Ok(RUN_ARGS),
            ARG_V_MODE_BUILD => Ok(BUILD_ARGS),
            ARG_V_MODE_EXEC => Ok(EXEC_ARGS),
            ARG_V_MODE_VALIDATE => Ok(VALIDATE_ARGS),
            _ => Err(ConfErr::ArgInvalid(InvalidArg {
                provided: mode.to_string(),
                arg_name: ARG_FLAG_MODE.to_string(),
            })),
        }?;

        for arg in args {
            let mut user_arg: Option<&str> = None;

            if user_args.contains_key(arg.name) {
                let value = user_args.get(arg.name).unwrap();
                user_arg = Some(value);
            }

            if user_arg.is_none() && arg.req {
                return Err(ConfErr::ArgRequired(arg.name.to_string()));
            }

            if user_arg.is_none() && arg.def.is_some() {
                res.insert(arg.name, arg.def.unwrap());
            } else if user_arg.is_some() {
                let user_arg = user_arg.unwrap();
                match arg.ty {
                    ArgType::SourceFile(ext) => {
                        let file = Self::validate_source_file(user_arg, ext)?;
                        res.insert(arg.name, file);
                    }
                    ArgType::Boolean => {
                        if user_arg.is_empty() {
                            res.insert(arg.name, ARG_V_BOOL_TRUE);
                        } else {
                            let value = if ARG_V_BOOL_TRUE == user_arg {
                                ARG_V_BOOL_TRUE
                            } else {
                                ARG_V_BOOL_FALSE
                            };
                            res.insert(arg.name, value);
                        }
                    }
                    ArgType::Any => {
                        res.insert(arg.name, user_arg);
                    }
                }
            }
        }

        Ok(res)
    }

    // Takes a reference to the array of strings that are raw arguments.
    // We don't own the data here, so the original owned data just reused and not
    // copied.
    pub fn new(args: &[String]) -> Result<Self, ConfErr> {
        // Parse raw arguments. This variable contains only arguments that were provided by
        // user. So if some argument is not provided, it won't be present in this data structure.
        let parsed_args = Self::parse_args(args);

        // Must be an error if mode is invalid.
        let mode = Self::validate_mode(parsed_args.get(ARG_FLAG_MODE).map(|mode| *mode))?;

        // At this point we have the full list of arguments with their values.
        // If some argument was not provided by user, it must be present in this variable with
        // a default value. Or if some argument was required but not provided, this function must
        // return an error.
        let args = Self::build_valid_args(&parsed_args, mode)?;

        match mode {
            ARG_V_MODE_RUN => Ok(Self::Run(ModeRunConf {
                source_code_path: Self::arg_str(args.get(ARG_FLAG_SOURCE_CODE)),
                data_path: Self::arg_str(args.get(ARG_FLAG_DATA)),
                data_schema_path: Self::arg_str(args.get(ARG_FLAG_DATA_SCHEMA)),
                print_bytecode: Self::arg_bool(args.get(ARG_FLAG_PRINT_BYTECODE)),
                output_path: Self::arg_any(args.get(ARG_FLAG_OUTPUT)),
            })),

            ARG_V_MODE_BUILD => Ok(Self::Build(ModeBuildConf {
                source_code_path: Self::arg_str(args.get(ARG_FLAG_SOURCE_CODE)),
                data_schema_path: Self::arg_str(args.get(ARG_FLAG_DATA_SCHEMA)),
                executable_output_path: Self::arg_str(args.get(ARG_FLAG_OUTPUT)),
            })),

            ARG_V_MODE_EXEC => Ok(Self::Exec(ModeExecConf {
                executable_path: Self::arg_str(args.get(ARG_FLAG_EXECUTABLE)),
                data_path: Self::arg_str(args.get(ARG_FLAG_DATA)),
            })),

            ARG_V_MODE_VALIDATE => Ok(Self::Validate(ModeValidateConf {
                data_path: Self::arg_str(args.get(ARG_FLAG_DATA)),
                data_schema_path: Self::arg_str(args.get(ARG_FLAG_DATA_SCHEMA)),
            })),

            _ => Err(ConfErr::ArgInvalid(InvalidArg {
                provided: mode.to_string(),
                arg_name: ARG_FLAG_MODE.to_string(),
            })),
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
    use crate::conf::config::{
        ARG_FLAG_DATA, ARG_FLAG_DATA_SCHEMA, ARG_FLAG_EXECUTABLE, ARG_FLAG_MODE, ARG_FLAG_OUTPUT,
        ARG_FLAG_SOURCE_CODE,
    };
    use crate::conf::{
        Conf, ConfErr, InvalidArg, ModeBuildConf, ModeExecConf, ModeRunConf, ModeValidateConf,
    };

    #[test]
    fn should_require_mode_flag() {
        let result = Conf::new(&[
            "--source-code=sample.eli".to_string(),
            "--data=data.csv".to_string(),
            "--data-schema=data.elt".to_string(),
        ]);
        assert_eq!(result, Err(ConfErr::ArgRequired(ARG_FLAG_MODE.to_string())));
    }

    #[test]
    fn should_reject_invalid_mode_flag() {
        let result = Conf::new(&[
            "--mode=invalid".to_string(),
            "--source-code=sample.eli".to_string(),
            "--data=data.csv".to_string(),
            "--data-schema=data.elt".to_string(),
        ]);
        assert_eq!(
            result,
            Err(ConfErr::ArgInvalid(InvalidArg {
                provided: "invalid".to_string(),
                arg_name: ARG_FLAG_MODE.to_string(),
            }))
        );
    }

    // RUN MODE TESTS START

    #[test]
    fn run_should_require_source_code_flag() {
        let result = Conf::new(&[
            "--mode=run".to_string(),
            "--data=data.csv".to_string(),
            "--data-schema=data.elt".to_string(),
        ]);
        assert_eq!(
            result,
            Err(ConfErr::ArgRequired(ARG_FLAG_SOURCE_CODE.to_string()))
        );
    }

    #[test]
    fn run_should_require_data_flag() {
        let result = Conf::new(&[
            "--mode=run".to_string(),
            "--source-code=sample.eli".to_string(),
            "--data-schema=data.elt".to_string(),
        ]);
        assert_eq!(result, Err(ConfErr::ArgRequired(ARG_FLAG_DATA.to_string())));
    }

    #[test]
    fn run_should_require_data_schema_flag() {
        let result = Conf::new(&[
            "--mode=run".to_string(),
            "--source-code=sample.eli".to_string(),
            "--data=data.csv".to_string(),
        ]);
        assert_eq!(
            result,
            Err(ConfErr::ArgRequired(ARG_FLAG_DATA_SCHEMA.to_string()))
        );
    }

    #[test]
    fn run_should_construct_conf() {
        let result = Conf::new(&[
            "--mode=run".to_string(),
            "--source-code=sample.eli".to_string(),
            "--data=data.csv".to_string(),
            "--data-schema=data.elt".to_string(),
            "--output=res.txt".to_string(),
        ]);
        assert_eq!(
            result,
            Ok(Conf::Run(ModeRunConf {
                source_code_path: "sample.eli".to_string(),
                data_path: "data.csv".to_string(),
                data_schema_path: "data.elt".to_string(),
                print_bytecode: false,
                output_path: Some("res.txt".to_string()),
            }))
        );
    }

    #[test]
    fn run_should_construct_conf_with_bytecode_enabled_flag() {
        let result = Conf::new(&[
            "--mode=run".to_string(),
            "--source-code=sample.eli".to_string(),
            "--data=data.csv".to_string(),
            "--data-schema=data.elt".to_string(),
            "--print-bytecode".to_string(),
        ]);
        assert_eq!(
            result,
            Ok(Conf::Run(ModeRunConf {
                source_code_path: "sample.eli".to_string(),
                data_path: "data.csv".to_string(),
                data_schema_path: "data.elt".to_string(),
                print_bytecode: true,
                output_path: None,
            }))
        );
    }

    // RUN MODE TESTS END

    // BUILD MODE TESTS START

    #[test]
    fn build_should_require_source_code_flag() {
        let result = Conf::new(&[
            "--mode=build".to_string(),
            "--data-schema=data.elt".to_string(),
            "--output=sample.elc".to_string(),
        ]);
        assert_eq!(
            result,
            Err(ConfErr::ArgRequired(ARG_FLAG_SOURCE_CODE.to_string()))
        );
    }

    #[test]
    fn build_should_require_data_schema_flag() {
        let result = Conf::new(&[
            "--mode=build".to_string(),
            "--source-code=sample.eli".to_string(),
            "--output=sample.elc".to_string(),
        ]);

        assert_eq!(
            result,
            Err(ConfErr::ArgRequired(ARG_FLAG_DATA_SCHEMA.to_string()))
        );
    }

    #[test]
    fn build_should_require_output_flag() {
        let result = Conf::new(&[
            "--mode=build".to_string(),
            "--source-code=sample.eli".to_string(),
            "--data-schema=data.elt".to_string(),
        ]);
        assert_eq!(
            result,
            Err(ConfErr::ArgRequired(ARG_FLAG_OUTPUT.to_string()))
        );
    }

    #[test]
    fn build_should_construct_conf() {
        let result = Conf::new(&[
            "--mode=build".to_string(),
            "--source-code=sample.eli".to_string(),
            "--data-schema=data.elt".to_string(),
            "--output=sample.elc".to_string(),
        ]);
        assert_eq!(
            result,
            Ok(Conf::Build(ModeBuildConf {
                source_code_path: "sample.eli".to_string(),
                executable_output_path: "sample.elc".to_string(),
                data_schema_path: "data.elt".to_string(),
            }))
        );
    }

    // BUILD MODE TESTS END

    // EXEC MODE TESTS START

    #[test]
    fn exec_should_require_executable_flag() {
        let result = Conf::new(&["--mode=exec".to_string(), "--data=data.csv".to_string()]);
        assert_eq!(
            result,
            Err(ConfErr::ArgRequired(ARG_FLAG_EXECUTABLE.to_string()))
        );
    }

    #[test]
    fn exec_should_require_data_flag() {
        let result = Conf::new(&[
            "--mode=exec".to_string(),
            "--executable=sample.elc".to_string(),
        ]);
        assert_eq!(result, Err(ConfErr::ArgRequired(ARG_FLAG_DATA.to_string())));
    }

    #[test]
    fn exec_should_construct_conf() {
        let result = Conf::new(&[
            "--mode=exec".to_string(),
            "--executable=sample.elc".to_string(),
            "--data=data.csv".to_string(),
        ]);
        assert_eq!(
            result,
            Ok(Conf::Exec(ModeExecConf {
                executable_path: "sample.elc".to_string(),
                data_path: "data.csv".to_string(),
            }))
        );
    }

    // EXEC MODE TESTS END

    // VALIDATE MODE TESTS START

    #[test]
    fn validate_should_require_data_flag() {
        let result = Conf::new(&[
            "--mode=validate".to_string(),
            "--data-schema=data.elt".to_string(),
        ]);
        assert_eq!(result, Err(ConfErr::ArgRequired(ARG_FLAG_DATA.to_string())));
    }

    #[test]
    fn validate_should_require_data_schema_flag() {
        let result = Conf::new(&["--mode=validate".to_string(), "--data=data.csv".to_string()]);
        assert_eq!(
            result,
            Err(ConfErr::ArgRequired(ARG_FLAG_DATA_SCHEMA.to_string()))
        );
    }

    #[test]
    fn validate_should_construct_conf() {
        let result = Conf::new(&[
            "--mode=validate".to_string(),
            "--data=data.csv".to_string(),
            "--data-schema=sample.elt".to_string(),
        ]);
        assert_eq!(
            result,
            Ok(Conf::Validate(ModeValidateConf {
                data_path: "data.csv".to_string(),
                data_schema_path: "sample.elt".to_string(),
            }))
        );
    }

    // VALIDATE MODE TESTS END
}

// ==========================
//
// TESTS END
//
// ==========================
