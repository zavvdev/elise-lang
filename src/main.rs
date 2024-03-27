pub mod lexer;

use lexer::tokenize;

fn main() {
    let tokens = tokenize("33");
        
    println!("{:#?}", tokens);
}
