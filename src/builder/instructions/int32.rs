use binaryen_sys::*;

use crate::builder::{Expr, Module, Type};

pub struct Int32 {
    module: BinaryenModuleRef,
}

impl Int32 {
    pub fn new(module: &Module) -> Self {
        Self {
            module: module.into(),
        }
    }

    pub fn get_local(&self, index: u32) -> Expr {
        unsafe { BinaryenLocalGet(self.module, index, Type::Int32.into()).into() }
    }

    pub fn set_local(&self, index: u32, value: Expr) -> Expr {
        unsafe { BinaryenLocalSet(self.module, index, value.into()).into() }
    }

    pub fn const_(&self, value: i32) -> Expr {
        unsafe {
            let value = BinaryenLiteralInt32(value);
            BinaryenConst(self.module, value).into()
        }
    }

    pub fn add(&self, left: Expr, right: Expr) -> Expr {
        unsafe { BinaryenBinary(self.module, BinaryenAddInt32(), left.into(), right.into()).into() }
    }
}
