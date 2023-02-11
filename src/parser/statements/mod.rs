use super::Expression;
use crate::tokenizer::TokenWithPosition;

use combine::{choice, parser, Stream};

mod class;
mod fn_;
mod return_;
mod var;

parser! {
    /// Any expression that is a statement.
    /// It cannot be used as value.
    pub fn statement[I]()(I) -> Expression
    where [ I: Stream<Token=TokenWithPosition> ] {

        choice((
            fn_::fn_(),
            class::class(),
            var::var(),
            return_::return_(),
        ))
    }
}
