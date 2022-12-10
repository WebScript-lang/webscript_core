use super::Token;
use combine::parser::Parser;
use combine::parser::{char, choice, range};
use combine::stream::RangeStream;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Punctuator {
    /// `(`
    LeftParen,
    /// `)`
    RightParen,
    /// `{`
    LeftBrace,
    /// `}`
    RightBrace,
    /// `[`
    LeftBracket,
    /// `]`
    RightBracket,
    /// `,`
    Comma,
    /// `:`
    Colon,
}

pub fn punctuator<'src, I>() -> impl Parser<I, Output = Token> + 'src
where
    I: RangeStream<Token = char, Range = &'src str> + 'src,
{
    return choice::choice((
        range::range("(").map(|_| Token::Punctuator(Punctuator::LeftParen)),
        range::range(")").map(|_| Token::Punctuator(Punctuator::RightParen)),
        range::range("{").map(|_| Token::Punctuator(Punctuator::LeftBrace)),
        range::range("}").map(|_| Token::Punctuator(Punctuator::RightBrace)),
        range::range("[").map(|_| Token::Punctuator(Punctuator::LeftBracket)),
        range::range("]").map(|_| Token::Punctuator(Punctuator::RightBracket)),
        range::range(",").map(|_| Token::Punctuator(Punctuator::Comma)),
        range::range(":").map(|_| Token::Punctuator(Punctuator::Colon)),
    ));
}
