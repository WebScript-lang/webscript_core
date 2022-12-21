#[derive(Debug, Clone, PartialEq)]
pub enum Store {
    /// Temporar value on safe-stack
    Value,
    /// Function argument
    Param(u32),
    /// Local variable
    Local(u32),
    /// In-memory stack
    Stack(u32),
    /// Memory
    Memory(u32),
    /// Global variable
    Global,
}

impl Store {
    pub fn index(&self) -> u32 {
        match self {
            Store::Value => 0,
            Store::Param(index) => *index,
            Store::Local(index) => *index,
            Store::Stack(index) => *index,
            Store::Memory(index) => *index,
            Store::Global => 0,
        }
    }

    pub fn is_value(&self) -> bool {
        matches!(self, Store::Value)
    }

    pub fn is_param(&self) -> bool {
        matches!(self, Store::Param(_))
    }

    pub fn is_local(&self) -> bool {
        matches!(self, Store::Local(_))
    }

    pub fn is_stack(&self) -> bool {
        matches!(self, Store::Stack(_))
    }

    pub fn is_memory(&self) -> bool {
        matches!(self, Store::Memory(_))
    }

    pub fn is_global(&self) -> bool {
        matches!(self, Store::Global)
    }
}
