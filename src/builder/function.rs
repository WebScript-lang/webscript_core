use binaryen_sys::*;

use super::{string_to_ptr, type_::Type, Expr, Module};

#[derive(Clone, Copy)]
pub struct Function(BinaryenFunctionRef);

impl Function {
    pub fn new(
        module: &Module,
        name: String,
        params: &[Type],
        results: &[Type],
        locals: &[Type],
        body: Expr,
    ) -> Self {
        let mut locals = locals
            .iter()
            .map(|type_| (*type_).into())
            .collect::<Vec<BinaryenType>>();

        unsafe {
            Function(BinaryenAddFunction(
                module.into(),
                string_to_ptr(&Some(name)).as_ptr(),
                Type::from_array(&params),
                Type::from_array(&results),
                locals.as_mut_ptr(),
                locals.len() as u32,
                body.into(),
            ))
        }
    }
}
