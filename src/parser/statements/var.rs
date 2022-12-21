use crate::{
    parser::{operations::operation, tokens::*, Expression, VarData},
    tokenizer::TokenWithPosition,
};
use combine::{optional, parser, Stream};

parser! {
    /// Syntax:
    /// ```
    /// var <identifier> [: <type>] [ = <operation>]
    pub fn var[I]()(I) -> Expression
    where [ I: Stream<Token=TokenWithPosition> ] {

        keyword(Keyword::Var).with(
            ignore_newlines!(
                identifier(), // name

                optional(ignore_newlines!( // type
                    punctuator(Punctuator::Colon),
                    type_(),
                ).map(|(_, type_)| type_)),

                optional(ignore_newlines!( // value
                    operator(Operator::Assign),
                    operation(),
                ).map(|(_, value)| value))
            )
        )

        .map(|(name, type_, value)| Expression::Var (
            Box::new(VarData { name, type_, value })
        ))
    }
}
