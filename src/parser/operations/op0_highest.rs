use combine::{between, choice, optional, parser, parser::repeat, Stream};

use crate::{
    parser::{
        tokens::{self, *},
        CallData, Expression,
    },
    tokenizer::TokenWithPosition,
};

use super::operation;

parser! {
    /// Variable or reference you can assign new values to
    pub fn assignable_operation[I]()(I) -> Expression
    where [ I: Stream<Token=TokenWithPosition> ] {
        // TODO: allow to assign to object properties, array elements, etc.
        identifier()
    }
}

parser! {
    /// Higher order operation (parenthesis, literals)
    fn higher_operation[I]()(I) -> Expression
    where [ I: Stream<Token=TokenWithPosition> ] {

        choice((
            parenthesis(),
            literal(),
            assignable_operation(),
        ))
    }
}

parser! {
    /// Parenthesis ( `( )` )
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
    /// Identifier
    fn identifier[I]()(I) -> Expression
    where [ I: Stream<Token=TokenWithPosition> ] {

        tokens::identifier().map(|name| Expression::Identifier(name))
    }
}

parser! {
    /// Literals (Number, Integer, String, Boolean, null) and identifiers
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
    /// Highest order operation (parenthesis, literals, function calls)
    pub fn highest_operation[I]()(I) -> Expression
    where [ I: Stream<Token=TokenWithPosition> ] {

        higher_operation().and(
            optional(
                repeat::many1( // allow multiple function calls
                    between( // function call
                        punctuator(Punctuator::LeftParen),
                        punctuator(Punctuator::RightParen),
                        repeat::sep_end_by(operation(), punctuator(Punctuator::Comma)), // args
                    )
                ).map(|funcs: Vec<_>| funcs)
            ).map(|funcs| funcs.unwrap_or_default()),
        )

        .map(|(expr, funcs)| funcs // fold function calls
            .into_iter()
            .fold(expr, |expr, args| {
                Expression::Call(Box::new(CallData { ref_: expr, args }))
            })
        )
    }
}
