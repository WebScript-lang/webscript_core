use crate::nscript::{values::Function, AnyType, AnyValue, Name};

pub struct FunctionScope {
    func: Function,
    locals: Vec<AnyType>,
    args_num: u32,
}

impl FunctionScope {
    pub fn new(func: Function) -> Self {
        let args_num = func.args().len() as u32;

        Self {
            func,
            locals: Vec::new(),
            args_num,
        }
    }

    /// Create global function scope (main)
    pub fn create_global() -> Self {
        Self::new(Function::new(
            Name::new(Some("main".into()), None),
            Vec::new(),
            AnyType::Null,
        ))
    }

    /// Get function
    pub fn func(&self) -> &Function {
        &self.func
    }

    /// Get local variables
    pub fn locals(&self) -> &Vec<AnyType> {
        &self.locals
    }

    /// Add a local variable
    pub fn add_local(&mut self, local: AnyValue) {
        self.locals.push(local.get_type());
    }

    /// Get the number of arguments
    pub fn args_num(&self) -> u32 {
        self.args_num
    }

    /// Get index of next local variable
    pub fn next_local_index(&self) -> u32 {
        self.args_num + self.locals.len() as u32
    }
}
