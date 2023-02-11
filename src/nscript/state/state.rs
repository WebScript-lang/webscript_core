use crate::nscript::{values::Function, AnyValue};

use super::{FunctionScope, Scope};

pub struct State {
    scopes: Vec<Scope>,
    func_trace: Vec<FunctionScope>,
}

impl State {
    pub fn new() -> Self {
        State {
            scopes: vec![Scope::create_global(), Scope::new()],
            func_trace: vec![FunctionScope::create_global()],
        }
    }

    // === Scopes ===

    /// Creates a new scope
    pub fn push_scope(&mut self) {
        self.scopes.push(Scope::new());
    }

    /// Removes the topmost scope
    pub fn pop_scope(&mut self) {
        self.scopes.pop();
    }

    /// Returns current scope
    pub fn scope(&self) -> &Scope {
        self.scopes.last().unwrap()
    }

    /// Returns current scope mutably
    fn scope_mut(&mut self) -> &mut Scope {
        self.scopes.last_mut().unwrap()
    }

    /// Check if current scope is a module scope
    pub fn is_module_scope(&self) -> bool {
        self.scopes.len() == 2
    }

    /// Check if current scope is a global scope
    pub fn is_global_scope(&self) -> bool {
        self.scopes.len() == 1
    }

    // === Global/Module ===

    /// Add a new item to the global scope
    #[must_use]
    pub fn add_global_item(&mut self, name: String, value: AnyValue) -> Option<AnyValue> {
        // Return if global scope contains the item
        if self.scopes[0].contains_key(&name) {
            return None;
        }

        // Add the item to the current scope
        if self.scopes[0].insert(name, value.clone()).is_some() {
            return None;
        }
        Some(value)
    }

    /// Add a new item to the module scope
    #[must_use]
    pub fn add_module_item(&mut self, name: String, value: AnyValue) -> Option<AnyValue> {
        // Return if module scope contains the item
        if self.scopes[1].contains_key(&name) {
            return None;
        }

        // Add the item to the current scope
        if self.scopes[1].insert(name, value.clone()).is_some() {
            return None;
        }
        Some(value)
    }

    /// Set an item in the module scope
    #[must_use]
    pub fn set_module_item(&mut self, name: String, value: AnyValue) -> Option<AnyValue> {
        // Add the item to the current scope
        if self.scopes[1].insert(name, value.clone()).is_some() {
            return None;
        }
        Some(value)
    }

    // === Function trace ===

    /// Add a function to the top of the function trace.
    /// Returns the code name of the function.
    pub fn push_function(&mut self, func: Function) {
        self.func_trace.push(FunctionScope::new(func));
    }

    /// Remove the topmost function from the function trace
    pub fn pop_function(&mut self) {
        self.func_trace.pop();
    }

    /// Get current function data
    pub fn current_function(&self) -> &FunctionScope {
        self.func_trace.last().expect("Function trace is empty")
    }

    /// Get mutable current function data
    pub fn current_function_mut(&mut self) -> &mut FunctionScope {
        self.func_trace.last_mut().expect("Function trace is empty")
    }

    /// Get the code name of the current function
    pub fn current_function_codename(&self) -> Option<String> {
        if self.func_trace.len() == 1 {
            return None;
        } else {
            let func = self.func_trace.last().unwrap();
            return Some(func.func().name().codename());
        }
    }

    // === Get/Add/Set ===

    /// Get an item from any scope
    pub fn get(&self, name: &str) -> Option<AnyValue> {
        // Find the first scope that contains the item
        let value = self
            .scopes
            .iter()
            .rev()
            .find(|scope| scope.contains_key(name))
            .and_then(|scope| scope.get(name));

        // If the item doesn't exist, return None
        if value.is_none() {
            return None;
        }

        let value = value.unwrap();
        Some(value.clone())
    }

    /// Add a new item to the topmost scope
    #[must_use]
    pub fn add(&mut self, name: String, value: AnyValue) -> Option<AnyValue> {
        // Add the item to the current scope
        if self.scope_mut().insert(name, value.clone()).is_some() {
            return None;
        }

        // If the value is local, add it to the function trace
        if value.get_store().is_local() {
            self.current_function_mut().add_local(value.clone());
        }

        Some(value)
    }

    /// Change the value of an item in any scope
    #[must_use]
    pub fn set(&mut self, name: String, value: AnyValue) -> Option<AnyValue> {
        // Find the first scope that contains the item
        let scope = self
            .scopes
            .iter_mut()
            .rev()
            .find(|scope| scope.contains_key(&name));

        // If the item doesn't exist, return none
        if scope.is_none() {
            return None;
        }
        // Set a new value in the same scope
        scope.unwrap().insert(name, value.clone());
        Some(value)
    }

    /// Print the whole state
    pub fn print(&self) {
        println!("=== State ===");
        for scope in &self.scopes {
            for (key, value) in scope.iter() {
                println!("{key} = {value}");
            }
            println!("------------");
        }
    }
}
