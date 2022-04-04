mod chunk;
mod compiler;
mod debug;
mod scanner;
mod value;
mod vm;

use std::env;
use std::fs;
use std::io::{self, Write};

use vm::{LoxError, VM};

fn repl(vm: &mut VM) {
    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        let mut line: String = String::new();
        io::stdin()
            .read_line(&mut line)
            .expect("Error reading line from REPL");
        if line.is_empty() {
            break;
        }
        vm.interpret(&line).ok();
    }
}

fn run_file(vm: &mut VM, path: &str) {
    let code = match fs::read_to_string(path) {
        Ok(content) => content,
        Err(error) => {
            eprintln!("Error reading file {}: {}", path, error);
            std::process::exit(74);
        }
    };
    if let Err(error) = vm.interpret(&code) {
        match error {
            LoxError::CompileError => std::process::exit(65),
            LoxError::RuntimeError => std::process::exit(70),
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut vm: VM = VM::new();

    match args.len() {
        1 => repl(&mut vm),
        2 => run_file(&mut vm, &args[1]),
        _ => {
            eprintln!("Usage: lox-rs [path]");
            std::process::exit(64);
        }
    }
}
