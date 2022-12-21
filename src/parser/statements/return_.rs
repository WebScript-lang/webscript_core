use crate::{
    parser::{operations::operation, tokens::*, Expression},
    tokenizer::TokenWithPosition,
};
use combine::{optional, parser, Stream};

parser! {
    /// Syntax:
    /// ```
    /// return [<operation>]
    pub fn return_[I]()(I) -> Expression
    where [ I: Stream<Token=TokenWithPosition> ] {

        keyword(Keyword::Return).and(
            optional(operation())
        )

        .map(|(_, value)| Expression::Return (
            Box::new(value.unwrap_or(Expression::Null))
        ))
    }
}
