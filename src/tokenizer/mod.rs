use combine::parser::{char, repeat};
use combine::parser::{EasyParser, Parser};
use combine::stream::position::Stream;
use combine::stream::RangeStream;
use combine::{attempt, choice, position, StreamOnce};

mod identifier;
mod keyword;
mod literals;
mod operator;
mod punctuator;
mod spaces;
mod tokens;

pub use keyword::*;
pub use operator::*;
pub use punctuator::*;
pub use spaces::{ignore_spaces, separator};
pub use tokens::{Token, TokenWithPosition};

pub fn parse<'src>(source: &'src str) -> Vec<TokenWithPosition> {
    let tokens = tokenize().easy_parse(Stream::new(source));

    match tokens {
        Ok((tokens, stream)) => {
            // Check if there are any tokens left.
            if !stream.input.is_empty() {
                eprintln!("[Tokenizer error]");
                eprintln!("Unparsed: {}", stream.input);
                eprintln!(
                    "Position: [{}:{}]",
                    stream.positioner.line, stream.positioner.column
                );
            }

            let mut output = Vec::with_capacity(tokens.len());
            for (start, token, end) in tokens {
                output.push(TokenWithPosition {
                    start,
                    end,
                    value: token,
                });
            }

            output
        }
        Err(errors) => {
            eprintln!("[Tokenizer error]");
            eprintln!(
                "position: [{}:{}]",
                errors.position.line, errors.position.column
            );
            eprintln!("error: {:?}\n", errors.errors);
            panic!();
        }
    }
}

fn tokenize<'src, I>() -> impl Parser<
    I,
    Output = Vec<(
        <I as StreamOnce>::Position,
        Token,
        <I as StreamOnce>::Position,
    )>,
> + 'src
where
    I: RangeStream<Token = char, Range = &'src str> + 'src,
{
    let token = choice((
        literals::literals(),
        keyword::keyword(),
        operator::operator(),
        punctuator::punctuator(),
        identifier::identifier(),
        attempt(spaces::terminator()),
        spaces::newline(),
    ));

    let token = (position(), token, position());

    return repeat::sep_end_by(token, spaces::spaces());
}
