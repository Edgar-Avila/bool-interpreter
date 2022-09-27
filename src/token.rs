use std::fmt::Display;

#[derive(Debug, PartialEq)]
pub enum Token {
    Bool(bool),
    And,
    Or,
    Lparen,
    RParen,
    Not,
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Bool(val) => write!(f, "{}", val),
            Self::And => write!(f, "and"),
            Self::Or => write!(f, "or"),
            Self::Lparen => write!(f, "("),
            Self::RParen => write!(f, ")"),
            Self::Not => write!(f, "not"),
        }
    }
}
