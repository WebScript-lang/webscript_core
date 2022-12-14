use crate::{
    parser::{tokens::*, Expression},
    tokenizer::TokenWithPosition,
};
use combine::{between, choice, optional, parser, parser::repeat, Stream};

use self::op9_logical::logical_or_operation;

// Operator precedence
// Category         | Operator    | Associativity
// -----------------+-------------+-----------------
// Unitary          | - + ! ~     | Right
// Power            | **          | Right
// Multiplicative   | * / %       | Left
// Additive         | + -         | Left
// Shift            | << >>       | Left
// Relational       | < > <= =>   | Left
// Equality         | == !=       | Left
// Bitwise AND      | &           | Left
// Bitwise XOR      | ^           | Left
// Bitwise OR       | |           | Left
// Logical AND      | &&          | Left
// Logical OR       | ||          | Left
// Assignment       | = *= /= **= | Right
//                  | += -= &= %=
//                  | >>= <<= |= ^=

mod op10_assignment;
mod op1_unitary;
mod op2_power;
mod op3_multiplicative;
mod op4_additive;
mod op5_shift;
mod op6_relational;
mod op7_equality;
mod op8_bitwise;
mod op9_logical;

pub use op10_assignment::assignment_operation;

use super::CallData;

parser! {
    pub fn operation[I]()(I) -> Expression
    where [ I: Stream<Token=TokenWithPosition> ] {

        logical_or_operation()
    }
}

parser! {
    fn parenthesis[I]()(I) -> Expression
    where [ I: Stream<Token=TokenWithPosition> ] {

        between(
            punctuator(Punctuator::LeftParen),
            punctuator(Punctuator::RightParen),
            operation(),
        )
    }
}

parser! {
    fn literal[I]()(I) -> Expression
    where [ I: Stream<Token=TokenWithPosition> ] {

        choice((
            number().map(|v| Expression::Number(v)),
            integer().map(|v| Expression::Integer(v)),
            string().map(|v| Expression::String(v)),
            boolean().map(|v| Expression::Boolean(v)),
            null().map(|_| Expression::Null),
        ))
    }
}

parser! {
    fn highest_operation[I]()(I) -> Expression
    where [ I: Stream<Token=TokenWithPosition> ] {

        choice((
            parenthesis(),
            literal(),
            identifier().map(|name| Expression::Identifier(name)),
        )).and(
            optional(repeat::many1(
                between( // function call
                    punctuator(Punctuator::LeftParen),
                    punctuator(Punctuator::RightParen),
                    repeat::sep_end_by(operation(), punctuator(Punctuator::Comma)),
                )
            ))
        ).map(|(expr, calls): (_, Option<Vec<_>>)|
            if let Some(calls) = calls {
                let mut expr = expr;
                for args in calls {
                    expr = Expression::Call(Box::new(CallData { ref_: expr, args }));
                }
                expr
            } else {
                expr
            }
        )
    }
}
