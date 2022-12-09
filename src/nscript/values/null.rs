use crate::nscript::AnyValue;

#[derive(Debug, Clone)]
pub struct Null;

impl Into<AnyValue> for Null {
    fn into(self) -> AnyValue {
        AnyValue::Null
    }
}
