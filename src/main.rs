mod displacement_mode;
mod effective_address_calculation;
mod op_code;
mod register;

use std::env;

use std::fs;

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
                op_code::width_6::MOV => decode_mov(bytes, current),
                _ => (0, String::from("")),
            };
        }

        if instruction_length == 0 {
            eprintln!("Error: Instruction not handled (byte: {:#b})", b);
            break;
        }

        output.push_str(&decoded);
        current += instruction_length;
    }

    println!("{}", &output);
}

/// Decodes MOV instruction.
/// Returns instruction length in bytes and output decoded string.
fn decode_mov(bytes: &[u8], current: usize) -> (usize, String) {
    let mut output: String = "MOV ".to_string();
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
        displacement_mode::MEM_0_BIT => effective_address_calculation::get_str(rm, mode, 0, 0),
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

    output_mov(&mut output, destination_str, source_str);
    (length, output)
}

/// Decodes MOV immediate to register instruction.
/// Returns instruction length in bytes and output decoded string.
fn decode_mov_immediate_reg(bytes: &[u8], current: usize) -> (usize, String) {
    let mut output: String = "MOV ".to_string();
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

    output_mov(&mut output, &reg_str, &data.to_string());

    (length, output)
}

fn output_mov(output: &mut String, destination_str: &str, source_str: &str) {
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
