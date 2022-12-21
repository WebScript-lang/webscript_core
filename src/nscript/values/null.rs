use crate::nscript::{AnyType, AnyValue};

use super::{Store, Value};

#[derive(Debug, Clone)]
pub struct Null;

impl Value for Null {
    fn get_store(&self) -> Store {
        Store::Value
    }

    fn get_type(&self) -> AnyType {
        AnyType::Null
    }

    fn satisfy(&self, type_: AnyType) -> bool {
        type_.is_null()
    }
}

impl Into<AnyValue> for Null {
    fn into(self) -> AnyValue {
        AnyValue::Null
    }
}
