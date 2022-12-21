use crate::{
    codegen::{codegen, module::ModuleEnv, ExprValue},
    nscript::values::Integer,
    parser::Expression,
};

pub fn add(env: &mut ModuleEnv, a: Expression, b: Expression) -> ExprValue {
    let a = codegen(env, a);
    let b = codegen(env, b);

    if a.value.is_integer() && b.value.is_integer() {
        (env.int32.add(a.expr, b.expr), Integer::new_value().into()).into()
    } else {
        unimplemented!()
    }
}
