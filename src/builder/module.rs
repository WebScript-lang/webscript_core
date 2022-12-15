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

    pub fn add_export(&self, name: String, external_name: Option<String>) {
        let external_name = external_name.or(Some(name.clone()));

        let name = cstring(Some(name));
        let external_name = cstring(external_name);

        unsafe {
            BinaryenAddExport(
                self.into(),
                cstring_to_ptr(&name),
                cstring_to_ptr(&external_name),
            );
        }
    }

    pub fn import_function(
        &self,
        name: String,
        module_name: String,
        external_name: Option<String>,
        params: &[Type],
        results: &[Type],
    ) {
        let external_name = external_name.or(Some(name.clone()));

        let name = cstring(Some(name));
        let module_name = cstring(Some(module_name));
        let external_name = cstring(external_name);

        unsafe {
            BinaryenAddFunctionImport(
                self.into(),
                cstring_to_ptr(&name),
                cstring_to_ptr(&module_name),
                cstring_to_ptr(&external_name),
                Type::from_array(params),
                Type::from_array(results),
            )
        }
    }

    pub fn int32(&self) -> Int32 {
        Int32::new(self)
    }

    pub fn op(&self) -> Op {
        Op::new(self)
    }

    pub fn auto_drop(&self) {
        unsafe { BinaryenModuleAutoDrop(self.into()) }
    }

    pub fn optimize(&self) {
        unsafe { BinaryenModuleOptimize(self.into()) }
    }

    pub fn print(&self) {
        unsafe { BinaryenModulePrint(self.into()) }
    }

    pub fn build(&self) -> Vec<u8> {
        unsafe {
            let result = BinaryenModuleAllocateAndWrite(self.into(), 0 as *const i8);

            Vec::from_raw_parts(
                result.binary as *mut u8,
                result.binaryBytes,
                result.binaryBytes,
            )
        }
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
