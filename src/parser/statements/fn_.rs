use crate::{
    parser::{expression, tokens::*, Expression, FunctionData},
    tokenizer::TokenWithPosition,
};
use combine::{optional, parser, parser::repeat, Stream};

parser! {
    /// Syntax:
    /// ```
    /// fn <identifier> ( <param>* ) [-> <type>] { <expression>* }
    /// param ::= <identifier> : <type> ,
    /// ```
    pub fn fn_[I]()(I) -> Expression
    where [ I: Stream<Token=TokenWithPosition> ] {

        keyword(Keyword::Fn).with(
            ignore_newlines!(
                identifier(), // name
                punctuator(Punctuator::LeftParen),
                repeat::sep_end_by( // args
                    ignore_newlines!(
                        identifier(), // arg name
                        punctuator(Punctuator::Colon),
                        type_(), // arg type
                    ).map(|(name, _, type_)| (name, type_)),
                    punctuator(Punctuator::Comma),
                ),
                punctuator(Punctuator::RightParen),
                optional( // return_type
                    ignore_newlines!(
                        operator(Operator::Arrow),
                        type_(),
                    ).map(|(_, type_)| type_),
                ),
                punctuator(Punctuator::LeftBrace),
                repeat::sep_end_by(expression(), terminator()), //body
                punctuator(Punctuator::RightBrace),
            )
        )

        .map(|(name, _, args, _, return_type, _, body, _)| Expression::Function(Box::new(FunctionData {
            name: Some(name),
            args,
            return_type,
            body,
        })))
    }
}
