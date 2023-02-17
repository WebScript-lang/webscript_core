use std::{ops::Deref, rc::Rc};

use crate::environment::{AnyType, Name};

use super::{AnyValue, Function, Store, Value};

#[derive(Debug, Clone)]
pub struct Class(Rc<ClassData>);

#[derive(Debug)]
pub struct ClassData {
    name: Name,
    fields: Vec<(String, AnyType)>,
    methods: Vec<(String, Function)>,
    parent: Option<Box<Class>>,
}

impl Class {
    pub fn new(
        name: Name,
        fields: Vec<(String, AnyType)>,
        methods: Vec<(String, Function)>,
        parent: Option<Box<Class>>,
    ) -> Self {
        Self(Rc::new(ClassData {
            name,
            fields,
            methods,
            parent,
        }))
    }

    pub fn name(&self) -> &Name {
        &self.name
    }

    pub fn fields(&self) -> &[(String, AnyType)] {
        &self.fields
    }

    pub fn methods(&self) -> &[(String, Function)] {
        &self.methods
    }

    pub fn parent(&self) -> Option<&Class> {
        self.parent.as_ref().map(|parent| parent.as_ref())
    }
}

impl Value for Class {
    fn get_store(&self) -> Store {
        Store::Global
    }

    fn get_type(&self) -> AnyType {
        AnyType::Class
    }

    fn satisfy(&self, type_: AnyType) -> bool {
        type_.is_class()
    }
}

impl Into<AnyValue> for Class {
    fn into(self) -> AnyValue {
        AnyValue::Class(self)
    }
}

impl Deref for Class {
    type Target = Rc<ClassData>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
