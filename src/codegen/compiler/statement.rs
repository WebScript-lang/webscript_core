use crate::{
    codegen::{
        codegen::codegen,
        module::ModuleEnv,
        utils::{get_type, to_builder_type},
        Value,
    },
    nscript::{
        values::{Function, Integer},
        AnyType, AnyValue, FnName, Store,
    },
    parser::{Expression, FunctionData},
};

pub fn return_(env: &mut ModuleEnv, value: Expression) -> Value {
    let value = codegen(env, value);

    (env.op.return_(value.expr), value.value).into()
}

pub fn fn_(env: &mut ModuleEnv, data: FunctionData) -> Value {
    // Get args types
    let mut args = Vec::with_capacity(data.args.len());
    for (name, type_name) in &data.args {
        args.push((name.clone(), get_type(&env.state, &type_name)))
    }

    // Get return type
    let return_type = if let Some(return_type) = data.return_type {
        get_type(&env.state, &return_type)
    } else {
        AnyType::Null
    };

    // Convert args to builder types
    let mut builder_args = Vec::with_capacity(args.len());
    for (_, arg) in &args {
        builder_args.extend_from_slice(&to_builder_type(&arg));
    }

    // Create the function
    let function = Function::new(data.name, args.clone(), return_type.clone());

    // Get internal name of function
    // TODO: nested functions should include the whole path
    let fn_name = function.name().to_string();

    // If function is named, add it to the module
    if let FnName::Name(name) = function.name() {
        let name = name.clone();
        env.state
            .add(name.clone(), function.clone().into())
            .expect(&format!("Identifier '{name}' already exists"));
    }

    // Create function scope
    env.state.push_scope();
    for (index, (name, type_)) in args.into_iter().enumerate() {
        if type_.is_integer() {
            env.state
                .add(
                    name,
                    Integer {
                        index: index as u32,
                        store: Store::Local,
                    }
                    .into(),
                )
                .unwrap();
        } else {
            unimplemented!();
        }
    }

    // Compile function's body
    let body = codegen(env, data.body);

    // Add function to a builder module
    env.builder.add_function(
        fn_name,
        &builder_args,
        &to_builder_type(&return_type),
        &[],
        body.expr,
    );

    (env.op.nop(), AnyValue::Function(function)).into()
}
