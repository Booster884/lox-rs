mod chunk;
mod compiler;
mod debug;
mod scanner;
mod value;
mod vm;

use std::env;
use std::fs;
use std::io::{self, Write};

use chunk::*;
use vm::{InterpretResult, VM};

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
        vm.interpret(&line);
    }
}

fn run_file(vm: &mut VM, path: &str) {
    let result = match fs::read_to_string(path) {
        Ok(content) => vm.interpret(&content),
        Err(error) => {
            eprintln!("Error reading file {}: {}", path, error);
            std::process::exit(74);
        }
    };
    match result {
        InterpretResult::CompileError => std::process::exit(65),
        InterpretResult::RuntimeError => std::process::exit(70),
        InterpretResult::Ok => std::process::exit(0),
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut chunk: Chunk = Chunk::new();
    let mut vm: VM = VM::new(&mut chunk);

    match args.len() {
        1 => repl(&mut vm),
        2 => run_file(&mut vm, &args[1]),
        _ => {
            eprintln!("Usage: manganate [path]");
            std::process::exit(64);
        }
    }
}
