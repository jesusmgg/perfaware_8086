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

        let instructions = Vec::<Instruction>::new();
        let instruction_counter = 0;

        Self {
            bytes,
            bytes_len,
            instructions,
            instruction_counter,
        }
    }
}

/// A single decoded instruction
pub struct Instruction {
    pub op_code: OpCode,

    pub dest_operand: InstructionOperand,
    pub src_operand: InstructionOperand,
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
