use crate::{parser::Expression, tokenizer::TokenWithPosition};
use combine::{parser, Stream};

mod op0_highest;
mod op10_assignment;
mod op1_unitary;
mod op2_power;
mod op3_multiplicative;
mod op4_additive;
mod op5_shift;
mod op6_relational;
mod op7_equality;
mod op8_bitwise;
mod op9_logical;

parser! {
    /// ## Any expression that can be used as a value
    ///
    /// ### Operator precedence
    ///
    /// | Category       |         Operator         | Associativity |
    /// |:---------------|:------------------------:|--------------:|
    /// | Unitary        |     `-` `+` `!` `~`      |         Right |
    /// | Power          |           `**`           |         Right |
    /// | Multiplicative |       `*` `/` `%`        |          Left |
    /// | Additive       |         `+` `-`          |          Left |
    /// | Shift          |        `<<` `>>`         |          Left |
    /// | Relational     |    `<` `>` `<=` `=>`     |          Left |
    /// | Equality       |        `==` `!=`         |          Left |
    /// | Bitwise AND    |           `&`            |          Left |
    /// | Bitwise XOR    |           `^`            |          Left |
    /// | Bitwise OR     |           `\|`           |          Left |
    /// | Logical AND    |           `&&`           |          Left |
    /// | Logical OR     |          `\|\|`          |          Left |
    /// | Assignment     | `=` `*=` `/=` `%=` `**=` |         Right |
    /// |                |   `+=` `-=` `&=` `\|=`   |               |
    /// |                |     `>>=` `<<=` `^=`     |               |
    pub fn operation[I]()(I) -> Expression
    where [ I: Stream<Token=TokenWithPosition> ] {


        op9_logical::logical_or_operation()
    }
}
