use combine::parser::{char, choice, combinator, range, Parser};
use combine::stream::RangeStream;

use crate::tokenizer::{ignore_spaces, separator, Token};

pub fn number<'src, I>() -> impl Parser<I, Output = Token> + 'src
where
    I: RangeStream<Token = char, Range = &'src str> + 'src,
{
    let take_num1 = || range::take_while1(|c: char| c.is_digit(10));
    let take_num = || range::take_while(|c: char| c.is_digit(10));

    // Parser for the exponent part of a number.
    let exponent = || {
        (
            char::char('e').or(char::char('E')),
            choice::choice((range::range("+"), range::range("-"), range::range(""))),
            take_num1(),
        )
    };

    // Parser for numbers with a decimal point and optional exponent.
    let number1 = choice::or(
        (take_num1(), char::char('.'), take_num()),
        (take_num(), char::char('.'), take_num1()),
    )
    .and(choice::optional(exponent()))
    .map(|((num1, _, num2), exp)| {
        if let Some((_, sign, exp)) = exp {
            // Parse a number with an exponent.
            let num = format!("{}.{}e{}{}", &num1, &num2, &sign, &exp);
            num.parse::<f64>().unwrap()
        } else {
            // Parse a number without an exponent.
            let num = format!("{}.{}", &num1, &num2);
            num.parse::<f64>().unwrap()
        }
    });

    // Parser for numbers without a decimal point, but with an exponent.
    let number2 = (take_num1(), exponent()).map(|(num, (_, sign, exp))| {
        let num = format!("{}e{}{}", &num, &sign, &exp);
        num.parse::<f64>().unwrap()
    });

    ignore_spaces(
        choice::choice((
            combinator::attempt(number1.skip(separator())),
            combinator::attempt(number2.skip(separator())),
        ))
        .map(|num| Token::Number(num)),
    )
}
