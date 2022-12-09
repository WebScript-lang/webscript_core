use codegen::Module;
use nscript::Environment;
use parser::{Expression, FnData};

mod builder;
mod codegen;
mod nscript;
mod parser;
mod tokenizer;

fn main() {
    let ast = Expression::Fn(Box::new(FnData {
        name: Some("add".into()),
        args: vec![
            ("a".into(), "Integer".into()),
            ("b".into(), "Integer".into()),
        ],
        return_type: Some("Integer".into()),
        body: Expression::Return(Box::new(Expression::Add(Box::new((
            Expression::Identifier("a".into()),
            Expression::Identifier("b".into()),
        ))))),
    }));

    let env = Environment::new();

    let wasm = Module::compile(&env, "main.ns".into(), ast);

    // wasm.optimize();
    wasm.print();
}
