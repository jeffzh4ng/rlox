use chunk::{Chunk, OpCode};
use disassembler::disassemble_chunk;

mod chunk;
mod disassembler;

fn main() {
    let mut c = Chunk::new();
    c.code.push(OpCode::OpReturn);

    disassemble_chunk(&c, "test chunk".to_owned());
}
