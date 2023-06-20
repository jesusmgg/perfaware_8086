use crate::{
    program::instruction::{InstructionOperand, OperandType},
    register,
};

pub fn get_register_string_and_operand(
    reg_bytes: u8,
    is_word: bool,
) -> Option<(String, InstructionOperand)> {
    let mut operand = InstructionOperand::new(OperandType::REGISTER);
    operand.register = Some(reg_bytes);
    operand.register_word = Some(is_word);

    let reg_string = get_register_string(reg_bytes, is_word);
    Some((reg_string, operand))
}

pub fn get_register_string(reg_bytes: u8, is_word: bool) -> String {
    if is_word {
        register::word::get_str(reg_bytes)
    } else {
        register::byte::get_str(reg_bytes)
    }
}
