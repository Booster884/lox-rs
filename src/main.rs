mod chunk;
use chunk::*;
mod debug;
mod value;
mod vm;
use vm::VM;

fn main() {
    let mut chunk: Chunk = Default::default();

    let index: usize = add_constant(&mut chunk, 1.2, 12);
    add_operator(&mut chunk, Op::Constant(index), 12);

    let index: usize = add_constant(&mut chunk, 3.4, 12);
    add_operator(&mut chunk, Op::Constant(index), 12);

    add_operator(&mut chunk, Op::Add, 12);

    let index: usize = add_constant(&mut chunk, 5.6, 12);
    add_operator(&mut chunk, Op::Constant(index), 12);

    add_operator(&mut chunk, Op::Divide, 12);

    add_operator(&mut chunk, Op::Negate, 12);
    add_operator(&mut chunk, Op::Return, 12);

    let mut vm: VM = VM::new(&mut chunk);
    vm.interpret();
    println!("");
    // debug::disasemble_chunk(&chunk, "test chunk");
}