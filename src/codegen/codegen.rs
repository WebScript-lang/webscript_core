use crate::parser::Expression;

use super::{compiler::*, module::ModuleEnv, ExprValue};

pub fn codegen(env: &mut ModuleEnv, expression: Expression) -> ExprValue {
    match expression {
        Expression::Null => literal::null(env),
        Expression::Integer(value) => literal::integer(env, value),
        Expression::Identifier(name) => literal::identifier(env, name),

        Expression::Add(data) => arithmetic::add(env, data.0, data.1),

        Expression::Var(data) => statement::var(env, *data),

        Expression::Function(data) => fn_::fn_(env, *data),
        Expression::Call(data) => call::call(env, *data),
        Expression::Return(data) => statement::return_(env, *data),
        _ => unimplemented!("codegen for {expression:?}"),
    }
}
