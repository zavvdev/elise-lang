const ARG_PREF: &str = "--";
const ARG_DIVIDER: &str = "=";

pub mod input {
    use std::collections::HashMap;

    // Take the list of provided arguments by user (args)
    // and the list of argument names that we want to parse (names).
    // Returns HashMap where keys are values from names argument
    // and values are Option<String>.
    pub fn parse_cli_args(_args: &Vec<String>, _names: &Vec<&str>) -> HashMap<String, String> {
        let res: HashMap<String, String> = HashMap::new();
        return res;
    }
}
