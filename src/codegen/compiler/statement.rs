use crate::{
    codegen::{codegen, module::ModuleEnv, utils::get_type, ExprValue, Memory},
    parser::{Expression, LetData, VarData},
};

pub fn return_(env: &mut ModuleEnv, value: Expression) -> ExprValue {
    let value = codegen(env, value);

    (env.op.return_(value.expr), value.value).into()
}

pub fn var(env: &mut ModuleEnv, data: VarData) -> ExprValue {
    let type_ = data.type_.map(|type_| get_type(&env.state, &type_));
    let value = data.value.map(|value| codegen(env, value));

    if let (Some(type_), Some(value)) = (&type_, &value) {
        if value.value.get_type() != *type_ {
            panic!("Incompatible types");
        }
    }

    let type_ = type_
        .or_else(|| value.clone().map(|val| val.value.get_type()))
        .expect("You need to specify a type or initial value");

    let value = value.or_else(|| unimplemented!("default value")).unwrap();

    let value = Memory::add_local(env, value);
    env.state
        .add(data.name.clone(), value.value.clone())
        .expect(&format!("Identifier {} already exists", data.name));

    value
}

pub fn let_(env: &mut ModuleEnv, data: LetData) -> ExprValue {
    let type_ = data.type_.map(|type_| get_type(&env.state, &type_));
    let value = codegen(env, data.value);

    if let Some(type_) = &type_ {
        if value.value.get_type() != *type_ {
            panic!("Incompatible types");
        }
    }

    let type_ = type_
        .or_else(|| Some(value.value.get_type()))
        .expect("You need to specify a type or initial value");

    todo!()
}
