const ARG_PREF: &str = "--";
const ARG_DIVIDER: &str = "=";

struct ParsedArg {
    pub name: String,
    pub value: String,
}

pub mod input {
    use crate::conf::input::ParsedArg;

    pub fn parse_args(_args: &Vec<&str>, names: &Vec<&str>) -> Vec<ParsedArg> {
        let res: Vec<ParsedArg> = Vec::new();

        if names.len() == 0 {
            return res;
        } else {
            return res;
        }
    }
}
