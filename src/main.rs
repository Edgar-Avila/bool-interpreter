mod token;
mod lexer;

use crate::lexer::Lexer;

fn main() {
    let code = String::from("(true or false) and ");
    let it = code.chars();
    let mut l = Lexer::new(it);
    let tokens = l.get_tokens();
    println!("{:?}", tokens);
}
