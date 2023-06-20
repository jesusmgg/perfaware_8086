use crate::{
    displacement_mode,
    program::instruction::{InstructionOperand, OperandType},
    register,
};

/// Returns the decoded EAC as an string and InstructionOperand.
pub fn get_eac_string_and_operand(
    rm: u8,
    mode: u8,
    displacement_low: u8,
    displacement_high: u8,
) -> Option<(String, InstructionOperand)> {
    let mut operand = InstructionOperand::new(OperandType::EAC);

    // Get registers
    let eac_str = match rm {
        0b000 => {
            operand.eac_reg_0 = Some(register::word::BX);
            operand.eac_reg_1 = Some(register::word::SI);
            "[BX + SI"
        }
        0b001 => {
            operand.eac_reg_0 = Some(register::word::BX);
            operand.eac_reg_1 = Some(register::word::DI);
            "[BX + DI"
        }
        0b010 => {
            operand.eac_reg_0 = Some(register::word::BP);
            operand.eac_reg_1 = Some(register::word::SI);
            "[BP + SI"
        }
        0b011 => {
            operand.eac_reg_0 = Some(register::word::BP);
            operand.eac_reg_1 = Some(register::word::DI);
            "[BP + DI"
        }
        0b100 => {
            operand.eac_reg_0 = Some(register::word::SI);
            "[SI"
        }
        0b101 => {
            operand.eac_reg_0 = Some(register::word::DI);
            "[DI"
        }
        0b110 if mode == displacement_mode::MEM_0_BIT => "[",
        0b110 => {
            operand.eac_reg_0 = Some(register::word::BP);
            "[BP"
        }
        0b111 => {
            operand.eac_reg_0 = Some(register::word::BX);
            "[BX"
        }
        _ => {
            println!(
                "Error decoding effective address calculation, invalid R/M value: {:#b}",
                rm
            );
            return None;
        }
    };

    let mut eac_string = String::from(eac_str);

    // Get displacement
    match mode {
        displacement_mode::MEM_0_BIT if rm == 0b110 => {
            let disp = displacement_low as u16 + displacement_high as u16 * 256;
            operand.eac_displacement = Some(disp);
            eac_string.push_str(disp.to_string().as_str());
        }
        displacement_mode::MEM_8_BIT => {
            operand.eac_displacement = Some(displacement_low as u16);
            eac_string.push_str(" + ");
            eac_string.push_str(displacement_low.to_string().as_str());
        }
        displacement_mode::MEM_16_BIT => {
            let disp = displacement_low as u16 + displacement_high as u16 * 256;
            operand.eac_displacement = Some(disp);
            eac_string.push_str(" + ");
            eac_string.push_str(disp.to_string().as_str());
        }
        _ => (),
    };

    eac_string.push(']');

    Some((eac_string, operand))
}
