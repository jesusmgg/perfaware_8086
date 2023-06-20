use std::collections::HashMap;

use super::instruction::{self, Instruction};

/// A decoded program
pub struct Program {
    bytes_len: usize,

    /// Decoded instructions.
    /// key: start_byte, value: instruction
    pub instructions: HashMap<usize, Instruction>,
}

impl Program {
    pub fn new(bytes: Vec<u8>) -> Self {
        let bytes_len = bytes.len();

        let instructions = HashMap::<usize, Instruction>::with_capacity(2048);

        Self {
            bytes_len,
            instructions,
        }
    }

    pub fn insert_instruction(&mut self, instruction: Instruction) {
        self.instructions
            .insert(instruction.start_byte, instruction);
    }

    pub fn get_instruction_at_byte(&mut self, byte: usize) -> Option<&Instruction> {
        if byte >= self.bytes_len {
            return Some(&instruction::END_OF_PROGRAM);
        } else if self.instructions.contains_key(&byte) {
            return self.instructions.get(&byte);
        } else {
            return Some(&instruction::INVALID_ADDRESS);
        }
    }
}
