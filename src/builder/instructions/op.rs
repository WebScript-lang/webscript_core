use binaryen_sys::*;

use crate::builder::{Expr, Module};

pub struct Op {
    module: BinaryenModuleRef,
}

impl Op {
    pub fn new(module: &Module) -> Self {
        Self {
            module: module.into(),
        }
    }

    pub fn unreachable(&self) -> Expr {
        unsafe { BinaryenUnreachable(self.module).into() }
    }

    pub fn nop(&self) -> Expr {
        unsafe { BinaryenNop(self.module).into() }
    }

    pub fn return_(&self, value: Expr) -> Expr {
        unsafe { BinaryenReturn(self.module, value.into()).into() }
    }
}
