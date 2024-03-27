pub mod lexer;

use lexer::tokenize;

fn main() {
    let tokens = tokenize("-33 : [] : () -> 3.45");
        
    println!("{:#?}", tokens);
}
