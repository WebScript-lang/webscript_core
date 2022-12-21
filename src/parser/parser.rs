use combine::stream::position::Stream as PositionStream;
use combine::Positioned;
use combine::{
    easy::Errors,
    parser::{repeat, EasyParser},
};

use super::expression;
use super::{tokens::terminator, Expression};
use crate::tokenizer::TokenWithPosition;

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
