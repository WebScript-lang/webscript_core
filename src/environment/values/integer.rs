use std::ops::Deref;

use crate::environment::AnyType;

use super::{AnyValue, Store, Value};

#[derive(Debug, Clone)]
pub struct Integer(Store);

impl Integer {
    pub fn new_value() -> Self {
        Self(Store::Value)
    }

    pub fn new_param(index: u32) -> Self {
        Self(Store::Param(index))
    }

    pub fn new_local(index: u32) -> Self {
        Self(Store::Local(index))
    }
}

impl Value for Integer {
    fn get_store(&self) -> Store {
        self.0.clone()
    }

    fn get_type(&self) -> AnyType {
        AnyType::Integer
    }

    fn satisfy(&self, type_: AnyType) -> bool {
        type_.is_integer()
    }
}

impl Into<AnyValue> for Integer {
    fn into(self) -> AnyValue {
        AnyValue::Integer(self)
    }
}

impl Deref for Integer {
    type Target = Store;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
