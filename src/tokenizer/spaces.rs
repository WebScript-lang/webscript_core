use combine::parser::{char, combinator, repeat, token, Parser};
use combine::stream::RangeStream;

use super::Token;

pub fn terminator<'src, I>() -> impl Parser<I, Output = Token> + 'src
where
    I: RangeStream<Token = char, Range = &'src str> + 'src,
{
    repeat::skip_many(char::space())
        .skip(char::char(';'))
        .skip(repeat::skip_many(char::space().or(char::char(';'))))
        .map(|_| Token::Terminator)
}

pub fn newline<'src, I>() -> impl Parser<I, Output = Token> + 'src
where
    I: RangeStream<Token = char, Range = &'src str> + 'src,
{
    repeat::skip_many(token::satisfy(|c: char| c != '\n' && c.is_whitespace()))
        .skip(char::char('\n'))
        .skip(char::spaces())
        .map(|_| Token::NewLine)
}

pub fn spaces<'src, I>() -> impl Parser<I, Output = ()> + 'src
where
    I: RangeStream<Token = char, Range = &'src str> + 'src,
{
    repeat::skip_many(token::satisfy(|c: char| c != '\n' && c.is_whitespace()))
}

pub fn separator<'src, I>() -> impl Parser<I, Output = ()> + 'src
where
    I: RangeStream<Token = char, Range = &'src str> + 'src,
{
    combinator::not_followed_by(char::alpha_num())
}

pub fn ignore_spaces<'src, I, P>(parser: P) -> impl Parser<I, Output = P::Output> + 'src
where
    I: RangeStream<Token = char, Range = &'src str> + 'src,
    P: Parser<I> + 'src,
{
    // char::spaces().with(parser).skip(char::spaces())
    parser
}
