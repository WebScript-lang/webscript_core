use combine::parser::{char, combinator, repeat, token, Parser};
use combine::stream::RangeStream;

use super::Token;

pub fn identifier<'src, I>() -> impl Parser<I, Output = Token> + 'src
where
    I: RangeStream<Token = char, Range = &'src str> + 'src,
{
    combinator::recognize((
        token::satisfy(|c: char| c.is_alphabetic() || c == '_'),
        repeat::skip_many(token::satisfy(|c: char| c.is_alphanumeric() || c == '_')),
        combinator::not_followed_by(char::alpha_num()),
    ))
    .map(|s| Token::Identifier(s))
}
