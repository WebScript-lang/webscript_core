use crate::{nscript::AnyValue, parser::Expression};

use super::{compiler::*, module::ModuleEnv, Value};

fn nop(env: &ModuleEnv) -> Value {
    (env.builder.op().nop(), AnyValue::Null).into()
}

pub fn codegen(env: &mut ModuleEnv, ast: Expression) -> Value {
    match ast {
        Expression::Null => nop(env),
        Expression::Integer(value) => nop(env),

        Expression::Identifier(name) => literal::identifier(env, name),

        Expression::Add(data) => arithmetic::add(env, data.0, data.1),
        Expression::Sub(data) => nop(env),
        Expression::Mul(data) => nop(env),

        Expression::Var(data) => nop(env),

        Expression::Assign(data) => nop(env),

        Expression::Function(data) => statement::fn_(env, *data),
        Expression::Call(data) => nop(env),
        Expression::Return(data) => statement::return_(env, *data),
    }
}
