use binaryen_sys::*;

use super::{string_to_ptr, Module};

#[derive(Clone, Copy)]
pub struct Export(BinaryenExportRef);

impl Export {
    pub fn new(module: &Module, name: String, external_name: Option<String>) -> Export {
        let external_name = external_name.or(Some(name.clone()));

        unsafe {
            Export(BinaryenAddExport(
                module.into(),
                string_to_ptr(&Some(name)).as_ptr(),
                string_to_ptr(&external_name).as_ptr(),
            ))
        }
    }
}
