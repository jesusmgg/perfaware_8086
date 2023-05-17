use crate::op_code::op::OpCode;

/// A decoded program
pub struct Program {
    bytes: Vec<u8>,
    bytes_len: usize,

    pub instructions: Vec<Instruction>,
    pub instruction_counter: usize,
}

impl Program {
    pub fn new(bytes: Vec<u8>) -> Self {
        let bytes_len = bytes.len();

        let instructions = Vec::<Instruction>::with_capacity(2048);
        let instruction_counter = 0;

        Self {
            bytes,
            bytes_len,
            instructions,
            instruction_counter,
        }
    }

    pub fn push_instruction(&mut self, instruction: Instruction) {
        self.instructions.push(instruction);
    }

    pub fn has_pending_instructions(&self) -> bool {
        self.instruction_counter < self.instructions.len()
    }

    /// Returns next instruction and advances `instruction_counter`.
    pub fn next_instruction(&mut self) -> Option<&Instruction> {
        if self.instruction_counter >= self.instructions.len() {
            println!("Reached end of program instruction list");
            return None;
        } else {
            let instruction = &self.instructions[self.instruction_counter];
            self.instruction_counter += 1;
            return Some(instruction);
        }
    }
}

/// A single decoded instruction
pub struct Instruction {
    pub op_code: OpCode,

    pub dest_operand: Option<InstructionOperand>,
    pub src_operand: Option<InstructionOperand>,
}

impl Instruction {
    pub fn new(
        op_code: OpCode,
        dest_operand: InstructionOperand,
        src_operand: InstructionOperand,
    ) -> Self {
        Self {
            op_code,
            dest_operand: Some(dest_operand),
            src_operand: Some(src_operand),
        }
    }

    pub fn invalid() -> Self {
        Instruction {
            op_code: OpCode::Invalid,
            dest_operand: None,
            src_operand: None,
        }
    }
}

pub struct InstructionOperand {
    pub operand_type: OperandType,
    pub register: Option<u8>,
    pub register_word: Option<bool>,
    pub eac_reg_0: Option<u8>,
    pub eac_reg_1: Option<u8>,
    pub eac_displacement: Option<u16>,
    pub literal: Option<u16>,
}

impl InstructionOperand {
    pub fn new(operand_type: OperandType) -> Self {
        Self {
            operand_type,
            register: None,
            register_word: None,
            eac_reg_0: None,
            eac_reg_1: None,
            eac_displacement: None,
            literal: None,
        }
    }
}

pub enum OperandType {
    REGISTER,
    EAC,
    LITERAL,
}
