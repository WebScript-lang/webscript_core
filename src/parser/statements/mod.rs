use crate::{parser::Expression, tokenizer::TokenWithPosition};
use combine::{choice, parser, Stream};

mod fn_;
mod return_;

parser! {
    pub fn statement[I]()(I) -> Expression
    where [ I: Stream<Token=TokenWithPosition> ] {

        choice((
            fn_::fn_(),
            return_::return_(),
        ))
    }
}
