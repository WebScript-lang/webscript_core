use crate::{
    parser::{tokens::*, Expression},
    tokenizer::TokenWithPosition,
};
use combine::{choice, parser, parser::repeat, Stream};

use super::op5_shift::shift_operation;

parser! {
    /// Relational operator (<, >, <=, >=)
    fn relational_operator[I]()(I) -> Operator
    where [ I: Stream<Token=TokenWithPosition> ] {

        choice((
            operator(Operator::LessThan),
            operator(Operator::GreaterThan),
            operator(Operator::LessOrEqual),
            operator(Operator::GreaterOrEqual),
        ))
    }
}

parser! {
    /// Relational operations (<, >, <=, >=)
    pub fn relational_operation[I]()(I) -> Expression
    where [ I: Stream<Token=TokenWithPosition> ] {

        repeat::chainl1(
            shift_operation(), // allows to nest higher-order operations
            relational_operator().map(|op| move |l, r| {
                match op {
                    Operator::LessThan => Expression::LessThan(Box::new((l, r))),
                    Operator::GreaterThan => Expression::GreaterThan(Box::new((l, r))),
                    Operator::LessOrEqual => Expression::LessOrEqual(Box::new((l, r))),
                    Operator::GreaterOrEqual => Expression::GreaterOrEqual(Box::new((l, r))),
                _ => unreachable!(),
                }
            })
        )
    }
}
