mod lexer;
mod nodes;
mod parser;
mod token;

use std::io::{self, Write};

use crate::lexer::Lexer;
use crate::parser::Parser;

fn main() {
    loop {
        // Read input
        print!(">>> ");
        io::stdout().flush().expect("Unable to flush stdout");
        let mut code = String::new();
        io::stdin()
            .read_line(&mut code)
            .expect("Could not read input");

        // Lexer
        let mut l = Lexer::new(code.chars());
        let tokens = l.get_tokens();
        println!("The tokens are: {:?}", tokens);

        // Parser
        let mut p = Parser::new(tokens.iter());
        let ast = p.parse();
        println!(
            "The ast generated evaluates the expression as follows: {}",
            ast
        );

        // Show final resutl
        let value = ast.eval();
        println!("The value evaluated is: {}", value);
    }
}
