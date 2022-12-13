use std::{env, path::PathBuf};

use codegen::Module;
use nscript::Environment;

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

    for token in &tokens {
        println!(
            "[{}:{} {}:{}] {}",
            token.start.line, token.start.column, token.end.line, token.end.column, token.value
        );
    }

    println!("\n ----- \n");

    let ast = match parser::parse(&tokens) {
        Ok(exprs) => {
            for expr in &exprs {
                println!("{expr:?}");
            }

            exprs
        }
        Err(err) => {
            let err = err
                .map_position(|pos| tokens[pos].start)
                .map_token(|token| token.value);

            println!("{err:?}");
            panic!()
        }
    };

    println!("\n ----- \n");

    let env = Environment::new();

    let wasm = Module::compile(&env, "main.ns".into(), ast);

    // wasm.optimize();
    wasm.print();
}
