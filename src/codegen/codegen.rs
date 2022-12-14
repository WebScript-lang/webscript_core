use crate::parser::Expression;

use super::{compiler::*, module::ModuleEnv, Value};

pub fn codegen(env: &mut ModuleEnv, expression: Expression) -> Value {
    match expression {
        Expression::Null => literal::null(env),
        Expression::Integer(value) => literal::integer(env, value),
        Expression::Identifier(name) => literal::identifier(env, name),

        Expression::Add(data) => arithmetic::add(env, data.0, data.1),

        Expression::Function(data) => statement::fn_(env, *data),
        Expression::Call(data) => call::call(env, *data),
        Expression::Return(data) => statement::return_(env, *data),
        _ => unimplemented!("codegen for {expression:?}"),
    }
}
