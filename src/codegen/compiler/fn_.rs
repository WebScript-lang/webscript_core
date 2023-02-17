use crate::{
    codegen::{
        codegen,
        module::ModuleEnv,
        utils::{get_type, to_builder_type},
        ExprValue,
    },
    environment::{
        values::{Function, Integer},
        AnyType, AnyValue, Name,
    },
    parser::FunctionData,
};

pub fn fn_(env: &mut ModuleEnv, data: FunctionData) -> ExprValue {
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
    let scope_name = env.state.current_function_codename();
    let function = Function::new(
        Name::new(data.name, scope_name),
        args.clone(),
        return_type.clone(),
    );

    // If function is named, add it to the module
    if let Name::Name { name, .. } = function.name() {
        let name = name.clone();
        env.state
            .add(name.clone(), function.clone().into())
            .expect(&format!("Identifier '{name}' already exists"));
    }

    // Open function scope
    env.state.push_function(function.clone());
    env.state.push_scope();

    for (index, (name, type_)) in args.into_iter().enumerate() {
        if type_.is_integer() {
            env.state
                .add(name, Integer::new_param(index as u32).into())
                .unwrap();
        } else {
            unimplemented!();
        }
    }

    // Compile function's body
    let mut body = Vec::with_capacity(data.body.len());
    for expr in data.body {
        body.push(codegen(env, expr).expr);
    }

    // Local variables
    let locals = env.state.current_function().locals();
    let mut builder_locals = Vec::with_capacity(locals.len());
    for local in locals {
        builder_locals.extend_from_slice(&to_builder_type(local));
    }

    // Close function scope
    env.state.pop_scope();
    env.state.pop_function();

    // Add function to a builder module
    env.builder.add_function(
        function.name().codename(),
        &builder_args,
        &to_builder_type(&return_type),
        &builder_locals,
        &body,
    );

    (env.op.nop(), AnyValue::Function(function)).into()
}
