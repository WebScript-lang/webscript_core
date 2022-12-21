use crate::nscript::AnyType;

use super::Store;

pub trait Value {
    fn get_store(&self) -> Store;
    fn get_type(&self) -> AnyType;
    fn satisfy(&self, type_: AnyType) -> bool;
}
