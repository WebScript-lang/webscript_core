use std::fs::File;
use std::io::Write;

use codegen::Module;

use anyhow::Result;

mod builder;
mod codegen;
mod environment;
mod parser;
mod tokenizer;

pub use environment::*;

pub fn load_main(url: String, code: String) -> Result<Vec<u8>> {
    // Parsing
    let ast = parser::parse(&url, &code)?;

    // Compilation
    let env = Environment::new();
    let wasm = Module::compile(&env, url, ast, true);

    #[cfg(feature = "debug-print-wasm-unoptimized")]
    {
        wasm.print();
        println!("\n ===== \n");
    }

    // Optimization
    wasm.auto_drop();
    wasm.optimize();

    #[cfg(feature = "debug-print-wasm-optimized")]
    {
        wasm.print();
        println!("\n ===== \n");
    }

    let output = wasm.build();

    // Save the output
    let mut file = File::create("target/output.wasm")?;
    file.write(&output).unwrap();

    Ok(output)
}
