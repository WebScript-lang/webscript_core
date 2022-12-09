use crate::nscript::{AnyValue, Store};

#[derive(Debug, Clone)]
pub struct Integer {
    pub index: u32,
    pub store: Store,
}

impl Into<AnyValue> for Integer {
    fn into(self) -> AnyValue {
        AnyValue::Integer(self)
    }
}
