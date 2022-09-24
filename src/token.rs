#[derive(Debug)]
pub enum TokenType {
    Bool,
    And,
    Or,
    Lparen,
    RParen,
}

#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub val: Option<bool>,
}

impl Token {
    pub fn new(token_type: TokenType, val: Option<bool>) -> Self {
        Token { token_type, val }
    }
}
