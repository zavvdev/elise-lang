use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    for arg in args.iter().skip(1) {
        println!("{:?}", arg);
    }
}
