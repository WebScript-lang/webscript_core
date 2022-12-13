use crate::{
    parser::{tokens::*, Expression},
    tokenizer::TokenWithPosition,
};
use combine::{choice, parser, parser::repeat, Stream};

use super::op3_multiplicative::multiplicative_operation;

parser! {
    /// Additive operator (+, -)
    fn additive_operator[I]()(I) -> Operator
    where [ I: Stream<Token=TokenWithPosition> ] {

        choice((
            operator(Operator::Plus),
            operator(Operator::Minus),
        ))
    }
}

parser! {
    /// Additive operations (+, -)
    pub fn additive_operation[I]()(I) -> Expression
    where [ I: Stream<Token=TokenWithPosition> ] {

        repeat::chainl1(
            multiplicative_operation(), // allows to nest higher-order operations
            additive_operator().map(|op| move |l, r| {
                match op {
                    Operator::Plus => Expression::Add(Box::new((l, r))),
                    Operator::Minus => Expression::Sub(Box::new((l, r))),
                    _ => unreachable!(),
                }
            })
        )
    }
}
