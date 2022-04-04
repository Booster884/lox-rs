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
    False,
    True,
    Nil,
    Not,
    Equal,
    NotEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
}

#[derive(Default)]
pub struct Chunk {
    pub code: Vec<Op>,
    pub values: Vec<Value>,
    pub lines: Vec<(u16, u8)>, // Caps source files to 65_535 lines, with max 255 operators per line.
}
impl Chunk {
    pub fn new() -> Self {
        Self {
            code: Vec::new(),
            values: Vec::new(),
            lines: Vec::new(),
        }
    }

    pub fn add_operator(&mut self, operator: Op, line: u16) {
        self.update_lines(line);
        self.code.push(operator);
    }

    pub fn add_constant(&mut self, value: Value, line: u16) -> usize {
        self.update_lines(line);
        self.values.push(value);
        self.values.len() - 1
    }

    // Assumes that lines are added in order
    fn update_lines(&mut self, line: u16) {
        if self.lines.len() == 0 {
            self.lines.push((line, 1));
        } else {
            let highest_index = self.lines.len() - 1;
            let (highest_line, ops) = self.lines[highest_index];
            if line == highest_line {
                self.lines[highest_index] = (highest_line, ops + 1);
            } else {
                self.lines.push((highest_line + 1, 1));
            }
        }
    }

    pub fn get_line(&self, offset: usize) -> u16 {
        let mut offset_copy = offset;
        for i in 0..self.lines.len() {
            let (line, ops) = self.lines[i];
            if offset_copy < ops as usize {
                return line;
            }
            offset_copy -= ops as usize;
        }
        42 // Jank
    }
}
