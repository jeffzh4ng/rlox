use crate::chunk::{Chunk, OpCode};

pub fn disassemble_chunk(chunk: &Chunk, name: String) {
    println!("== {} ==", name);

    for (index, op_code) in chunk.code.iter().enumerate() {
        disassemble_instruction(index, op_code)
    }
}

fn disassemble_instruction(index: usize, instruction: &OpCode) {
    println!(
        // "{:04}  L{:03}  {:?}",
        "{:04}  {:?}",
        index, instruction
    );
}