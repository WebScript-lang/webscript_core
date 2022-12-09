use std::fmt::{Display, Error, Formatter};

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
    Fn,
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
        matches!(self, AnyType::Fn)
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
            AnyType::Fn => write!(f, "Function"),
        }
    }
}
