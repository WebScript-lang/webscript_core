use binaryen_sys::{BinaryenExpressionGetType, BinaryenExpressionRef};

use super::Type;

#[derive(Clone, Copy)]
pub struct Expr(BinaryenExpressionRef);

impl Expr {
    pub fn get_type(&self) -> Type {
        unsafe { BinaryenExpressionGetType(self.0).into() }
    }

    pub fn is_null(&self) -> bool {
        self.get_type() == Type::Null
    }

    pub fn is_int32(&self) -> bool {
        self.get_type() == Type::Int32
    }
}

impl Into<BinaryenExpressionRef> for Expr {
    fn into(self) -> BinaryenExpressionRef {
        self.0
    }
}

impl From<BinaryenExpressionRef> for Expr {
    fn from(ref_: BinaryenExpressionRef) -> Self {
        Expr(ref_)
    }
}
