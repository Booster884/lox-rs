use crate::chunk::*;
use crate::compiler::*;
use crate::value::*;

pub enum InterpretResult {
    Ok,
    CompileError,
    RuntimeError,
}

pub enum BinOp {
    Add,
    Subtract,
    Multiply,
    Divide,
}

pub struct VM {
    pub chunk: Chunk,
    pub stack: Vec<Value>,
    pub ip: usize,
}

impl VM {
    pub fn new() -> Self {
        Self {
            chunk: Chunk::new(),
            stack: Vec::new(),
            ip: 0,
        }
    }

    pub fn interpret(&mut self, source: &str) -> InterpretResult {
        if !compile(&mut self.chunk, source) {
            return InterpretResult::CompileError;
        }

        self.ip = 0;
        // self.chunk = chunk;
        self.run()
    }

    fn push(&mut self, value: Value) {
        self.stack.push(value);
    }

    fn pop(&mut self) -> Value {
        self.stack.pop().expect("Empty stack")
    }

    fn binary_op(&mut self, binop: BinOp) {
        let b: Value = self.pop();
        let a: Value = self.pop();
        match binop {
            BinOp::Add => self.push(a + b),
            BinOp::Subtract => self.push(a - b),
            BinOp::Multiply => self.push(a * b),
            BinOp::Divide => self.push(a / b),
        }
    }

    fn run(&mut self) -> InterpretResult {
        loop {
            let instruction: Op = self.chunk.code[self.ip];

            #[cfg(feature = "debug_trace_execution")]
            {
                println!("          {:?}", self.stack);
                crate::debug::disassemble_instr(&self.chunk, self.ip);
            }
            self.ip += 1;

            match instruction {
                Op::Constant(index) => {
                    let value: Value = self.chunk.values[index];
                    self.push(value);
                }
                Op::Negate => {
                    let value: Value = self.pop();
                    self.push(-value);
                }
                Op::Add => {
                    self.binary_op(BinOp::Add);
                }
                Op::Subtract => {
                    self.binary_op(BinOp::Subtract);
                }
                Op::Multiply => {
                    self.binary_op(BinOp::Multiply);
                }
                Op::Divide => {
                    self.binary_op(BinOp::Divide);
                }
                Op::Return => {
                    println!("{}", self.pop());
                    return InterpretResult::Ok;
                }
            }
        }
    }
}
