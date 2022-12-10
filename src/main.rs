use std::{env, path::PathBuf};

use codegen::Module;
use nscript::Environment;
use parser::{Expression, FunctionData};

mod builder;
mod codegen;
mod nscript;
mod parser;
mod tokenizer;

fn main() {
    // Enable better panic.
    better_panic::install();

    // Get the path to the file.
    let path = match env::args().nth(1) {
        Some(path) => PathBuf::from(path),
        None => {
            eprintln!("Usage: nscript <path>");
            return;
        }
    };

    // Get the contents of the file.
    let script = match std::fs::read_to_string(path.clone()) {
        Ok(s) => s,
        Err(err) => {
            let path = if path.is_absolute() {
                path
            } else {
                env::current_dir().unwrap().join(&path)
            };

            let path = path.to_str().unwrap();

            eprintln!("Failed to open a file \"{path}\n{err}\"");
            return;
        }
    };

    let tokens = tokenizer::parse(script.as_str());

    println!("{tokens:?}");

    // let ast = Expression::Function(Box::new(FunctionData {
    //     name: Some("add".into()),
    //     args: vec![
    //         ("a".into(), "Integer".into()),
    //         ("b".into(), "Integer".into()),
    //     ],
    //     return_type: Some("Integer".into()),
    //     body: Expression::Return(Box::new(Expression::Add(Box::new((
    //         Expression::Identifier("a".into()),
    //         Expression::Identifier("b".into()),
    //     ))))),
    // }));

    // let env = Environment::new();

    // let wasm = Module::compile(&env, "main.ns".into(), ast);

    // // wasm.optimize();
    // wasm.print();
}
