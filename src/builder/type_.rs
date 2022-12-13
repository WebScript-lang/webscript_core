use binaryen_sys::{BinaryenType, BinaryenTypeCreate, BinaryenTypeInt32, BinaryenTypeNone};

#[derive(Clone, Copy, PartialEq)]
pub enum Type {
    Null,
    Int32,
    Other(BinaryenType),
}

impl Type {
    pub fn from_array(types: &[Type]) -> BinaryenType {
        let mut types = types
            .iter()
            .map(|type_| (*type_).into())
            .collect::<Vec<BinaryenType>>();

        unsafe { BinaryenTypeCreate(types.as_mut_ptr(), types.len() as u32) }
    }
}

impl Into<BinaryenType> for Type {
    fn into(self) -> BinaryenType {
        match self {
            Type::Null => unsafe { BinaryenTypeNone() },
            Type::Int32 => unsafe { BinaryenTypeInt32() },
            Type::Other(type_) => type_,
        }
    }
}

impl From<BinaryenType> for Type {
    fn from(type_: BinaryenType) -> Self {
        unsafe {
            if type_ == BinaryenTypeNone() {
                Type::Null
            } else if type_ == BinaryenTypeInt32() {
                Type::Int32
            } else {
                Type::Other(type_)
            }
        }
    }
}
