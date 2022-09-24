use std::fmt::Display;

pub enum Node {
    Bool(bool),
    And { left: Box<Node>, right: Box<Node> },
    Or { left: Box<Node>, right: Box<Node> },
}

impl Node {
    pub fn eval(&self) -> bool {
        match self {
            Self::Bool(val) => *val,
            Self::And { left, right } => left.eval() && right.eval(),
            Self::Or { left, right } => left.eval() || right.eval(),
        }
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Bool(val) => write!(f, "{}", val),
            Self::And { left, right } => write!(f, "({} and {})", left, right),
            Self::Or { left, right } => write!(f, "({} or {})", left, right),
        }
    }
}
