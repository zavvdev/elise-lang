pub mod lexer;

use lexer::tokenize;

fn main() {
    let tokens = tokenize("@add(2 @mul(3 @div(10 @sub(2 1))))");

    println!("{:#?}", tokens);
}
