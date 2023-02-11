use std::{ops::Deref, rc::Rc};

use uuid::Uuid;

use crate::nscript::{AnyType, AnyValue, Name};

use super::{Store, Value};

#[derive(Debug, Clone)]
pub struct Function(Rc<FunctionData>);

#[derive(Debug)]
pub struct FunctionData {
    name: Name,
    args: Vec<(String, AnyType)>,
    return_type: AnyType,
}

impl Function {
    pub fn new(name: Name, args: Vec<(String, AnyType)>, return_type: AnyType) -> Self {
        Self(Rc::new(FunctionData {
            name,
            args,
            return_type,
        }))
    }

    pub fn name(&self) -> &Name {
        &self.name
    }

    pub fn args(&self) -> &[(String, AnyType)] {
        &self.args
    }

    pub fn return_type(&self) -> AnyType {
        self.return_type.clone()
    }
}

impl Value for Function {
    fn get_store(&self) -> Store {
        Store::Global
    }

    fn get_type(&self) -> AnyType {
        AnyType::Function
    }

    fn satisfy(&self, type_: AnyType) -> bool {
        type_.is_function()
    }
}

impl Deref for Function {
    type Target = Rc<FunctionData>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Into<AnyValue> for Function {
    fn into(self) -> AnyValue {
        AnyValue::Function(self)
    }
}
