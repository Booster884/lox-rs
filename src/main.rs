mod chunk;
use chunk::*;
mod debug;
mod value;

fn main() {
    let mut chunk = Default::default();

    add_operator(&mut chunk, Op::Return, 12);
    let index: usize = add_constant(&mut chunk, 32.1, 12);
    add_operator(&mut chunk, Op::Constant(index), 12);

    debug::disasemble_chunk(&chunk, "test chunk");
}