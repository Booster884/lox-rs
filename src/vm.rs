use crate::chunk::*;
use crate::compiler::*;
use crate::value::*;

pub enum LoxError {
    CompileError,
    RuntimeError,
}

pub enum BinOp {
    Add,
    Subtract,
    Multiply,
    Divide,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
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

    pub fn interpret(&mut self, source: &str) -> Result<(), LoxError> {
        if !compile(&mut self.chunk, source) {
            Err(LoxError::CompileError)
        } else {
            self.ip = 0;
            self.run()
        }
    }

    fn runtime_error(&self, message: &str) -> Result<(), LoxError> {
        eprintln!("{}", message);
        let line: u16 = self.chunk.get_line(self.ip);
        eprintln!("[line {}] in script", line);
        Err(LoxError::RuntimeError)
    }

    fn push(&mut self, value: Value) {
        self.stack.push(value);
    }

    fn pop(&mut self) -> Value {
        self.stack.pop().expect("Empty stack")
    }

    fn peek(&mut self, distance: usize) -> Value {
        return self.stack[self.stack.len() - 1 - distance];
    }

    fn binary_op(&mut self, binop: BinOp) -> Result<(), LoxError> {
        let operands = (self.pop(), self.pop());
        match operands {
            (Value::Number(b), Value::Number(a)) => {
                match binop {
                    BinOp::Add => self.push(Value::Number(a + b)),
                    BinOp::Subtract => self.push(Value::Number(a - b)),
                    BinOp::Multiply => self.push(Value::Number(a * b)),
                    BinOp::Divide => self.push(Value::Number(a / b)),
                    BinOp::Greater => self.push(Value::Boolean(a > b)),
                    BinOp::GreaterEqual => self.push(Value::Boolean(a >= b)),
                    BinOp::Less => self.push(Value::Boolean(a < b)),
                    BinOp::LessEqual => self.push(Value::Boolean(a <= b)),
                }
                Ok(())
            }
            _ => self.runtime_error("Operands must be numbers."),
        }
    }

    fn run(&mut self) -> Result<(), LoxError> {
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
                Op::False => self.push(Value::Boolean(false)),
                Op::True => self.push(Value::Boolean(true)),
                Op::Nil => self.push(Value::Nil),
                Op::Negate => match self.peek(0) {
                    Value::Number(value) => {
                        self.pop();
                        self.push(Value::Number(-value));
                    }
                    _ => return self.runtime_error("Operand must be a number."),
                },
                Op::Add => self.binary_op(BinOp::Add)?,
                Op::Subtract => self.binary_op(BinOp::Subtract)?,
                Op::Multiply => self.binary_op(BinOp::Multiply)?,
                Op::Divide => self.binary_op(BinOp::Divide)?,
                Op::Not => {
                    let value = self.pop().is_falsey();
                    self.push(Value::Boolean(value));
                }
                Op::Equal => {
                    let b: Value = self.pop();
                    let a: Value = self.pop();
                    self.push(Value::Boolean(a == b));
                }
                Op::NotEqual => {
                    let b: Value = self.pop();
                    let a: Value = self.pop();
                    self.push(Value::Boolean(a != b));
                }
                Op::Greater => self.binary_op(BinOp::Greater)?,
                Op::GreaterEqual => self.binary_op(BinOp::GreaterEqual)?,
                Op::Less => self.binary_op(BinOp::Less)?,
                Op::LessEqual => self.binary_op(BinOp::LessEqual)?,
                Op::Return => {
                    println!("{:?}", self.pop());
                    return Ok(());
                }
            }
        }
    }
}
