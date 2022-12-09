use crate::{
    builder,
    nscript::{self, environment::Environment, module::StateMutRef},
    parser::Expression,
};

use super::*;

pub struct Module(builder::Module);

pub struct ModuleEnv<'a> {
    pub builder: builder::Module,
    pub state: StateMutRef<'a>,
    pub op: builder::Op,
    pub int32: builder::Int32,
}

impl Module {
    pub fn compile(env: &Environment, path: String, ast: Expression) -> Self {
        let builder = builder::Module::new();
        let module = nscript::Module::new(path);

        let builder = {
            let state = module.state_mut();

            let mut module_env = ModuleEnv {
                state,
                op: builder.op(),
                int32: builder.int32(),
                builder,
            };

            codegen::codegen(&mut module_env, ast);

            module_env.builder
        };

        env.add_module(module);

        Self(builder)
    }

    pub fn optimize(&self) {
        self.0.optimize()
    }

    pub fn print(&self) {
        self.0.print()
    }
}
