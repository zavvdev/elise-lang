pub struct Config {
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Self {
        if args.len() < 2 {
            panic!("Not enough arguments");
        }

        let filename = args[1].clone();

        Self { filename }
    }
}
