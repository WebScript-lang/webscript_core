use crate::{
    codegen::{
        codegen,
        module::ModuleEnv,
        utils::{get_type, to_builder_type},
        ExprValue,
    },
    nscript::{
        values::{Class, Function, Integer},
        AnyType, AnyValue, Name,
    },
    parser::{ClassData, Expression},
};

pub fn class(env: &mut ModuleEnv, data: ClassData) -> ExprValue {
    // Convert fields
    let mut fields = Vec::with_capacity(data.fields.len());
    for (name, type_name) in &data.fields {
        fields.push((name.clone(), get_type(&env.state, &type_name)))
    }

    // Convert methods
    // TODO: Create a class scope
    let mut methods = Vec::with_capacity(data.methods.len());
    for (name, fn_) in data.methods {
        let fn_ = codegen(env, fn_);

        methods.push((name, fn_.value.into_function().unwrap()));
    }

    // Create the class
    let scopeName = env.state.current_function_codename();
    let class = Class::new(
        Name::new(data.name, scopeName),
        fields.clone(),
        methods.clone(),
        None,
    );

    // If class is named, add it to the module
    if let Name::Name { name, .. } = class.name() {
        let name = name.clone();
        env.state
            .add(name.clone(), class.clone().into())
            .expect(&format!("Identifier '{name}' already exists"));
    }

    // Return the class
    (env.op.nop(), class.into()).into()
}
