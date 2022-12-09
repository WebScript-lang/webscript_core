use std::collections::HashMap;

use super::{any_value::AnyValue, AnyType};

pub struct State {
    scopes: Vec<HashMap<String, AnyValue>>,
}

impl State {
    pub fn new() -> Self {
        let mut global = HashMap::new();

        // Insert global types
        global.insert("null".into(), AnyValue::Type(AnyType::Null));
        global.insert("Integer".into(), AnyValue::Type(AnyType::Integer));

        State {
            scopes: vec![global, HashMap::new()],
        }
    }

    // Scopes

    /// Creates a new scope on the top of the stack
    pub fn push_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    /// Removes the topmost scope from the stack
    pub fn pop_scope(&mut self) {
        self.scopes.pop();
    }

    /// Returns the topmost scope
    pub fn scope(&self) -> &HashMap<String, AnyValue> {
        self.scopes.last().unwrap()
    }

    /// Returns the topmost scope mutably
    fn scope_mut(&mut self) -> &mut HashMap<String, AnyValue> {
        self.scopes.last_mut().unwrap()
    }

    // Globals

    /// Adds a new global element
    pub fn add_global(&mut self, name: String, value: AnyValue) -> Option<AnyValue> {
        // Return if global scope contains the element
        if self.scopes[0].contains_key(&name) {
            return None;
        }

        // Add the element to the current scope
        if self.scopes[0].insert(name, value.clone()).is_some() {
            return None;
        }
        Some(value)
    }

    // Get/Add/Set

    /// Returns the value of the element from any scope
    pub fn get(&self, name: &str) -> Option<AnyValue> {
        // Find the first scope that contains the element
        let value = self
            .scopes
            .iter()
            .rev()
            .find(|scope| scope.contains_key(name))
            .and_then(|scope| scope.get(name));

        // If the element doesn't exist, return None
        if value.is_none() {
            return None;
        }

        let value = value.unwrap();
        Some(value.clone())
    }

    /// Adds a new element to the topmost scope
    #[must_use]
    pub fn add(&mut self, name: String, value: AnyValue) -> Option<AnyValue> {
        // Add the element to the current scope
        if self.scope_mut().insert(name, value.clone()).is_some() {
            return None;
        }
        Some(value)
    }

    /// Changes the value of the element from any scope
    #[must_use]
    pub fn set(&mut self, name: String, value: AnyValue) -> Option<AnyValue> {
        // Find the first scope that contains the element
        let scope = self
            .scopes
            .iter_mut()
            .rev()
            .find(|scope| scope.contains_key(&name));

        // If the element doesn't exist, return none
        if scope.is_none() {
            return None;
        }
        let scope = scope.unwrap();

        // Set new value in the same scope
        scope.insert(name, value.clone());
        Some(value)
    }

    /// Print the whole state
    pub fn print(&self) {
        println!("---");
        for scope in &self.scopes {
            for (key, value) in scope.iter() {
                println!("{key} = {value}");
            }
            println!("---");
        }
    }
}
