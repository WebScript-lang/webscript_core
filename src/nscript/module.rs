use std::{
    ops::{Deref, DerefMut},
    sync::{RwLock, RwLockReadGuard, RwLockWriteGuard},
};

use super::state::State;

pub struct Module(RwLock<ModuleData>);

pub struct ModuleData {
    path: String,
    state: State,
}

pub type StateRef<'a> = RwLockReadGuard<'a, ModuleData>;
pub type StateMutRef<'a> = RwLockWriteGuard<'a, ModuleData>;

impl Module {
    pub fn new(path: String) -> Self {
        Self(RwLock::new(ModuleData {
            path,
            state: State::new(),
        }))
    }

    pub fn path(&self) -> String {
        self.0.read().unwrap().path.clone()
    }

    pub fn state(&self) -> RwLockReadGuard<ModuleData> {
        self.0.read().unwrap()
    }

    pub fn state_mut(&self) -> RwLockWriteGuard<ModuleData> {
        self.0.write().unwrap()
    }
}

impl Deref for ModuleData {
    type Target = State;

    fn deref(&self) -> &Self::Target {
        &self.state
    }
}

impl DerefMut for ModuleData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.state
    }
}
