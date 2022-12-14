use combine::{
    parser,
    parser::{repeat, token},
    Stream,
};

use crate::tokenizer::TokenWithPosition;
pub use crate::tokenizer::{Keyword, Operator, Punctuator, Token};

// Ignore newlines
macro_rules! ignore_newlines {
    ($arg:expr, $($args:expr),*$(,)?) => {
        ( $arg, $(newline().with($args),)* )
    }
}

pub(crate) use ignore_newlines;

// Keyword
parser! {
    /// Check if token is a keyword.
    pub fn keyword[I](keyword: Keyword)(I) -> ()
    where [ I: Stream<Token=TokenWithPosition> ] {

        token::satisfy(|token: TokenWithPosition| token.value == Token::Keyword(*keyword)).map(|_| ())
    }
}

// Punctuator
parser! {
    /// Check if token is a punctuator.
    pub fn punctuator[I](punctuator: Punctuator)(I) -> ()
    where [ I: Stream<Token=TokenWithPosition> ] {

        token::satisfy(|token: TokenWithPosition| token.value == Token::Punctuator(*punctuator)).map(|_| ())
    }
}

// Operator
parser! {
    /// Check if token is an operator.
    pub fn operator[I](operator: Operator)(I) -> Operator
    where [ I: Stream<Token=TokenWithPosition> ] {

        token::satisfy(|token: TokenWithPosition| token.value == Token::Operator(*operator)).map(|_| *operator)
    }
}

// Identifier
parser! {
    /// Check if token is an identifier.
    pub fn identifier[I]()(I) -> String
    where [ I: Stream<Token=TokenWithPosition> ] {

        token::satisfy(|token: TokenWithPosition| {
            if let Token::Identifier(_) = token.value {true} else {false}
        }).map(|token: TokenWithPosition| {
            if let Token::Identifier(identifier) = token.value { identifier }
            else { unreachable!() }
        })
    }
}

// Type
parser! {
    /// Check if token is a type.
    pub fn type_[I]()(I) -> String
    where [ I: Stream<Token=TokenWithPosition> ] {

        token::satisfy(|token: TokenWithPosition| {
            if let Token::Identifier(_) = token.value {true} else {false}
        }).map(|token: TokenWithPosition| {
            if let Token::Identifier(type_) = token.value { type_ }
            else { unreachable!() }
        })
    }
}

// Integer
parser! {
    /// Check if token is an integer.
    pub fn integer[I]()(I) -> i32
    where [ I: Stream<Token=TokenWithPosition> ] {

        token::satisfy(|token: TokenWithPosition| {
            if let Token::Integer(_) = token.value {true} else {false}
        }).map(|token: TokenWithPosition| {
            if let Token::Integer(value) = token.value { value }
            else { unreachable!() }
        })
    }
}

// Number
parser! {
    /// Check if token is a number.
    pub fn number[I]()(I) -> f64
    where [ I: Stream<Token=TokenWithPosition> ] {

        token::satisfy(|token: TokenWithPosition| {
            if let Token::Number(_) = token.value {true} else {false}
        }).map(|token: TokenWithPosition| {
            if let Token::Number(value) = token.value { value }
            else { unreachable!() }
        })
    }
}

// Boolean
parser! {
    /// Check if token is a boolean.
    pub fn boolean[I]()(I) -> bool
    where [ I: Stream<Token=TokenWithPosition> ] {

        token::satisfy(|token: TokenWithPosition| {
            if let Token::Boolean(_) = token.value {true} else {false}
        }).map(|token: TokenWithPosition| {
            if let Token::Boolean(value) = token.value { value }
            else { unreachable!() }
        })
    }
}

// String
parser! {
    /// Check if token is a string.
    pub fn string[I]()(I) -> String
    where [ I: Stream<Token=TokenWithPosition> ] {

        token::satisfy(|token: TokenWithPosition| {
            if let Token::String(_) = token.value {true} else {false}
        }).map(|token: TokenWithPosition| {
            if let Token::String(value) = token.value { value }
            else { unreachable!() }
        })
    }
}

// Null
parser! {
    /// Check if token is a null.
    pub fn null[I]()(I) -> ()
    where [ I: Stream<Token=TokenWithPosition> ] {

        token::satisfy(|token: TokenWithPosition| {
            if let Token::Null = token.value {true} else {false}
        }).map(|token: TokenWithPosition| {
            if let Token::Null = token.value { () }
            else { unreachable!() }
        })
    }
}

// Terminator
parser! {
    /// Check if token is a terminator.
    pub fn terminator[I]()(I) -> ()
    where [ I: Stream<Token=TokenWithPosition> ] {

        token::satisfy(|token: TokenWithPosition| token.value == Token::NewLine || token.value == Token::Terminator).map(|_| ())
    }
}

// NewLine
parser! {
    /// Allow an optional newlines.
    pub fn newline[I]()(I) -> ()
    where [ I: Stream<Token=TokenWithPosition> ] {

        repeat::skip_many(token::satisfy(|token: TokenWithPosition| token.value == Token::NewLine))
    }
}
