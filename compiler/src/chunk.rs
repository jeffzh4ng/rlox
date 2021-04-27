#[derive(Debug)]
pub enum OpCode {
    OpReturn
}

pub struct Chunk {
    pub code: Vec<OpCode>
}

impl Chunk {
    pub fn new() -> Self {
        Chunk {
            code: Vec::new()
        }
    }
}