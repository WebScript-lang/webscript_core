use crate::{
    parser::{tokens::*, Expression},
    tokenizer::TokenWithPosition,
};
use combine::{choice, parser, parser::repeat, Stream};

use super::op2_power::power_operation;

parser! {
    /// MultiOplicative operator (*, /, %)
    fn multiplicative_operator[I]()(I) -> Operator
    where [ I: Stream<Token=TokenWithPosition> ] {

        choice((
            operator(Operator::Multiply),
            operator(Operator::Divide),
            operator(Operator::Modulo),
        ))
    }
}

parser! {
    /// Multiplicative operations (*, /, %)
    pub fn multiplicative_operation[I]()(I) -> Expression
    where [ I: Stream<Token=TokenWithPosition> ] {

        repeat::chainl1(
            power_operation(), // allows to nest higher-order operations
            multiplicative_operator().map(|op| move |l, r| {
                match op {
                    Operator::Multiply => Expression::Mul(Box::new((l, r))),
                    Operator::Divide => Expression::Div(Box::new((l, r))),
                    Operator::Modulo => Expression::Modulo(Box::new((l, r))),
                    _ => unreachable!(),
                }
            })
        )
    }
}
