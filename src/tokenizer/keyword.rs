use combine::parser::{char, choice, combinator, range, sequence, Parser};
use combine::stream::RangeStream;

use super::Token;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Keyword {
    This,
    Typeof,
    Extends,
    Fn,
    Class,
    Type,
    Constructor,
    Var,
    Let,
    Const,
    If,
    Else,
    While,
    For,
    Match,
    Import,
    Export,
    From,

    Return,
    Break,
    Continue,
    Default,
    Async,
    Await,
    Gen,
    Yield,
    In,
    As,
    Is,
    Not,
    Try,
    Catch,
    Finally,
    Throw,
    Static,
    Get,
    Set,
}

fn parse_keyword<'src, I>(
    key: &'static str,
) -> sequence::With<
    range::Range<I>,
    combinator::NotFollowedBy<impl Parser<I, Output = char, PartialState = ()>>,
>
where
    I: RangeStream<Token = char, Range = &'src str>,
{
    range::range(key).with(combinator::not_followed_by(char::alpha_num()))
}

pub fn keyword<'src, I>() -> impl Parser<I, Output = Token> + 'src
where
    I: RangeStream<Token = char, Range = &'src str> + 'src,
{
    let keyword1 = choice::choice((
        parse_keyword("this").map(|_| Token::Keyword(Keyword::This)),
        parse_keyword("typeof").map(|_| Token::Keyword(Keyword::Typeof)),
        parse_keyword("extends").map(|_| Token::Keyword(Keyword::Extends)),
        parse_keyword("fn").map(|_| Token::Keyword(Keyword::Fn)),
        parse_keyword("class").map(|_| Token::Keyword(Keyword::Class)),
        parse_keyword("type").map(|_| Token::Keyword(Keyword::Type)),
        parse_keyword("constructor").map(|_| Token::Keyword(Keyword::Constructor)),
        parse_keyword("let").map(|_| Token::Keyword(Keyword::Let)),
        parse_keyword("var").map(|_| Token::Keyword(Keyword::Var)),
        parse_keyword("const").map(|_| Token::Keyword(Keyword::Const)),
        parse_keyword("if").map(|_| Token::Keyword(Keyword::If)),
        parse_keyword("else").map(|_| Token::Keyword(Keyword::Else)),
        parse_keyword("while").map(|_| Token::Keyword(Keyword::While)),
        parse_keyword("for").map(|_| Token::Keyword(Keyword::For)),
        parse_keyword("match").map(|_| Token::Keyword(Keyword::Match)),
        parse_keyword("import").map(|_| Token::Keyword(Keyword::Import)),
        parse_keyword("export").map(|_| Token::Keyword(Keyword::Export)),
        parse_keyword("from").map(|_| Token::Keyword(Keyword::From)),
        parse_keyword("return").map(|_| Token::Keyword(Keyword::Return)),
    ));

    let keyword2 = choice::choice((
        parse_keyword("break").map(|_| Token::Keyword(Keyword::Break)),
        parse_keyword("continue").map(|_| Token::Keyword(Keyword::Continue)),
        parse_keyword("default").map(|_| Token::Keyword(Keyword::Default)),
        parse_keyword("async").map(|_| Token::Keyword(Keyword::Async)),
        parse_keyword("await").map(|_| Token::Keyword(Keyword::Await)),
        parse_keyword("gen").map(|_| Token::Keyword(Keyword::Gen)),
        parse_keyword("yield").map(|_| Token::Keyword(Keyword::Yield)),
        parse_keyword("in").map(|_| Token::Keyword(Keyword::In)),
        parse_keyword("as").map(|_| Token::Keyword(Keyword::As)),
        parse_keyword("is").map(|_| Token::Keyword(Keyword::Is)),
        parse_keyword("not").map(|_| Token::Keyword(Keyword::Not)),
        parse_keyword("try").map(|_| Token::Keyword(Keyword::Try)),
        parse_keyword("catch").map(|_| Token::Keyword(Keyword::Catch)),
        parse_keyword("finally").map(|_| Token::Keyword(Keyword::Finally)),
        parse_keyword("throw").map(|_| Token::Keyword(Keyword::Throw)),
        parse_keyword("static").map(|_| Token::Keyword(Keyword::Static)),
        parse_keyword("get").map(|_| Token::Keyword(Keyword::Get)),
        parse_keyword("set").map(|_| Token::Keyword(Keyword::Set)),
    ));

    keyword1.or(keyword2)
}
