use crate::{
    parser::{tokens::*, Expression},
    tokenizer::TokenWithPosition,
};
use combine::{parser, parser::repeat, Stream};

use super::op7_equality::equality_operation;

parser! {
    /// Bitwise AND operator (&)
    fn bitwise_and_operator[I]()(I) -> Operator
    where [ I: Stream<Token=TokenWithPosition> ] {

        operator(Operator::BitwiseAnd)
    }
}

parser! {
    /// Bitwise AND operation (&)
    pub fn bitwise_and_operation[I]()(I) -> Expression
    where [ I: Stream<Token=TokenWithPosition> ] {

        repeat::chainl1(
            equality_operation(), // allows to nest higher-order operations
            bitwise_and_operator().map(|_| move |l, r| {
                Expression::BitwiseAnd(Box::new((l, r)))
            })
        )
    }
}

parser! {
    /// Bitwise XOR operator (^)
    fn bitwise_xor_operator[I]()(I) -> Operator
    where [ I: Stream<Token=TokenWithPosition> ] {

        operator(Operator::BitwiseXor)
    }
}

parser! {
    /// Bitwise XOR operation (^)
    pub fn bitwise_xor_operation[I]()(I) -> Expression
    where [ I: Stream<Token=TokenWithPosition> ] {

        repeat::chainl1(
            bitwise_and_operation(), // allows to nest higher-order operations
            bitwise_xor_operator().map(|_| move |l, r| {
                Expression::BitwiseXor(Box::new((l, r)))
            })
        )
    }
}

parser! {
    /// Bitwise OR operator (|)
    fn bitwise_or_operator[I]()(I) -> Operator
    where [ I: Stream<Token=TokenWithPosition> ] {

        operator(Operator::BitwiseOr)
    }
}

parser! {
    /// Bitwise OR operation (|)
    pub fn bitwise_or_operation[I]()(I) -> Expression
    where [ I: Stream<Token=TokenWithPosition> ] {

        repeat::chainl1(
            bitwise_xor_operation(), // allows to nest higher-order operations
            bitwise_or_operator().map(|_| move |l, r| {
                Expression::BitwiseOr(Box::new((l, r)))
            })
        )
    }
}
