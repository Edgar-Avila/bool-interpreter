use crate::token::Token;
use std::str::Chars;

pub struct Lexer<'a> {
    curr_char: Option<char>,
    it: Chars<'a>,
}

impl<'a> Lexer<'a> {
    pub fn new(char_it: Chars<'a>) -> Self {
        let mut l = Lexer {
            curr_char: None,
            it: char_it
        };
        l.advance();
        l
    }

    fn advance(&mut self) {
        self.curr_char = self.it.next();
    }

    fn verify(&mut self, s: &str) {
        for c in s.chars() {
            self.advance();
            if self.curr_char.unwrap() != c {
                panic!("Invalid token");
            }
        }
    }

    pub fn get_tokens(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        while let Some(c) = self.curr_char {
            match c {
                ' ' | '\t' | '\n' => self.advance(),
                '(' => {
                    tokens.push(Token::Lparen);
                    self.advance();
                },
                ')' => {
                    tokens.push(Token::RParen);
                    self.advance();
                },
                't' => {
                    let rest = "rue";
                    self.verify(&rest);
                    self.advance();
                    tokens.push(Token::Bool(true));
                },
                'f' => {
                    let rest = "alse";
                    self.verify(&rest);
                    self.advance();
                    tokens.push(Token::Bool(false));

                },
                'a' => {
                    let rest = "nd";
                    self.verify(&rest);
                    self.advance();
                    tokens.push(Token::And);
                },
                'o' => {
                    let rest = "r";
                    self.verify(&rest);
                    self.advance();
                    tokens.push(Token::Or);
                }
                _ => (),
            }
        }
        tokens
    }
}
