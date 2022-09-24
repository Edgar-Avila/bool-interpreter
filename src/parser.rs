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
        let term = self.term();
        self.advance();
        if let Some(token) = self.curr_token {
            match token {
                Token::Or => {
                    self.advance();
                    let t2 = self.expr();
                    return Node::Or {
                        left: Box::new(term),
                        right: Box::new(t2),
                    };
                }
                Token::And => {
                    self.advance();
                    let t2 = self.expr();
                    return Node::And {
                        left: Box::new(term),
                        right: Box::new(t2),
                    };
                }
                _ => (),
            }
        }
        term
    }
    fn term(&mut self) -> Node {
        let token = match self.curr_token {
            Some(token) => token,
            None => panic!("Invalid syntax, expected a term, got nothing"),
        };
        match token {
            Token::Bool(val) => Node::Bool(*val),
            Token::Lparen => {
                self.advance();
                let e = self.expr();
                if let Some(token) = self.curr_token {
                    match token {
                        Token::RParen => e,
                        _ => panic!("Invalid syntax, expected ')', got '{}'", token)
                    }
                } else {
                    panic!("Invalid syntax, expected ')' got nothing");
                }
            }
            _ => panic!("Invalid syntax, expected a term, got '{}'", token),
        }
    }
}
