use binaryen_sys::*;

use super::*;

pub struct Module(BinaryenModuleRef);

impl Module {
    pub fn new() -> Self {
        unsafe { Self(BinaryenModuleCreate()) }
    }

    pub fn add_function(
        &self,
        name: String,
        args: &[Type],
        results: &[Type],
        locals: &[Type],
        body: &[Expr],
    ) -> Function {
        Function::new(self, name, args, results, locals, body)
    }

    pub fn add_export(&self, name: String, external_name: Option<String>) -> Export {
        Export::new(self, name, external_name)
    }

    pub fn optimize(&self) {
        unsafe { BinaryenModuleOptimize(self.into()) }
    }

    pub fn print(&self) {
        unsafe { BinaryenModulePrint(self.into()) }
    }

    pub fn int32(&self) -> Int32 {
        Int32::new(self)
    }

    pub fn op(&self) -> Op {
        Op::new(self)
    }
}

impl Drop for Module {
    fn drop(&mut self) {
        unsafe { BinaryenModuleDispose(self.0) }
    }
}

impl Into<BinaryenModuleRef> for &Module {
    fn into(self) -> BinaryenModuleRef {
        self.0
    }
}
