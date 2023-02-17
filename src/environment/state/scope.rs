use std::{
    collections::HashMap,
    ops::{Deref, DerefMut},
};

use crate::environment::{values::Function, AnyType, AnyValue, Name};

pub struct Scope(HashMap<String, AnyValue>);

impl Scope {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn create_global() -> Self {
        let mut global = HashMap::new();

        // Build-in types
        global.insert("null".into(), AnyValue::Type(AnyType::Null));
        global.insert("Integer".into(), AnyValue::Type(AnyType::Integer));
        global.insert("Function".into(), AnyValue::Type(AnyType::Function));

        // fn print(value: Integer)
        global.insert(
            "print".into(),
            Function::new(
                Name::new(Some("print".into()), None),
                vec![("value".into(), AnyType::Integer)],
                AnyType::Null,
            )
            .into(),
        );

        Scope(global)
    }
}

impl Deref for Scope {
    type Target = HashMap<String, AnyValue>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Scope {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
