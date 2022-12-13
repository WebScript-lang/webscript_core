use crate::{
    parser::{tokens::*, Expression},
    tokenizer::TokenWithPosition,
};
use combine::{choice, parser, parser::repeat, Stream};

use super::op6_relational::relational_operation;

parser! {
    /// Equality operator (==, !=)
    fn equality_operator[I]()(I) -> Operator
    where [ I: Stream<Token=TokenWithPosition> ] {

        choice((
            operator(Operator::Equal),
            operator(Operator::NotEqual),
        ))
    }
}

parser! {
    /// Equality operations (==, !=)
    pub fn equality_operation[I]()(I) -> Expression
    where [ I: Stream<Token=TokenWithPosition> ] {

        repeat::chainl1(
            relational_operation(), // allows to nest higher-order operations
            equality_operator().map(|op| move |l, r| {
                match op {
                    Operator::Equal => Expression::Equal(Box::new((l, r))),
                    Operator::NotEqual => Expression::NotEqual(Box::new((l, r))),
                _ => unreachable!(),
                }
            })
        )
    }
}
