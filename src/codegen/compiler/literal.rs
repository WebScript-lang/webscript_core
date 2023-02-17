use crate::{
    codegen::{module::ModuleEnv, ExprValue},
    environment::{values::Integer, AnyValue},
};

pub fn null(env: &mut ModuleEnv) -> ExprValue {
    (env.op.nop(), AnyValue::Null).into()
}

pub fn integer(env: &mut ModuleEnv, value: i32) -> ExprValue {
    (env.int32.const_(value), Integer::new_value().into()).into()
}

pub fn identifier(env: &mut ModuleEnv, name: String) -> ExprValue {
    let value = env
        .state
        .get(&name)
        .expect(&format!("Identifier '{name}' doesn't exist"));

    match value.clone() {
        AnyValue::Null => (env.op.nop(), AnyValue::Null).into(),
        AnyValue::Integer(val) => {
            if val.is_local() || val.is_param() {
                (
                    env.int32.get_local(val.index()),
                    Integer::new_value().into(),
                )
                    .into()
            } else {
                todo!("{val:?}")
            }
        }
        AnyValue::Function(fn_) => (env.op.nop(), fn_.into()).into(),
        AnyValue::Class(class) => (env.op.nop(), class.into()).into(),
        AnyValue::Type(type_) => (env.op.nop(), type_.into()).into(),
    }
}
