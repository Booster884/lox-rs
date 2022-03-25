use crate::value::*;

#[derive(Clone, Copy)]
pub enum Op {
    Constant(usize),
    Negate,
    Add,
    Subtract,
    Multiply,
    Divide,
    Return,
}

#[derive(Default)]
pub struct Chunk {
    pub code: Vec<Op>,
    pub values: Vec<Value>,
    pub lines: Vec<(u16, u8)>, // Caps source files to 65_535 lines, with max 255 operators per line.
}

pub fn add_operator(chunk: &mut Chunk, operator: Op, line: u16) {
    update_lines(chunk, line);
    chunk.code.push(operator);
}

pub fn add_constant(chunk: &mut Chunk, value: Value, line: u16) -> usize {
    update_lines(chunk, line);
    chunk.values.push(value);
    chunk.values.len() - 1
}

// Assumes that lines are added in order
fn update_lines(chunk: &mut Chunk, line: u16) {
    if chunk.lines.len() == 0 {
        chunk.lines.push((line, 1));
    } else {
        let highest_index = chunk.lines.len() - 1;
        let (highest_line, ops) = chunk.lines[highest_index];
        if line == highest_line {
            chunk.lines[highest_index] = (highest_line, ops + 1);
        } else {
            chunk.lines.push((highest_line + 1, 1));
        }
    }
}

pub fn get_line(chunk: &Chunk, offset: usize) -> u16 {
    let mut offset_copy = offset;
    for i in 0..chunk.lines.len() {
        let (line, ops) = chunk.lines[i];
        if offset_copy < ops as usize {
            return line;
        }
        offset_copy -= ops as usize;
    }
    42 // Jank
}