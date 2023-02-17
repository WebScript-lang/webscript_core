use crate::{
    parser::{tokens::*, ClassData, Expression},
    tokenizer::TokenWithPosition,
};
use combine::{parser, parser::repeat, Stream};

parser! {
    /// Syntax:
    /// ```
    /// class <identifier> { <field_or_method>* }
    /// field ::= <identifier> : <type> ;
    /// method ::= fn <identifier> ( <param>* ) [-> <type>] { <expression>* }
    /// field_or_method ::= field | method
    /// ```
    pub fn class[I]()(I) -> Expression
    where [ I: Stream<Token=TokenWithPosition> ] {

        keyword(Keyword::Class).with(
            ignore_newlines!(
                identifier(), // name
                punctuator(Punctuator::LeftBrace),
                repeat::sep_end_by( // fields
                    ignore_newlines!(
                        identifier(), // field name
                        punctuator(Punctuator::Colon),
                        type_(), // field type
                    ).map(|(name, _, type_)| (name, type_)),
                    terminator(),
                ),
                punctuator(Punctuator::RightBrace),
            )
        )

        .map(|(name, _, fields, _)| Expression::Class(Box::new(ClassData {
            name: Some(name),
            fields,
            methods: vec![],
            parent: None,
        })))
    }
}
