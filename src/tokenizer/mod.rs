use combine::parser::{char, repeat};
use combine::parser::{EasyParser, Parser};
use combine::stream::position::Stream;
use combine::stream::RangeStream;
use combine::{attempt, choice};

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
pub use spaces::separator;
pub use tokens::Token;

pub fn parse<'src>(source: &'src str) -> Vec<Token> {
    let tokens = tokenize().easy_parse(Stream::new(source));

    match tokens {
        Ok((tokens, _)) => tokens,
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

fn tokenize<'src, I>() -> impl Parser<I, Output = Vec<Token>> + 'src
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

    return repeat::sep_end_by(token, spaces::spaces());
}
