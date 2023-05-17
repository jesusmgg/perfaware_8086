use std::fs;

use crate::{
    displacement_mode, effective_address_calculation,
    op_code::{self, op::OpCode},
    program::{Instruction, InstructionOperand, OperandType, Program},
    register::{self, util::get_register_str_and_operand},
};

/// Decodes an asm file and returns a `Program` with the decoded instructions.
pub fn decode(file_name: &str) -> Result<Program, ()> {
    println!("Decoder started with {}", file_name);

    let bytes = &fs::read(file_name).unwrap();
    let bytes_len = bytes.len();
    let mut cur_byte: usize = 0;

    let mut program = Program::new(bytes.clone());

    let mut output: String = Default::default();

    output.push_str("bits 16\n\n");

    while cur_byte < bytes_len {
        let b = bytes[cur_byte]; // Current byte
        let mut instruction_length: usize;
        let mut decoded_string: String;
        let mut instruction: Instruction;

        // Instruction width 4
        (instruction_length, decoded_string, instruction) = match (b & 0b1111_0000) >> 4 {
            op_code::width_4::MOV_IMMEDIATE_REG => decode_mov_immediate_reg(bytes, cur_byte),
            _ => (0, String::from(""), Instruction::invalid()),
        };

        // Instruction width 6
        if instruction_length == 0 {
            (instruction_length, decoded_string) = match (b & 0b1111_1100) >> 2 {
                op_code::width_6::MOV_REG_MEM_REG => {
                    decode_reg_mem_reg(OpCode::Mov, bytes, cur_byte)
                }
                op_code::width_6::ADD_REG_MEM_REG => {
                    decode_reg_mem_reg(OpCode::Add, bytes, cur_byte)
                }
                op_code::width_6::SUB_REG_MEM_REG => {
                    decode_reg_mem_reg(OpCode::Sub, bytes, cur_byte)
                }
                op_code::width_6::CMP_REG_MEM_REG => {
                    decode_reg_mem_reg(OpCode::Cmp, bytes, cur_byte)
                }
                op_code::width_6::IMMEDIATE_REG_MEM => decode_immediate_reg_mem(bytes, cur_byte),
                _ => (0, String::from("")),
            };
        }

        // Instruction width 7
        if instruction_length == 0 {
            (instruction_length, decoded_string) = match (b & 0b1111_1110) >> 1 {
                op_code::width_7::MOV_IMMEDIATE_REG_MEM => {
                    decode_immediate_reg_mem(bytes, cur_byte)
                }
                op_code::width_7::MOV_MEM_ACC => {
                    decode_mem_acc(OpCode::Mov, bytes, cur_byte, false)
                }
                op_code::width_7::MOV_ACC_MEM => decode_mem_acc(OpCode::Mov, bytes, cur_byte, true),
                op_code::width_7::ADD_IMMEDIATE_ACC => {
                    decode_mem_acc(OpCode::Add, bytes, cur_byte, false)
                }
                op_code::width_7::SUB_IMMEDIATE_ACC => {
                    decode_mem_acc(OpCode::Sub, bytes, cur_byte, false)
                }
                op_code::width_7::CMP_IMMEDIATE_ACC => {
                    decode_mem_acc(OpCode::Cmp, bytes, cur_byte, false)
                }
                _ => (0, String::from("")),
            };
        }

        // Instruction width 8
        if instruction_length == 0 {
            (instruction_length, decoded_string) = match b {
                op_code::width_8::JNZ => decode_ip_inc_8(OpCode::Jnz, bytes, cur_byte),
                op_code::width_8::JE => decode_ip_inc_8(OpCode::Je, bytes, cur_byte),
                op_code::width_8::JL => decode_ip_inc_8(OpCode::Jl, bytes, cur_byte),
                op_code::width_8::JLE => decode_ip_inc_8(OpCode::Jle, bytes, cur_byte),
                op_code::width_8::JB => decode_ip_inc_8(OpCode::Jb, bytes, cur_byte),
                op_code::width_8::JBE => decode_ip_inc_8(OpCode::Jbe, bytes, cur_byte),
                op_code::width_8::JP => decode_ip_inc_8(OpCode::Jp, bytes, cur_byte),
                op_code::width_8::JO => decode_ip_inc_8(OpCode::Jo, bytes, cur_byte),
                op_code::width_8::JS => decode_ip_inc_8(OpCode::Js, bytes, cur_byte),
                op_code::width_8::JNL => decode_ip_inc_8(OpCode::Jnl, bytes, cur_byte),
                op_code::width_8::JG => decode_ip_inc_8(OpCode::Jg, bytes, cur_byte),
                op_code::width_8::JNB => decode_ip_inc_8(OpCode::Jnb, bytes, cur_byte),
                op_code::width_8::JA => decode_ip_inc_8(OpCode::Ja, bytes, cur_byte),
                op_code::width_8::JNP => decode_ip_inc_8(OpCode::Jnp, bytes, cur_byte),
                op_code::width_8::JNO => decode_ip_inc_8(OpCode::Jno, bytes, cur_byte),
                op_code::width_8::JNS => decode_ip_inc_8(OpCode::Jns, bytes, cur_byte),
                op_code::width_8::LOOP => decode_ip_inc_8(OpCode::Loop, bytes, cur_byte),
                op_code::width_8::LOOPZ => decode_ip_inc_8(OpCode::Loopz, bytes, cur_byte),
                op_code::width_8::LOOPNZ => decode_ip_inc_8(OpCode::Loopnz, bytes, cur_byte),
                op_code::width_8::JCXZ => decode_ip_inc_8(OpCode::Jcxz, bytes, cur_byte),
                _ => (0, String::from("")),
            };
        }

        if instruction_length == 0 {
            eprintln!("Error: Instruction not handled (byte: {:#b})", b);
            break;
        }

        // // Debug print
        // for i in current..current + instruction_length {
        //     print!("{:08b} ", bytes[i]);
        // }
        // println!("  =>  {}", &decoded);

        output.push_str(&decoded_string);
        cur_byte += instruction_length;

        program.push_instruction(instruction);
    }

    println!("{}", &output);

    Ok(program)
}

/// Decodes MOV/ADD/SUB/CMP instruction from register/memory to/from/with register.
/// Returns instruction length in bytes and output decoded string.
fn decode_reg_mem_reg(op: OpCode, bytes: &[u8], current: usize) -> (usize, String) {
    let mut output: String = String::from("");
    let op_str = op_code::strings::get_str(op);

    let mut length: usize = 1;
    let mut b = bytes[current];

    let direction: bool = b & (1 << 1) != 0;
    let word: bool = b & (1 << 0) != 0;

    b = bytes[current + length];
    length += 1;

    let mode = (b & 0b1100_0000) >> 6;

    let reg = (b & 0b0011_1000) >> 3;
    let rm = b & 0b0000_0111;

    let (reg_str, reg_operand) = get_register_str_and_operand(reg, word).unwrap();

    let (rm_str, rm_operand) = match mode {
        displacement_mode::REGISTER => get_register_str_and_operand(rm, word).unwrap(),
        displacement_mode::MEM_8_BIT => {
            b = bytes[current + length];
            length += 1;
            effective_address_calculation::get_eac_str_and_operand(rm, mode, b, 0).unwrap()
        }
        displacement_mode::MEM_16_BIT => {
            b = bytes[current + length];
            let disp_hi = bytes[current + length + 1];
            length += 2;
            effective_address_calculation::get_eac_str_and_operand(rm, mode, b, disp_hi).unwrap()
        }
        displacement_mode::MEM_0_BIT if rm == 0b110 => {
            b = bytes[current + length];
            let disp_hi = bytes[current + length + 1];
            length += 2;
            effective_address_calculation::get_eac_str_and_operand(rm, mode, b, disp_hi).unwrap()
        }
        displacement_mode::MEM_0_BIT => {
            effective_address_calculation::get_eac_str_and_operand(rm, mode, 0, 0).unwrap()
        }
        _ => {
            println!("Invalid MOV mode: {:#b}", mode);
            return (length, output);
        }
    };

    // direction == 1 => reg is destination
    let (destination_str, source_str) = if direction {
        (&reg_str, &rm_str)
    } else {
        (&rm_str, &reg_str)
    };

    output_fmt_op_dest_source(&mut output, op_str, destination_str, source_str);
    (length, output)
}

/// Decodes MOV immediate to register instruction.
/// Returns instruction length in bytes and output decoded string.
fn decode_mov_immediate_reg(bytes: &[u8], current: usize) -> (usize, String, Instruction) {
    let mut output: String = String::from("");
    let op_code = OpCode::Mov;
    let op_str = op_code::strings::get_str(op_code);

    let mut length: usize = 1;
    let mut b = bytes[current];

    let word: bool = b & (1 << 3) != 0;
    let reg = b & 0b0000_0111;
    let (reg_str, reg_operand) = get_register_str_and_operand(reg, word).unwrap();

    b = bytes[current + length];
    length += 1;

    let mut data: u16 = b as u16;

    if word {
        b = bytes[current + length];
        length += 1;
        data += b as u16 * 256;
    }

    output_fmt_op_dest_source(&mut output, op_str, &reg_str, &data.to_string());

    let mut src_operand = InstructionOperand::new(OperandType::LITERAL);
    src_operand.literal = Some(data);

    let instruction = Instruction::new(op_code, reg_operand, src_operand);

    (length, output, instruction)
}

/// Decodes MOV/ADD/SUB/CMP immediate to register/memory instruction with explicit sizes.
/// Returns instruction length in bytes and output decoded string.
fn decode_immediate_reg_mem(bytes: &[u8], current: usize) -> (usize, String) {
    let mut length: usize = 1;
    let mut b = bytes[current];

    let op_subcode = (bytes[current + 1] & 0b00111000) >> 3;
    let op = if matches!(
        (b & 0b11111110) >> 1,
        op_code::width_7::MOV_IMMEDIATE_REG_MEM
    ) {
        OpCode::Mov
    } else {
        op_code::immediate_reg_mem::get_op_code(op_subcode)
    };

    if matches!(op, OpCode::Invalid) {
        return (0, String::from(""));
    }

    let mut output: String = String::from("");
    let op_str = op_code::strings::get_str(op);

    let sign_extend: bool = b & (1 << 1) != 0;
    let word: bool = b & 1 != 0;

    b = bytes[current + length];
    length += 1;

    let mode = (b & 0b1100_0000) >> 6;
    let rm = b & 0b0000_0111;

    let (rm_str, rm_operand) = match mode {
        displacement_mode::REGISTER => get_register_str_and_operand(rm, word).unwrap(),
        displacement_mode::MEM_8_BIT => {
            b = bytes[current + length];
            length += 1;
            effective_address_calculation::get_eac_str_and_operand(rm, mode, b, 0).unwrap()
        }
        displacement_mode::MEM_16_BIT => {
            b = bytes[current + length];
            let disp_hi = bytes[current + length + 1];
            length += 2;
            effective_address_calculation::get_eac_str_and_operand(rm, mode, b, disp_hi).unwrap()
        }
        displacement_mode::MEM_0_BIT if rm == 0b110 => {
            b = bytes[current + length];
            let disp_hi = bytes[current + length + 1];
            length += 2;
            effective_address_calculation::get_eac_str_and_operand(rm, mode, b, disp_hi).unwrap()
        }
        displacement_mode::MEM_0_BIT => {
            effective_address_calculation::get_eac_str_and_operand(rm, mode, 0, 0).unwrap()
        }
        _ => {
            println!("Invalid MOV mode: {:#b}", mode);
            return (length, output);
        }
    };

    b = bytes[current + length];
    length += 1;

    let mut data: u16 = b as u16;
    let mut data_string;

    if word {
        if !sign_extend {
            b = bytes[current + length];
            length += 1;
            data += b as u16 * 256;
        }
        data_string = String::from("word ");
    } else {
        data_string = String::from("byte ");
    }

    data_string.push_str(&data.to_string());

    output_fmt_op_dest_source(&mut output, op_str, &rm_str, &data_string);

    (length, output)
}

/// Decodes MOV/ADD/SUB/CMP memory to/from/with accumulator.
/// If `dir_acc_mem` parameter is `true`, direction is accumulator to address/data. This is only expected in MOVs.
/// Returns instruction length in bytes and output decoded string.
fn decode_mem_acc(op: OpCode, bytes: &[u8], current: usize, dir_acc_mem: bool) -> (usize, String) {
    let mut output: String = String::from("");
    let op_str = op_code::strings::get_str(op);
    let mut length: usize = 1;
    let mut b = bytes[current];

    let word: bool = b & 1 != 0;

    let reg_string = register::word::get_str(register::word::AX);

    b = bytes[current + length];
    length += 1;

    let mut address: u16 = b as u16;
    let mut address_string = String::from("[");
    if word {
        b = bytes[current + length];
        length += 1;
        address += b as u16 * 256;
    }
    address_string.push_str(&address.to_string());
    address_string.push(']');

    if dir_acc_mem {
        output_fmt_op_dest_source(&mut output, op_str, &address_string, &reg_string);
    } else {
        output_fmt_op_dest_source(&mut output, op_str, &reg_string, &address_string);
    }

    (length, output)
}

/// Decodes instructions that take an 8 bit signed increment as argument (jumps, loops).
/// Returns instruction length in bytes and output decoded string.
fn decode_ip_inc_8(op: OpCode, bytes: &[u8], current: usize) -> (usize, String) {
    let mut output: String = String::from("");
    let op_str = op_code::strings::get_str(op);
    let length: usize = 2;

    let increment: i8 = bytes[current + 1] as i8;
    let increment_string = format!("${:+}", increment);
    output_fmt_op_dest(&mut output, op_str, &increment_string);

    (length, output)
}

fn output_fmt_op_dest_source(
    output: &mut String,
    op_str: &str,
    destination_str: &str,
    source_str: &str,
) {
    output.push_str(op_str);
    output.push(' ');
    output.push_str(destination_str);
    output.push_str(", ");
    output.push_str(source_str);
    output.push('\n');
}

fn output_fmt_op_dest(output: &mut String, op_str: &str, destination_str: &str) {
    output.push_str(op_str);
    output.push(' ');
    output.push_str(destination_str);
    output.push('\n');
}
