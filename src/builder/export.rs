use binaryen_sys::*;

use super::{cstring, cstring_to_ptr, Module};

#[derive(Clone, Copy)]
pub struct Export(BinaryenExportRef);

impl Export {
    pub fn new(module: &Module, name: String, external_name: Option<String>) -> Export {
        let external_name = external_name.or(Some(name.clone()));

        let name = cstring(Some(name));
        let external_name = cstring(external_name);

        unsafe {
            Export(BinaryenAddExport(
                module.into(),
                cstring_to_ptr(&name),
                cstring_to_ptr(&external_name),
            ))
        }
    }
}
