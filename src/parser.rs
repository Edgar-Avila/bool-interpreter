use std::slice::Iter;

use crate::{nodes::Node, token::Token};

pub struct Parser<'a> {
    curr_token: Option<&'a Token>,
    it: Iter<'a, Token>,
}

impl<'a> Parser<'a> {
    pub fn new(it: Iter<'a, Token>) -> Self {
        let mut p = Parser {
            curr_token: None,
            it,
        };
        p.advance();
        p
    }
    fn advance(&mut self) {
        self.curr_token = self.it.next();
    }
    pub fn parse(&mut self) -> Node {
        self.expr()
    }
    fn expr(&mut self) -> Node {
        let mut result = self.term();
        while let Some(token) = self.curr_token {
            match token {
                Token::Or => {
                    self.advance();
                    result = Node::Or {
                        left: Box::new(result),
                        right: Box::new(self.term()),
                    };
                }
                Token::And => {
                    self.advance();
                    result = Node::And {
                        left: Box::new(result),
                        right: Box::new(self.term()),
                    };
                }
                _ => break,
            }
        }
        result
    }
    fn term(&mut self) -> Node {
        let token = match self.curr_token {
            Some(token) => token,
            None => panic!("Invalid syntax, expected a term, got nothing"),
        };
        match token {
            Token::Bool(val) => {
                self.advance();
                Node::Bool(*val)
            }
            Token::Lparen => {
                self.advance();
                let e = self.expr();
                if let Some(token) = self.curr_token {
                    match token {
                        Token::RParen => {
                            self.advance();
                            e
                        }
                        _ => panic!("Invalid syntax, expected ')', got '{}'", token),
                    }
                } else {
                    panic!("Invalid syntax, expected ')' got nothing");
                }
            }
            _ => panic!("Invalid syntax, expected a term, got '{}'", token),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Parser;
    use crate::Lexer;

    fn test(s: &str, expected: bool) {
        let tokens = Lexer::new(s.chars()).get_tokens();
        let mut parser = Parser::new(tokens.iter());
        assert_eq!(parser.parse().eval(), expected);
    }

    #[test]
    fn single_bool() {
        test("true", true);
        test("false", false);
    }

    #[test]
    fn binary_and() {
        test("true and true", true);
        test("true and false", false);
        test("false and true", false);
        test("false and false", false);
    }

    #[test]
    fn binary_or() {
        test("true or true", true);
        test("true or false", true);
        test("false or true", true);
        test("false or false", false);
    }

    #[test]
    fn associativity() {
        test("true or true and false", false);
        test("false and false or true", true);
    }

    #[test]
    fn expr_with_parentheses() {
        test("(true or true) and false", false);
        test("(false and false) or true", true);
        test("true or (true and false)", true);
        test("false and (false or true)", false);
    }
}
