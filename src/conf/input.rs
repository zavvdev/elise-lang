pub mod input {
    use std::collections::HashMap;

    use regex::Regex;

    // Take the list of provided arguments by user (args)
    // and the list of argument names that we want to parse (names).
    // Returns HashMap where keys are values from names argument
    // and values are Option<String>.
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

        return res;
    }
}
