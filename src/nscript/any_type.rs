use std::fmt::{Display, Error, Formatter};

use super::AnyValue;

#[derive(Debug, Clone, PartialEq)]
pub enum AnyType {
    // Value types
    Null,
    Integer,
    // Boolean,
    // Number,

    // Reference Types
    // Object,
    // String,
    Function,
    // Class,
    // Ref(Box<AnyType>),

    // Composite Types
    // Union(Box<(AnyType, AnyType)>),
}

impl AnyType {
    pub fn is_null(&self) -> bool {
        matches!(self, AnyType::Null)
    }

    pub fn is_integer(&self) -> bool {
        matches!(self, AnyType::Integer)
    }

    pub fn is_function(&self) -> bool {
        matches!(self, AnyType::Function)
    }
}

impl Into<AnyValue> for AnyType {
    fn into(self) -> AnyValue {
        AnyValue::Type(self)
    }
}

impl Display for AnyType {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match *self {
            AnyType::Null => write!(f, "null"),
            AnyType::Integer => write!(f, "Integer"),
            // AnyType::Boolean => write!(f, "Boolean"),
            // AnyType::Number => write!(f, "Number"),
            // AnyType::String => write!(f, "String"),
            AnyType::Function => write!(f, "Function"),
        }
    }
}
