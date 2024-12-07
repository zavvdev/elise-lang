const EXT: &str = "el";

pub struct Config {
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Self {
        if args.len() < 2 {
            panic!("Not enough arguments");
        }

        let filename = args[1].clone();
        let ext = filename.split(".").last();

        match ext {
            Some(x) => {
                if x == EXT {
                    return Self { filename };
                } else {
                    panic!("File extention should be \".{}\"", EXT);
                }
            }
            None => {
                panic!("Missing file extension");
            }
        }
    }
}
