use combine::stream::position::Stream as PositionStream;
use combine::{choice, Positioned};
use combine::{
    easy::Errors,
    parser,
    parser::{repeat, EasyParser},
    Stream,
};

use crate::tokenizer::TokenWithPosition;

use super::operations::operation;
use super::statements::statement;
use super::tokens::identifier;
use super::{tokens::terminator, Expression};

parser! {
    pub fn expression[I]()(I) -> Expression
    where [ I: Stream<Token=TokenWithPosition> ] {

        choice!(
            statement(),
            operation(),
            identifier().map(|name| Expression::Identifier(name))
        )
    }
}

pub fn parse(
    tokens: &[TokenWithPosition],
) -> Result<Vec<Expression>, Errors<TokenWithPosition, &[TokenWithPosition], usize>> {
    let parser = || repeat::sep_end_by(expression(), terminator());
    let expressions = parser().easy_parse(PositionStream::new(tokens));

    match expressions {
        Ok((expressions, stream)) => {
            // Check if there are any tokens left.
            if !stream.input.is_empty() {
                eprintln!("[Parser]");
                eprintln!("Unparsed:");
                for token in stream.input {
                    eprintln!("  {token:?}");
                }
                eprintln!("\nPosition: {}\n", stream.position());
            }

            // Return the expressions.
            Ok(expressions)
        }
        Err(errors) => Err(errors),
    }
}
