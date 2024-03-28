pub mod lexer;

use lexer::tokenize;

fn main() {
    let tokens = tokenize(" ");
        
    println!("{:#?}", tokens);
}
