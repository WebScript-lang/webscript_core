use crate::{builder::Expr, nscript::AnyValue};

mod codegen;
mod compiler;
mod module;
mod utils;

pub use module::Module;

pub struct Value {
    expr: Expr,
    value: AnyValue,
}

impl From<(Expr, AnyValue)> for Value {
    fn from((expr, value): (Expr, AnyValue)) -> Self {
        Self { expr, value }
    }
}
