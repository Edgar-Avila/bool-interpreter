#[derive(Debug)]
pub enum Token {
    Bool(bool),
    And,
    Or,
    Lparen,
    RParen,
}
