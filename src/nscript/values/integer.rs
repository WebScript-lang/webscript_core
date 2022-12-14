use crate::nscript::{AnyValue, Store};

#[derive(Debug, Clone)]
pub struct Integer {
    pub index: u32,
    pub store: Store,
}

impl Integer {
    pub fn new(index: u32, store: Store) -> Self {
        Self { index, store }
    }

    pub fn temporar() -> Self {
        Self {
            index: 0,
            store: Store::Temporar,
        }
    }
}

impl Into<AnyValue> for Integer {
    fn into(self) -> AnyValue {
        AnyValue::Integer(self)
    }
}
