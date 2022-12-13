use std::ffi::CString;

use binaryen_sys::*;

use super::{block::block, cstring, cstring_to_ptr, type_::Type, Expr, Module};

#[derive(Clone, Copy)]
pub struct Function(BinaryenFunctionRef);

impl Function {
    pub fn new(
        module: &Module,
        name: String,
        params: &[Type],
        results: &[Type],
        locals: &[Type],
        body: &[Expr],
    ) -> Self {
        let mut locals = locals
            .iter()
            .map(|type_| (*type_).into())
            .collect::<Vec<BinaryenType>>();

        let results = Type::from_array(&results);
        let body = block(module, None, body, results.into());
        let name = cstring(Some(name));

        unsafe {
            Function(BinaryenAddFunction(
                module.into(),
                cstring_to_ptr(&name),
                Type::from_array(&params),
                results,
                locals.as_mut_ptr(),
                locals.len() as u32,
                body.into(),
            ))
        }
    }
}
