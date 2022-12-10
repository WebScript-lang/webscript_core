use combine::{choice, Parser, RangeStream};

use super::Token;

mod boolean;
mod integer;
mod null;
mod number;
mod string;

pub fn literals<'src, I>() -> impl Parser<I, Output = Token> + 'src
where
    I: RangeStream<Token = char, Range = &'src str> + 'src,
{
    choice((
        number::number(),
        integer::integer(),
        boolean::boolean(),
        string::string(),
        null::null(),
    ))
}
