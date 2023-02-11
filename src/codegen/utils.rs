use crate::{
    builder::Type,
    nscript::{module::StateMutRef, AnyType},
};

/// Get `AnyType` from string
pub fn get_type(state: &StateMutRef, type_name: &str) -> AnyType {
    state
        .get(type_name)
        .expect(&format!("Undefined type: '{type_name}'"))
        .into_type()
        .expect(&format!("Identifier '{type_name}' is not a type"))
}

/// Convert `AnyType` to `builder::Type`
pub fn to_builder_type(type_: &AnyType) -> Vec<Type> {
    match type_ {
        AnyType::Null => vec![],
        AnyType::Integer => vec![Type::Int32],
        AnyType::Function => vec![],
        AnyType::Class => vec![],
    }
}
