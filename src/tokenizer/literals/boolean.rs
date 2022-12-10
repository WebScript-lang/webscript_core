use crate::tokenizer::{separator, Token};
use combine::parser::{choice, range, Parser};
use combine::stream::RangeStream;

pub fn boolean<'src, I>() -> impl Parser<I, Output = Token> + 'src
where
    I: RangeStream<Token = char, Range = &'src str> + 'src,
{
    let true_ = range::range("true")
        .with(separator())
        .map(|_| Token::Boolean(true));

    let false_ = range::range("false")
        .with(separator())
        .map(|_| Token::Boolean(false));

    choice::or(true_, false_)
}
