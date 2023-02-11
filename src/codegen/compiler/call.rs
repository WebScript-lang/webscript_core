use crate::{
    builder::Function,
    codegen::{codegen, module::ModuleEnv, utils::to_builder_type, ExprValue},
    parser::CallData,
};

pub fn call(env: &mut ModuleEnv, data: CallData) -> ExprValue {
    let fn_ = codegen(env, data.ref_).value;

    let fn_ = fn_
        .into_function()
        .expect("Attempt to call a not callable object");

    if data.args.len() != fn_.args().len() {
        panic!("Incorrect number of arguments");
    }

    let mut args = Vec::with_capacity(data.args.len());
    for (arg, (_, type_)) in data.args.into_iter().zip(fn_.args()) {
        let value = codegen(env, arg);
        if value.value.get_type() != *type_ {
            panic!(
                "Expected value of type '{}', found '{}'",
                type_,
                value.value.get_type()
            )
        }
        args.push(value.expr);
    }

    let return_type = to_builder_type(&fn_.return_type());

    let expr = Function::call(&env.builder, fn_.name().codename(), &args, &return_type);

    (expr, fn_.return_type().default_value()).into()
}
