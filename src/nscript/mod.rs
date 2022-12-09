pub mod environment;
pub mod module;

mod any_type;
mod any_value;
mod fn_name;
mod state;

pub use any_type::AnyType;
pub use any_value::AnyValue;
pub use environment::Environment;
pub use fn_name::FnName;
pub use module::Module;

pub mod values;

#[derive(Debug, Clone, PartialEq)]
pub enum Store {
    Temporar,
    Local,
    Stack,
    Memory,
}
