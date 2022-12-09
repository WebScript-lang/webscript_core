use std::ffi::CString;

mod export;
mod expr;
mod function;
mod module;
mod type_;

mod instructions;

pub use export::Export;
pub use expr::Expr;
pub use function::Function;
pub use module::Module;
pub use type_::Type;

pub use instructions::int32::Int32;
pub use instructions::op::Op;

fn string_to_ptr(string: &Option<String>) -> CString {
    match string {
        Some(name) => CString::new(name.as_str()).unwrap(),
        None => CString::default(),
    }
}
