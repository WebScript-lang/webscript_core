use crate::{
    parser::{tokens::*, AssignData, Expression},
    tokenizer::TokenWithPosition,
};
use combine::{choice, parser, Stream};

use super::{highest_operation, op9_logical::logical_and_operation};

parser! {
    /// Assignment operator (=, +=, -=, *=, /=, %=, **=, <<=, >>=, &=, ^=, |=)
    fn assignment_operator[I]()(I) -> Operator
    where [ I: Stream<Token=TokenWithPosition> ] {

        choice((
            operator(Operator::Assign),
            operator(Operator::PlusAssign),
            operator(Operator::MinusAssign),
            operator(Operator::PowerAssign),
            operator(Operator::MultiplyAssign),
            operator(Operator::DivideAssign),
            operator(Operator::ModuloAssign),
            operator(Operator::LeftShiftAssign),
            operator(Operator::RightShiftAssign),
            operator(Operator::BitwiseAndAssign),
            operator(Operator::BitwiseXorAssign),
            operator(Operator::BitwiseOrAssign),
        ))
    }
}

// TODO: allows to chain assignment operations
parser! {
    /// Assignment operations (=, +=, -=, *=, /=, %=, **=, <<=, >>=, &=, ^=, |=)
    pub fn assignment_operation[I]()(I) -> Expression
    where [ I: Stream<Token=TokenWithPosition> ] {

        (
            highest_operation(),
            assignment_operator(),
            logical_and_operation(), // allows to nest higher-order operations
        ).map(|(ref_, op, value)| {
            match op {
                Operator::Assign => Expression::Assign(Box::new(AssignData { ref_, value })),
                Operator::PlusAssign => Expression::AddAssign(Box::new(AssignData { ref_, value })),
                Operator::MinusAssign => Expression::SubAssign(Box::new(AssignData { ref_, value })),
                Operator::PowerAssign => Expression::MulAssign(Box::new(AssignData { ref_, value })),
                Operator::MultiplyAssign => Expression::DivAssign(Box::new(AssignData { ref_, value })),
                Operator::DivideAssign => Expression::ModuloAssign(Box::new(AssignData { ref_, value })),
                Operator::ModuloAssign => Expression::PowerAssign(Box::new(AssignData { ref_, value })),
                Operator::LeftShiftAssign => Expression::LeftShiftAssign(Box::new(AssignData { ref_, value })),
                Operator::RightShiftAssign => Expression::RightShiftAssign(Box::new(AssignData { ref_, value })),
                Operator::BitwiseAndAssign => Expression::BitwiseAndAssign(Box::new(AssignData { ref_, value })),
                Operator::BitwiseXorAssign => Expression::BitwiseXorAssign(Box::new(AssignData { ref_, value })),
                Operator::BitwiseOrAssign => Expression::BitwiseOrAssign(Box::new(AssignData { ref_, value })),
                _ => unreachable!(),
            }
        })
    }
}
