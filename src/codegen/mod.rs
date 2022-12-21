use crate::{builder::Expr, nscript::AnyValue};

mod codegen;
mod compiler;
mod memory;
mod module;
mod utils;

pub use codegen::codegen;
pub use memory::Memory;
pub use module::Module;

#[derive(Clone)]
pub struct ExprValue {
    expr: Expr,
    value: AnyValue,
}

impl From<(Expr, AnyValue)> for ExprValue {
    fn from((expr, value): (Expr, AnyValue)) -> Self {
        Self { expr, value }
    }
}
