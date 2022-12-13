use crate::{
    parser::{parser::expression, tokens::*, Expression},
    tokenizer::TokenWithPosition,
};
use combine::{optional, parser, Stream};

parser! {
    /// Syntax:
    /// ```
    /// return [<expression>]
    pub fn return_[I]()(I) -> Expression
    where [ I: Stream<Token=TokenWithPosition> ] {

        ignore_newlines!(
            keyword(Keyword::Return),
            optional(expression()), // value
        )

        .map(|(_, value)| Expression::Return (
            Box::new(value.unwrap_or(Expression::Null))
        ))
    }
}
