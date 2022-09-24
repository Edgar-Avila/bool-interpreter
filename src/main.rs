mod token;
mod lexer;
mod nodes;
mod parser;

use crate::lexer::Lexer;
use crate::parser::Parser;

fn main() {
    let code = String::from("false or (true or false) and true");

    // Lexer
    let mut l = Lexer::new(code.chars());
    let tokens = l.get_tokens();
    println!("The tokens are: {:?}", tokens);

    // Parser
    let mut p = Parser::new(tokens.iter());
    let ast = p.parse();
    println!("The ast generated evaluates the expression as follows: {}", ast);

    // Show final resutl
    let value = ast.eval();
    println!("The value evaluated is: {}", value);
}
