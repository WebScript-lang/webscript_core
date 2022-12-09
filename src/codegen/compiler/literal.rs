use crate::{
    codegen::{module::ModuleEnv, Value},
    nscript::{AnyValue, Store},
};

pub fn identifier(env: &mut ModuleEnv, name: String) -> Value {
    let value = env
        .state
        .get(&name)
        .expect(&format!("Identifier '{name}' doesn't exist"));

    if let AnyValue::Integer(val) = value.clone() {
        if val.store == Store::Local {
            (env.int32.get_local(val.index), value).into()
        } else {
            todo!()
        }
    } else {
        todo!()
    }
}
