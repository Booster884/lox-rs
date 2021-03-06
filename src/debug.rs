#![allow(dead_code)]
use crate::chunk::*;
use crate::value::*;

pub fn disasemble_chunk(chunk: &Chunk, name: &str) {
    println!("== {} ==", name);

    let mut offset: usize = 0;
    while offset < chunk.code.len() {
        offset = disassemble_instr(chunk, offset);
    }
}

pub fn disassemble_instr(chunk: &Chunk, offset: usize) -> usize {
    print!("{:04} ", offset);
    if offset > 0 && chunk.get_line(offset) == chunk.get_line(offset - 1) {
        print!("   | ");
    } else {
        print!("{:4} ", chunk.get_line(offset));
    }

    let instruction: Op = chunk.code[offset];
    match instruction {
        Op::Constant(index) => return constant_instr("OP_CONSTANT", chunk, index, offset),
        Op::False => return simple_instr("OP_FALSE", offset),
        Op::True => return simple_instr("OP_TRUE", offset),
        Op::Nil => return simple_instr("OP_NIL", offset),
        Op::Negate => return simple_instr("OP_NEGATE", offset),
        Op::Add => return simple_instr("OP_ADD", offset),
        Op::Subtract => return simple_instr("OP_SUBTRACT", offset),
        Op::Multiply => return simple_instr("OP_MULTIPLY", offset),
        Op::Divide => return simple_instr("OP_DIVIDE", offset),
        Op::Not => return simple_instr("OP_NOT", offset),
        Op::Return => return simple_instr("OP_RETURN", offset),
        Op::Equal => return simple_instr("OP_EQUAL", offset),
        Op::NotEqual => return simple_instr("OP_NOT_EQUAL", offset),
        Op::Greater => return simple_instr("OP_GREATER", offset),
        Op::Less => return simple_instr("OP_GREATER_EQUAL", offset),
        Op::LessEqual => return simple_instr("OP_LESS", offset),
        Op::GreaterEqual => return simple_instr("OP_LESS_EQUAL", offset),
    }
}

fn simple_instr(name: &str, offset: usize) -> usize {
    println!("{}", name);
    return offset + 1;
}

fn constant_instr(name: &str, chunk: &Chunk, index: usize, offset: usize) -> usize {
    let constant: Value = chunk.values[index];
    println!("{} {:4} '{:?}'", name, index, constant);
    return offset + 1;
}
