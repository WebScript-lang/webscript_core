use crate::{
    parser::{tokens::*, Expression},
    tokenizer::TokenWithPosition,
};
use combine::{parser, parser::repeat, Stream};

use super::op8_bitwise::bitwise_or_operation;

parser! {
    /// Logical AND operator (&&)
    fn logical_and_operator[I]()(I) -> Operator
    where [ I: Stream<Token=TokenWithPosition> ] {

        operator(Operator::And)
    }
}

parser! {
    /// Logical AND operation (&&)
    pub fn logical_and_operation[I]()(I) -> Expression
    where [ I: Stream<Token=TokenWithPosition> ] {

        repeat::chainl1(
            bitwise_or_operation(), // allows to nest higher-order operations
            logical_and_operator().map(|_| move |l, r| {
                Expression::And(Box::new((l, r)))
            })
        )
    }
}

parser! {
    /// Logical OR operator (||)
    fn logical_or_operator[I]()(I) -> Operator
    where [ I: Stream<Token=TokenWithPosition> ] {

        operator(Operator::Or)
    }
}

parser! {
    /// Logical OR operation (||)
    pub fn logical_or_operation[I]()(I) -> Expression
    where [ I: Stream<Token=TokenWithPosition> ] {

        repeat::chainl1(
            logical_and_operation(), // allows to nest higher-order operations
            logical_or_operator().map(|_| move |l, r| {
                Expression::Or(Box::new((l, r)))
            })
        )
    }
}
