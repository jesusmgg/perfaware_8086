mod displacement_mode;
mod effective_address_calculation;
mod op_code;
mod register;

use std::env;

use std::fs;

use op_code::op::OpCode;

fn main() {
    let args: Vec<String> = env::args().collect();
    let operation = &args[1];
    let operand = &args[2];

    match operation.as_str() {
        "decode" => decode(operand),
        &_ => {}
    }
}

fn decode(file_name: &str) {
    let bytes = &fs::read(file_name).unwrap();
    let bytes_len = bytes.len();
    let mut current: usize = 0;

    let mut output: String = Default::default();

    output.push_str("bits 16\n\n");

    while current < bytes_len {
        let b = bytes[current]; // Current byte
        let mut instruction_length: usize;
        let mut decoded: String;

        // Instruction width 4
        (instruction_length, decoded) = match (b & 0b1111_0000) >> 4 {
            op_code::width_4::MOV_IMMEDIATE_REG => decode_mov_immediate_reg(bytes, current),
            _ => (0, String::from("")),
        };

        // Instruction width 6
        if instruction_length == 0 {
            (instruction_length, decoded) = match (b & 0b1111_1100) >> 2 {
                op_code::width_6::MOV_REG_MEM_REG => {
                    decode_reg_mem_reg(OpCode::Mov, bytes, current)
                }
                op_code::width_6::ADD_REG_MEM_REG => {
                    decode_reg_mem_reg(OpCode::Add, bytes, current)
                }
                op_code::width_6::SUB_REG_MEM_REG => {
                    decode_reg_mem_reg(OpCode::Sub, bytes, current)
                }
                op_code::width_6::CMP_REG_MEM_REG => {
                    decode_reg_mem_reg(OpCode::Cmp, bytes, current)
                }
                op_code::width_6::IMMEDIATE_REG_MEM => decode_immediate_reg_mem(bytes, current),
                _ => (0, String::from("")),
            };
        }

        // Instruction width 7
        if instruction_length == 0 {
            (instruction_length, decoded) = match (b & 0b1111_1110) >> 1 {
                op_code::width_7::MOV_IMMEDIATE_REG_MEM => decode_immediate_reg_mem(bytes, current),
                op_code::width_7::MOV_MEM_ACC => decode_mem_acc(OpCode::Mov, bytes, current, false),
                op_code::width_7::MOV_ACC_MEM => decode_mem_acc(OpCode::Mov, bytes, current, true),
                op_code::width_7::ADD_IMMEDIATE_ACC => {
                    decode_mem_acc(OpCode::Add, bytes, current, false)
                }
                op_code::width_7::SUB_IMMEDIATE_ACC => {
                    decode_mem_acc(OpCode::Sub, bytes, current, false)
                }
                op_code::width_7::CMP_IMMEDIATE_ACC => {
                    decode_mem_acc(OpCode::Cmp, bytes, current, false)
                }
                _ => (0, String::from("")),
            };
        }

        if instruction_length == 0 {
            eprintln!("Error: Instruction not handled (byte: {:#b})", b);
            break;
        }

        // Debug print
        for i in current..current + instruction_length {
            print!("{:08b} ", bytes[i]);
        }
        println!("  =>  {}", &decoded);

        output.push_str(&decoded);
        current += instruction_length;
    }

    println!("{}", &output);
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

    let reg_str = get_register_str(reg, word);

    let rm_str: String = match mode {
        displacement_mode::REGISTER => get_register_str(rm, word),
        displacement_mode::MEM_8_BIT => {
            b = bytes[current + length];
            length += 1;
            effective_address_calculation::get_str(rm, mode, b, 0)
        }
        displacement_mode::MEM_16_BIT => {
            b = bytes[current + length];
            let disp_hi = bytes[current + length + 1];
            length += 2;
            effective_address_calculation::get_str(rm, mode, b, disp_hi)
        }
        displacement_mode::MEM_0_BIT if rm == 0b110 => {
            b = bytes[current + length];
            let disp_hi = bytes[current + length + 1];
            length += 2;
            effective_address_calculation::get_str(rm, mode, b, disp_hi)
        }
        displacement_mode::MEM_0_BIT => effective_address_calculation::get_str(rm, mode, 0, 0),
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

    output_fmt(&mut output, op_str, destination_str, source_str);
    (length, output)
}

/// Decodes MOV immediate to register instruction.
/// Returns instruction length in bytes and output decoded string.
fn decode_mov_immediate_reg(bytes: &[u8], current: usize) -> (usize, String) {
    let mut output: String = String::from("");
    let op_str = op_code::strings::get_str(OpCode::Mov);

    let mut length: usize = 1;
    let mut b = bytes[current];

    let word: bool = b & (1 << 3) != 0;
    let reg = b & 0b0000_0111;
    let reg_str = get_register_str(reg, word);

    b = bytes[current + length];
    length += 1;

    let mut data: u16 = b as u16;

    if word {
        b = bytes[current + length];
        length += 1;
        data += b as u16 * 256;
    }

    output_fmt(&mut output, op_str, &reg_str, &data.to_string());

    (length, output)
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

    let rm_str: String = match mode {
        displacement_mode::REGISTER => get_register_str(rm, word),
        displacement_mode::MEM_8_BIT => {
            b = bytes[current + length];
            length += 1;
            effective_address_calculation::get_str(rm, mode, b, 0)
        }
        displacement_mode::MEM_16_BIT => {
            b = bytes[current + length];
            let disp_hi = bytes[current + length + 1];
            length += 2;
            effective_address_calculation::get_str(rm, mode, b, disp_hi)
        }
        displacement_mode::MEM_0_BIT if rm == 0b110 => {
            b = bytes[current + length];
            let disp_hi = bytes[current + length + 1];
            length += 2;
            effective_address_calculation::get_str(rm, mode, b, disp_hi)
        }
        displacement_mode::MEM_0_BIT => effective_address_calculation::get_str(rm, mode, 0, 0),
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

    output_fmt(&mut output, op_str, &rm_str, &data_string);

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
        output_fmt(&mut output, op_str, &address_string, &reg_string);
    } else {
        output_fmt(&mut output, op_str, &reg_string, &address_string);
    }

    (length, output)
}

fn output_fmt(output: &mut String, op_str: &str, destination_str: &str, source_str: &str) {
    output.push_str(op_str);
    output.push(' ');
    output.push_str(destination_str);
    output.push_str(", ");
    output.push_str(source_str);
    output.push('\n');
}

fn get_register_str(reg_bytes: u8, is_word: bool) -> String {
    if is_word {
        register::word::get_str(reg_bytes)
    } else {
        register::byte::get_str(reg_bytes)
    }
}
