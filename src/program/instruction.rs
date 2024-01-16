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

    pub time_estimation: Option<InstructionTime>,
}

impl Instruction {
    pub fn new(
        op_code: OpCode,
        dest_operand: Option<InstructionOperand>,
        src_operand: Option<InstructionOperand>,
        decoded_string: Option<String>,
        start_byte: usize,
        length: usize,
        time_estimation: Option<InstructionTime>,
    ) -> Self {
        Self {
            op_code,
            dest_operand,
            src_operand,
            decoded_string,
            start_byte,
            length,
            time_estimation,
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
    time_estimation: None,
};
pub static INVALID_ADDRESS: Instruction = Instruction {
    op_code: OpCode::InvalidAddress,
    dest_operand: None,
    src_operand: None,
    decoded_string: None,
    start_byte: 0,
    length: 0,
    time_estimation: None,
};
pub static END_OF_PROGRAM: Instruction = Instruction {
    op_code: OpCode::EndOfProgram,
    dest_operand: None,
    src_operand: None,
    decoded_string: None,
    start_byte: 0,
    length: 0,
    time_estimation: None,
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

#[derive(Clone, Copy)]
pub struct InstructionTime {
    pub cycles_base: usize,
    pub cycles_ea: usize,
}

impl InstructionTime {
    pub fn new(cycles_base: usize, cycles_ea: usize) -> Self {
        Self {
            cycles_base,
            cycles_ea,
        }
    }

    pub fn total_time(&self) -> usize {
        self.cycles_base + self.cycles_ea
    }

    pub fn new_from_estimation(
        op_code: OpCode,
        dest_operand: &InstructionOperand,
        src_operand: &InstructionOperand,
    ) -> Option<Self> {
        use crate::register::word::AX;
        use OperandType::*;

        match op_code {
            OpCode::Invalid => None,
            OpCode::InvalidAddress => None,
            OpCode::EndOfProgram => None,

            OpCode::Add | OpCode::Sub => {
                match (dest_operand.operand_type, src_operand.operand_type) {
                    (REGISTER, REGISTER) => Some(Self::new(3, 0)),
                    (REGISTER, EAC) => Some(Self::new(
                        9,
                        Self::get_cycles_for_ea(&dest_operand, &src_operand),
                    )),
                    (EAC, REGISTER) => Some(Self::new(
                        16,
                        Self::get_cycles_for_ea(&dest_operand, &src_operand),
                    )),
                    (EAC, LITERAL) => Some(Self::new(
                        17,
                        Self::get_cycles_for_ea(&dest_operand, &src_operand),
                    )),
                    (REGISTER, LITERAL) if dest_operand.register.unwrap() == AX => {
                        Some(Self::new(4, 0))
                    }
                    (REGISTER, LITERAL) => Some(Self::new(4, 0)),
                    _ => panic!("Error: Invalid operands for instruction detected while timing."),
                }
            }

            OpCode::Mov => match (dest_operand.operand_type, src_operand.operand_type) {
                (EAC, REGISTER) if src_operand.register.unwrap() == AX => Some(Self::new(10, 0)),
                (REGISTER, EAC) if dest_operand.register.unwrap() == AX => Some(Self::new(10, 0)),
                (REGISTER, REGISTER) => Some(Self::new(2, 0)),
                (REGISTER, EAC) => Some(Self::new(
                    8,
                    Self::get_cycles_for_ea(&dest_operand, &src_operand),
                )),
                (EAC, REGISTER) => Some(Self::new(
                    9,
                    Self::get_cycles_for_ea(&dest_operand, &src_operand),
                )),
                (REGISTER, LITERAL) => Some(Self::new(4, 0)),
                (EAC, LITERAL) => Some(Self::new(
                    10,
                    Self::get_cycles_for_ea(&dest_operand, &src_operand),
                )),
                _ => panic!("Error: Invalid operands for instruction detected while timing."),
            },

            OpCode::Cmp => todo!(),
            OpCode::Jnz => todo!(),
            OpCode::Je => todo!(),
            OpCode::Jl => todo!(),
            OpCode::Jle => todo!(),
            OpCode::Jb => todo!(),
            OpCode::Jbe => todo!(),
            OpCode::Jp => todo!(),
            OpCode::Jo => todo!(),
            OpCode::Js => todo!(),
            OpCode::Jnl => todo!(),
            OpCode::Jg => todo!(),
            OpCode::Jnb => todo!(),
            OpCode::Ja => todo!(),
            OpCode::Jnp => todo!(),
            OpCode::Jno => todo!(),
            OpCode::Jns => todo!(),
            OpCode::Loop => todo!(),
            OpCode::Loopz => todo!(),
            OpCode::Loopnz => todo!(),
            OpCode::Jcxz => todo!(),
        }
    }

    /// Get effective address calculation time for an instruction.
    /// See table 2.20 in the 8086 Family Users Manual.
    fn get_cycles_for_ea(
        dest_operand: &InstructionOperand,
        src_operand: &InstructionOperand,
    ) -> usize {
        // NOTE: probably only one operand should be taken into account
        let dest_cycles = Self::get_operand_ea_cycles(dest_operand);
        let src_cycles = Self::get_operand_ea_cycles(src_operand);

        dest_cycles + src_cycles
    }

    /// Get effective address calculation time for an instruction operand.
    /// See table 2.20 in the 8086 Family Users Manual.
    fn get_operand_ea_cycles(operand: &InstructionOperand) -> usize {
        match operand.operand_type {
            OperandType::REGISTER => 0,
            OperandType::LITERAL => 0,
            OperandType::EAC => {
                use crate::register::word::{BP, BX, DI, SI};

                let has_displacement = operand.eac_displacement.is_some_and(|x| x > 0);
                let has_base_reg = operand.eac_reg_0.is_some();
                let has_index_reg = operand.eac_reg_1.is_some();

                match (has_displacement, has_base_reg, has_index_reg) {
                    (true, false, false) => 6,
                    (false, true, false) | (false, false, true) => 5,
                    (true, true, false) | (true, false, true) => 9,
                    (false, true, true) => {
                        let base_reg = operand.eac_reg_0.unwrap();
                        let index_reg = operand.eac_reg_1.unwrap();
                        match (base_reg, index_reg) {
                            (BP, DI) | (BX, SI) => 7,
                            (BP, SI) | (BX, DI) => 8,
                            _ => {
                                panic!("Error: Invalid register combination for EAC calculation.");
                            }
                        }
                    }
                    (true, true, true) => {
                        let base_reg = operand.eac_reg_0.unwrap();
                        let index_reg = operand.eac_reg_1.unwrap();
                        match (base_reg, index_reg) {
                            (BP, DI) | (BX, SI) => 11,
                            (BP, SI) | (BX, DI) => 12,
                            _ => {
                                panic!("Error: Invalid register combination for EAC calculation.");
                            }
                        }
                    }
                    (false, false, false) => 0,
                }
            }
        }
    }

    pub fn get_string(&self) -> String {
        if self.cycles_ea > 0 {
            String::from(format!(
                "{} ({} + {}ea)",
                self.total_time(),
                self.cycles_base,
                self.cycles_ea
            ))
        } else {
            String::from(format!("{}", self.cycles_base))
        }
    }
}
