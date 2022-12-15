use std::ffi::CString;
use std::os::raw::c_char;

mod block;
mod expr;
mod function;
mod module;
mod type_;

mod instructions;

pub use expr::Expr;
pub use function::Function;
pub use module::Module;
pub use type_::Type;

pub use instructions::int32::Int32;
pub use instructions::op::Op;

fn cstring(string: Option<String>) -> Option<CString> {
    string.map(|string| CString::new(string).unwrap())
}

fn cstring_to_ptr(string: &Option<CString>) -> *const c_char {
    match string {
        Some(string) => string.as_ptr(),
        None => 0 as *const c_char,
    }
}
