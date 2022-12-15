use std::{env, fs::File, io::Write, path::PathBuf};

use codegen::Module;
use nscript::Environment;

use anyhow::{anyhow, bail, Result};
use wasmtime::*;

mod builder;
mod codegen;
mod nscript;
mod parser;
mod tokenizer;

fn main() -> Result<()> {
    // Enable better panic.
    better_panic::install();

    // Get the path to the file.
    let path = match env::args().nth(1) {
        Some(path) => PathBuf::from(path),
        None => {
            bail!("Usage: nscript <path>");
        }
    };

    // Get the contents of the file.
    let script = match std::fs::read_to_string(path.clone()) {
        Ok(s) => s,
        Err(err) => {
            let path = if path.is_absolute() {
                path
            } else {
                env::current_dir()?.join(&path)
            };

            let path = path.to_str().unwrap();

            bail!(anyhow!(err).context(format!("Failed to open a file \"{path}\"")));
        }
    };

    // Tokenizer
    let tokens = tokenizer::parse(script.as_str());
    let mut file = File::create("target/output.tokens")?;
    for token in &tokens {
        writeln!(
            file,
            "[{}:{} {}:{}] {}",
            token.start.line, token.start.column, token.end.line, token.end.column, token.value
        )?;
    }

    // Parser
    let mut file = File::create("target/output.ast")?;
    let ast = match parser::parse(&tokens) {
        Ok(exprs) => {
            for expr in &exprs {
                writeln!(file, "{expr:?}")?;
            }

            exprs
        }
        Err(err) => {
            let err = err
                .map_position(|pos| tokens[pos].start)
                .map_token(|token| token.value);

            bail!("{err:?}");
        }
    };

    // Compilation
    let env = Environment::new();
    let wasm = Module::compile(&env, path.to_str().unwrap().into(), ast, true);

    wasm.auto_drop();
    wasm.optimize();
    // wasm.print();
    let output = wasm.build();

    let mut file = File::create("target/output.wasm")?;
    file.write(&output).unwrap();

    // Wasmtime
    let engine = Engine::default();
    let module = wasmtime::Module::new(&engine, output)?;

    let mut store = Store::new(&engine, 4);
    let print = Func::wrap(&mut store, |_: Caller<u32>, param: i32| {
        println!("{param}");
    });

    let instance = Instance::new(&mut store, &module, &[print.into()])?;
    let fn_main = instance.get_typed_func::<(), (), _>(&mut store, "main")?;

    fn_main.call(&mut store, ())?;

    Ok(())
}
