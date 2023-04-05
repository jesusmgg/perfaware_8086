use crate::{program::InstructionOperand, register};

pub fn get_register_str_and_operand(
    reg_bytes: u8,
    is_word: bool,
) -> Option<(String, InstructionOperand)> {
    let mut operand = InstructionOperand::new(crate::program::OperandType::REGISTER);
    operand.register = Some(reg_bytes);
    operand.register_word = Some(is_word);
    if is_word {
        Some((register::word::get_str(reg_bytes), operand))
    } else {
        Some((register::byte::get_str(reg_bytes), operand))
    }
}
