use crate::{
    parser::{tokens::*, Expression},
    tokenizer::TokenWithPosition,
};
use combine::{choice, parser, Stream};

use super::{highest_operation, operation};

parser! {
    /// Unitary operator (- + ! ~)
    fn unitary_operator[I]()(I) -> Operator
    where [ I: Stream<Token=TokenWithPosition> ] {

        choice((
            operator(Operator::Minus),
            operator(Operator::Plus),
            operator(Operator::Not),
            operator(Operator::BitwiseNot),
        ))
    }
}

parser! {
    /// Unitary operation (- + ! ~)
    pub fn unitary_operation[I]()(I) -> Expression
    where [ I: Stream<Token=TokenWithPosition> ] {

        choice((
            highest_operation(), // allows to nest highest-order operations
            unitary_operator().and(operation()).map(|(op, expr)| {
                match op {
                    Operator::Minus => Expression::Minus(Box::new(expr)),
                    Operator::Plus => Expression::Plus(Box::new(expr)),
                    Operator::Not => Expression::Not(Box::new(expr)),
                    Operator::BitwiseNot => Expression::BitwiseNot(Box::new(expr)),
                    _ => unreachable!(),
                }
            }),
        ))
    }
}
