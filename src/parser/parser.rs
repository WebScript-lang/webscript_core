use anyhow::{anyhow, Result};
use combine::stream::position::Stream as PositionStream;
use combine::Positioned;
use combine::{
    easy::Errors,
    parser::{repeat, EasyParser},
};

use super::expression;
use super::{tokens::terminator, Expression};
use crate::tokenizer::{self, TokenWithPosition};

pub fn parse(path: &str, code: &str) -> Result<Vec<Expression>> {
    // Tokenizer
    let tokens = tokenizer::parse(&code);
    debug_save_tokens(&tokens, "target/output.tokens");

    // Parser
    match parse_tokens(&tokens) {
        Ok(exprs) => {
            debug_save_ast(&exprs, "target/output.ast");
            Ok(exprs)
        }
        Err(err) => {
            let err = err
                .map_position(|pos| tokens[pos].start)
                .map_token(|token| token.value);

            Err(err).map_err(|err| anyhow!("Failed to parse a file \"{path}\":\n{err:?}"))
        }
    }
}

fn parse_tokens(
    tokens: &[TokenWithPosition],
) -> Result<Vec<Expression>, Errors<TokenWithPosition, &[TokenWithPosition], usize>> {
    let parser = || repeat::sep_end_by(expression(), terminator());
    let expressions = parser().easy_parse(PositionStream::new(tokens));

    match expressions {
        Ok((expressions, stream)) => {
            // Check if there are any tokens left.
            if !stream.input.is_empty() {
                eprintln!("[Parser]");
                eprintln!("Unparsed:");
                for token in stream.input {
                    eprintln!("  {token:?}");
                }
                eprintln!("\nPosition: {}\n", stream.position());
            }

            // Return the expressions.
            Ok(expressions)
        }
        Err(errors) => Err(errors),
    }
}

#[cfg(feature = "debug-save-tokens")]
#[inline]
fn debug_save_tokens(tokens: &[TokenWithPosition], path: &str) {
    use std::fs::File;
    use std::io::Write;

    let mut file = File::create(path).unwrap();
    for token in tokens {
        writeln!(
            file,
            "[{}:{} {}:{}] {}",
            token.start.line, token.start.column, token.end.line, token.end.column, token.value
        )
        .unwrap();
    }
}

#[cfg(feature = "debug-save-ast")]
fn debug_save_ast(ast: &[Expression], path: &str) {
    use std::fs::File;
    use std::io::Write;

    let mut file = File::create(path).unwrap();
    for expr in ast {
        writeln!(file, "{expr:?}").unwrap();
    }
}

#[cfg(not(feature = "debug-save-tokens"))]
#[inline]
fn debug_save_tokens(_: &[TokenWithPosition], _: &str) {}

#[cfg(not(feature = "debug-save-ast"))]
#[inline]
fn debug_save_ast(_: &[Expression], _: &str) {}
