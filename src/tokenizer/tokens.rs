use std::fmt::Display;

use combine::stream::position::SourcePosition;

use super::{keyword::Keyword, operator::Operator, punctuator::Punctuator};

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Identifier(String),
    Integer(i32),
    Number(f64),
    Boolean(bool),
    String(String),
    Null,
    Keyword(Keyword),
    Operator(Operator),
    Punctuator(Punctuator),
    Terminator,
    NewLine,
    // Empty,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TokenWithPosition {
    pub start: SourcePosition,
    pub end: SourcePosition,
    pub value: Token,
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Display for TokenWithPosition {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "[{}:{}] {}",
            self.start.line, self.start.column, self.value
        )
    }
}
