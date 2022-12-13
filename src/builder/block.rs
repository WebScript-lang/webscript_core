use binaryen_sys::*;

use super::{cstring, cstring_to_ptr, type_::Type, Expr, Module};

pub fn block(module: &Module, name: Option<String>, children: &[Expr], type_: Type) -> Expr {
    unsafe {
        let mut expr = Vec::with_capacity(children.len());
        for child in children {
            expr.push((*child).into());
        }

        let name = cstring(name);

        BinaryenBlock(
            module.into(),
            cstring_to_ptr(&name),
            expr.as_mut_ptr(),
            expr.len() as u32,
            type_.into(),
        )
        .into()
    }
}
