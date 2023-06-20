use crate::op_code::op::OpCode;

/// A single decoded instruction
#[derive(Clone)]
pub struct Instruction {
    pub op_code: OpCode,

    pub dest_operand: Option<InstructionOperand>,
    pub src_operand: Option<InstructionOperand>,

    pub decoded_string: Option<String>,

    pub start_byte: usize,
    pub length: usize,
}

impl Instruction {
    pub fn new(
        op_code: OpCode,
        dest_operand: Option<InstructionOperand>,
        src_operand: Option<InstructionOperand>,
        decoded_string: Option<String>,
        start_byte: usize,
        length: usize,
    ) -> Self {
        Self {
            op_code,
            dest_operand,
            src_operand,
            decoded_string,
            start_byte,
            length,
        }
    }
}

pub static INVALID: Instruction = Instruction {
    op_code: OpCode::Invalid,
    dest_operand: None,
    src_operand: None,
    decoded_string: None,
    start_byte: 0,
    length: 0,
};
pub static INVALID_ADDRESS: Instruction = Instruction {
    op_code: OpCode::InvalidAddress,
    dest_operand: None,
    src_operand: None,
    decoded_string: None,
    start_byte: 0,
    length: 0,
};
pub static END_OF_PROGRAM: Instruction = Instruction {
    op_code: OpCode::EndOfProgram,
    dest_operand: None,
    src_operand: None,
    decoded_string: None,
    start_byte: 0,
    length: 0,
};

#[derive(Clone, Copy)]
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

#[derive(Clone, Copy)]
pub enum OperandType {
    REGISTER,
    EAC,
    LITERAL,
}
