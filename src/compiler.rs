use crate::scanner::*;

pub fn compile(source: &str) {
    let mut scanner: Scanner = Scanner::new(source);
    loop {
        let token: Token = scanner.scan_token();
        print!("{:4} ", token.line);
        println!("{:?} '{}'", token.kind, token.lexeme);
    }
}
