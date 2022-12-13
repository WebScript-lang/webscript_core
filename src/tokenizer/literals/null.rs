use combine::parser::{range, Parser};
use combine::stream::RangeStream;

use crate::tokenizer::{ignore_spaces, separator, Token};

pub fn null<'src, I>() -> impl Parser<I, Output = Token> + 'src
where
    I: RangeStream<Token = char, Range = &'src str> + 'src,
{
    ignore_spaces(range::range("null").with(separator()).map(|_| Token::Null))
}
