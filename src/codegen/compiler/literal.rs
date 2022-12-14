use crate::{
    codegen::{module::ModuleEnv, Value},
    nscript::{values::Integer, AnyValue, Store},
};

pub fn null(env: &mut ModuleEnv) -> Value {
    (env.op.nop(), AnyValue::Null).into()
}

pub fn integer(env: &mut ModuleEnv, value: i32) -> Value {
    (
        env.int32.const_(value),
        Integer::new(0, Store::Temporar).into(),
    )
        .into()
}

pub fn identifier(env: &mut ModuleEnv, name: String) -> Value {
    let value = env
        .state
        .get(&name)
        .expect(&format!("Identifier '{name}' doesn't exist"));

    match value.clone() {
        AnyValue::Null => (env.op.nop(), AnyValue::Null).into(),
        AnyValue::Integer(val) => {
            if val.store == Store::Local {
                (env.int32.get_local(val.index), value).into()
            } else {
                todo!()
            }
        }
        AnyValue::Function(fn_) => (env.op.nop(), fn_.into()).into(),
        AnyValue::Type(type_) => (env.op.nop(), type_.into()).into(),
    }
}
