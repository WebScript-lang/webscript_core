use crate::environment::{values::Integer, AnyType};

use super::{module::ModuleEnv, ExprValue};

pub struct Memory;

impl Memory {
    pub fn set_local(env: &mut ModuleEnv, index: u32, value: ExprValue) -> ExprValue {
        if value.value.is_integer() {
            let expr = env.builder.int32().set_local(index, value.expr);
            (expr, Integer::new_local(index).into()).into()
        } else {
            unimplemented!()
        }
    }

    pub fn get_local(env: &mut ModuleEnv, index: u32, type_: AnyType) -> ExprValue {
        if type_.is_integer() {
            let expr = env.builder.int32().get_local(index);
            (expr, Integer::new_value().into()).into()
        } else {
            unimplemented!()
        }
    }

    pub fn add_local(env: &mut ModuleEnv, value: ExprValue) -> ExprValue {
        let index = env.state.current_function().next_local_index();

        if value.value.is_integer() {
            let expr = env.builder.int32().set_local(index, value.expr);
            (expr, Integer::new_local(index).into()).into()
        } else {
            unimplemented!()
        }
    }
}
