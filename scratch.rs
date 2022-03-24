use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 2 {
        run_file(&args[1]);
    } else {
        eprintln!("Usage: manganate <path>");
        std::process::exit(64);
    }
}

fn run_file(path: &str) {
    let contents = fs::read_to_string(path);
    match contents {
        Ok(x) => run(&x),
        Err(x) => {
            eprintln!("Error reading file: {}", x);
            std::process::exit(66);
        }
    }
}

fn run(source: &str) {
    println!("{}", source);
}

fn error(line: i32, message: &str) {
    report(line, "", message);
}

fn report(line: i32, location: &str, message: &str) {
    eprintln!("[line {}] Error{}: {}", line, location, message);
}