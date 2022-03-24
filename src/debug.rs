use crate::chunk::*;
use crate::value::*;

pub fn disasemble_chunk(chunk: &Chunk, name: &str) {
    println!("== {} ==", name);

    let mut offset: usize = 0;
    while offset < chunk.code.len() {
        offset = disasemble_instr(chunk, offset);
    }
}

fn disasemble_instr(chunk: &Chunk, offset: usize) -> usize {
    print!("{:04} ", offset);
    if offset > 0 && chunk.lines[offset] == chunk.lines[offset - 1] {
        print!("   | ");
    } else {
        print!("{:4} ", get_line(chunk, offset));
    }

    let instruction: Op = chunk.code[offset];
    match instruction {
        Op::Return => {
            return simple_instr("OP_RETURN", offset);
        },
        Op::Constant(index) => {
            return constant_instr("OP_CONSTANT", chunk, index, offset);
        },
    }
}

fn simple_instr(name: &str, offset: usize) -> usize {
    println!("{}", name);
    return offset + 1;
}

fn constant_instr(name: &str, chunk: &Chunk, index: usize, offset: usize) -> usize {
    let constant: Value = chunk.values[index];
    println!("{} {:4} '{}'", name, index, constant);
    return offset + 1;
}