const ARG_PREF: &str = "--";
const ARG_DIVIDER: &str = "=";

pub struct ParsedArg {
    pub name: String,
    pub value: String,
}

pub mod input {
    use crate::conf::input::ParsedArg;

    // Take the list of provided arguments by user (args)
    // and the list of argument names that we want to parse (names).
    // Return the list of parsed argument values in the same order as names.
    // TODO: Replace Vec<ParsedArg> with HashMap<String, String>.
    // This function should not validate the arguments, just parse them.
    // Validation should be done by caller.
    pub fn parse_cli_args(_args: &Vec<String>, names: &Vec<&str>) -> Vec<ParsedArg> {
        let res: Vec<ParsedArg> = Vec::new();

        if names.len() == 0 {
            return res;
        } else {
            return res;
        }
    }
}
