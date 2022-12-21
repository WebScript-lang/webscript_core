use crate::{
    parser::{tokens::*, AssignData, Expression},
    tokenizer::TokenWithPosition,
};
use combine::{choice, optional, parser, parser::repeat, Stream};

use super::op0_highest::assignable_operation;
use super::op9_logical::logical_or_operation;

parser! {
    /// Assignment operator (=, +=, -=, *=, /=, %=, **=, <<=, >>=, &=, ^=, |=)
    fn assignment_operator[I]()(I) -> Operator
    where [ I: Stream<Token=TokenWithPosition> ] {

        choice((
            operator(Operator::Assign),
            operator(Operator::PlusAssign),
            operator(Operator::MinusAssign),
            operator(Operator::PowerAssign),
            operator(Operator::MultiplyAssign),
            operator(Operator::DivideAssign),
            operator(Operator::ModuloAssign),
            operator(Operator::LeftShiftAssign),
            operator(Operator::RightShiftAssign),
            operator(Operator::BitwiseAndAssign),
            operator(Operator::BitwiseXorAssign),
            operator(Operator::BitwiseOrAssign),
        ))
    }
}

// TODO: allows to chain assignment operations
parser! {
    /// Assignment operations (=, +=, -=, *=, /=, %=, **=, <<=, >>=, &=, ^=, |=)
    pub fn assignment_operation[I]()(I) -> Expression
    where [ I: Stream<Token=TokenWithPosition> ] {

        (
            assignable_operation(),
            assignment_operator(),
            logical_or_operation(), // allows to nest higher-order operations
        ).map(|(ref_, op, value)| {
            match op {
                Operator::Assign => Expression::Assign(Box::new(AssignData { ref_, value })),
                Operator::PlusAssign => Expression::AddAssign(Box::new(AssignData { ref_, value })),
                Operator::MinusAssign => Expression::SubAssign(Box::new(AssignData { ref_, value })),
                Operator::PowerAssign => Expression::MulAssign(Box::new(AssignData { ref_, value })),
                Operator::MultiplyAssign => Expression::DivAssign(Box::new(AssignData { ref_, value })),
                Operator::DivideAssign => Expression::ModuloAssign(Box::new(AssignData { ref_, value })),
                Operator::ModuloAssign => Expression::PowerAssign(Box::new(AssignData { ref_, value })),
                Operator::LeftShiftAssign => Expression::LeftShiftAssign(Box::new(AssignData { ref_, value })),
                Operator::RightShiftAssign => Expression::RightShiftAssign(Box::new(AssignData { ref_, value })),
                Operator::BitwiseAndAssign => Expression::BitwiseAndAssign(Box::new(AssignData { ref_, value })),
                Operator::BitwiseXorAssign => Expression::BitwiseXorAssign(Box::new(AssignData { ref_, value })),
                Operator::BitwiseOrAssign => Expression::BitwiseOrAssign(Box::new(AssignData { ref_, value })),
                _ => unreachable!(),
            }
        })
    }
}

// parser! {
//     /// Assignment operations (=, +=, -=, *=, /=, %=, **=, <<=, >>=, &=, ^=, |=)
//     pub fn assignment_operation[I]()(I) -> Expression
//     where [ I: Stream<Token=TokenWithPosition> ] {

//         enum AssignNode {
//             Leaf(Expression),
//             Node(Expression, Operator, Box<AssignNode>),
//         }

//         repeat::chainl1(
//             assignable_operation().map(|expr| AssignNode::Leaf(expr)), // allow to chain assignable objects
//             assignment_operator().map(|op| move |node, leaf| {
//                 match leaf {
//                     AssignNode::Leaf(val) => AssignNode::Node(val, op, Box::new(node)),
//                     _ => unreachable!(),
//                 }
//             })
//         )
//         .and(optional((
//             assignment_operator(),
//             logical_or_operation(), // allow higher-order operation
//         )))
//         .map(|(nodes, option)| {
//             let mut nodes = match option {
//                 Some((op, value)) => AssignNode::Node(value, op, Box::new(nodes)),
//                 None => nodes,
//             };

//             let mut prev_value;
//             let mut prev_op;

//             match nodes {
//                 AssignNode::Leaf(value) => return value,
//                 AssignNode::Node(value, op, sub_nodes) => {
//                     prev_value = value;
//                     prev_op = op;
//                     nodes = *sub_nodes;
//                 }
//             }

//             loop {
//                 match nodes {
//                     AssignNode::Leaf(value) => return assign(value, prev_op, prev_value),
//                     AssignNode::Node(value, op, sub_nodes) => {
//                         prev_value = assign(value, prev_op, prev_value);
//                         prev_op = op;
//                         nodes = *sub_nodes;
//                     }
//                 };
//             }
//         })
//     }
// }

// fn assign(ref_: Expression, op: Operator, value: Expression) -> Expression {
//     match op {
//         Operator::Assign => Expression::Assign(Box::new(AssignData { ref_, value })),
//         Operator::PlusAssign => Expression::AddAssign(Box::new(AssignData { ref_, value })),
//         Operator::MinusAssign => Expression::SubAssign(Box::new(AssignData { ref_, value })),
//         Operator::PowerAssign => Expression::MulAssign(Box::new(AssignData { ref_, value })),
//         Operator::MultiplyAssign => Expression::DivAssign(Box::new(AssignData { ref_, value })),
//         Operator::DivideAssign => Expression::ModuloAssign(Box::new(AssignData { ref_, value })),
//         Operator::ModuloAssign => Expression::PowerAssign(Box::new(AssignData { ref_, value })),
//         Operator::LeftShiftAssign => {
//             Expression::LeftShiftAssign(Box::new(AssignData { ref_, value }))
//         }
//         Operator::RightShiftAssign => {
//             Expression::RightShiftAssign(Box::new(AssignData { ref_, value }))
//         }
//         Operator::BitwiseAndAssign => {
//             Expression::BitwiseAndAssign(Box::new(AssignData { ref_, value }))
//         }
//         Operator::BitwiseXorAssign => {
//             Expression::BitwiseXorAssign(Box::new(AssignData { ref_, value }))
//         }
//         Operator::BitwiseOrAssign => {
//             Expression::BitwiseOrAssign(Box::new(AssignData { ref_, value }))
//         }
//         _ => unreachable!(),
//     }
// }
