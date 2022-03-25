use crate::chunk::*;
#[cfg(feature = "debug_trace_execution")] 
use crate::debug::*;
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

pub struct VM<'a> {
    pub chunk: &'a Chunk,
    pub stack: Vec<Value>,
    pub ip: usize,
}

impl<'a> VM<'a> {
    pub fn new(chunk: &'a Chunk) -> Self {
        let vm: VM = Self {
            chunk: &chunk,
            stack: Vec::new(),
            ip: 0
        };
        vm
    }

    pub fn interpret(&mut self) -> InterpretResult {
        // self.chunk = &chunk;
        self.ip = 0;
        self.run()
    }

    fn push(&mut self, value: Value) {
        self.stack.push(value);
    }

    fn pop(&mut  self) -> Value {
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
                disassemble_instr(self.chunk, self.ip);
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