use std::{ops::Deref, rc::Rc};

use uuid::Uuid;

use crate::nscript::{AnyType, AnyValue, FnName};

#[derive(Clone)]
pub struct Function(Rc<FunctionData>);

pub struct FunctionData {
    name: FnName,
    args: Vec<(String, AnyType)>,
    return_type: AnyType,
}

impl Function {
    pub fn new(name: Option<String>, args: Vec<(String, AnyType)>, return_type: AnyType) -> Self {
        let name = name.map_or_else(
            || FnName::Anonymous(Uuid::new_v4()),
            |name| FnName::Name(name),
        );

        Self(Rc::new(FunctionData {
            name,
            args,
            return_type,
        }))
    }

    pub fn name(&self) -> &FnName {
        &self.name
    }

    pub fn args(&self) -> &[(String, AnyType)] {
        &self.args
    }

    pub fn return_type(&self) -> AnyType {
        self.return_type.clone()
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
        AnyValue::Fn(self)
    }
}
