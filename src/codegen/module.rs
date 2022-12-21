use crate::{
    builder::{self, Type},
    nscript::{self, environment::Environment, module::StateMutRef},
    parser::Expression,
};

use super::{utils::to_builder_type, *};

pub struct Module(builder::Module);

pub struct ModuleEnv<'a> {
    pub builder: builder::Module,
    pub state: StateMutRef<'a>,
    pub op: builder::Op,
    pub int32: builder::Int32,
}

impl Module {
    pub fn compile(
        env: &Environment,
        path: String,
        expressions: Vec<Expression>,
        is_main: bool,
    ) -> Self {
        let builder = builder::Module::new();
        let module = nscript::Module::new(path);

        let builder = {
            let state = module.state_mut();

            // Create module environment
            let mut module_env = ModuleEnv {
                state,
                op: builder.op(),
                int32: builder.int32(),
                builder,
            };

            // Execute global instructions
            let mut body = Vec::with_capacity(expressions.len());
            for expr in expressions {
                body.push(codegen::codegen(&mut module_env, expr).expr);
            }

            // Add main function to the builder module
            if is_main {
                let locals = module_env.state.current_function().locals();
                let mut main_locals = Vec::with_capacity(locals.len());
                for local in locals {
                    main_locals.extend_from_slice(&to_builder_type(&local));
                }

                module_env
                    .builder
                    .add_function("main".into(), &[], &[], &main_locals, &body);

                module_env.builder.add_export("main".into(), None);

                module_env.builder.import_function(
                    "print".into(),
                    "global".into(),
                    None,
                    &[Type::Int32],
                    &[],
                );
            }

            module_env.builder
        };

        env.add_module(module);

        Self(builder)
    }

    pub fn auto_drop(&self) {
        self.0.auto_drop()
    }

    pub fn optimize(&self) {
        self.0.optimize()
    }

    pub fn print(&self) {
        self.0.print()
    }

    pub fn build(&self) -> Vec<u8> {
        self.0.build()
    }
}
