use std::fmt::Display;

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
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
