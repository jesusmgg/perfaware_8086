mod displacement_mode;
mod op_code;
mod register;

use std::env;

use std::fs;
use std::io::Write;

fn main() {
    let args: Vec<String> = env::args().collect();
    let operation = &args[1];
    let operand = &args[2];

    match operation.as_str() {
        "decode" => decode(operand),
        &_ => {}
    }
}

/// # 8086 instruction structure
/// ## Byte 0
/// - bits 0-5      : opcode
/// - bit 6         : direction      0 -> REG is source     1 -> REG is destination
/// - bit 7         : byte or word   0 -> byte operation    1 -> word operation  
/// ## Byte 1
/// - bits 0-1      : MODE
/// - bits 2-4      : REG
/// - bits 5-7      : R/M
fn decode(file_name: &str) {
    let bytes = &fs::read(file_name).unwrap();
    let bytes_len = bytes.len();
    let mut current: usize = 0;

    let mut output: String = Default::default();

    output.push_str("bits 16\n\n");

    while current < bytes_len {
        let b = bytes[current]; // Current byte
        let instruction_length: usize;
        let decoded: String;

        (instruction_length, decoded) = match (b & 0b1111_1100) >> 2 {
            op_code::MOV => decode_mov(bytes, current),
            _ => (0, String::from("")), // TODO: replace by static empty string
        };

        output.push_str(&decoded);
        current += instruction_length;
    }

    let mut out_file = fs::File::create("out.asm").unwrap();
    write!(&mut out_file, "{}", &output).unwrap();
    out_file.flush().unwrap();
}

/// Decodes MOV instruction.
/// Returns instruction length in bytes.
fn decode_mov(bytes: &[u8], current: usize) -> (usize, String) {
    let mut output: String = "MOV ".to_string();
    let mut length: usize = 1;
    let mut b = bytes[current];

    let direction: bool = b & (1 << 1) != 0;
    let word: bool = b & (1 << 0) != 0;

    b = bytes[current + 1];
    length += 1;

    let mode = (b & 0b1100_0000) >> 6;

    let reg = (b & 0b0011_1000) >> 3;
    let rm = b & 0b0000_0111;

    let reg_str = if word {
        register::word::get_str(reg)
    } else {
        register::byte::get_str(reg)
    };
    let rm_str = if word {
        register::word::get_str(rm)
    } else {
        register::byte::get_str(rm)
    };

    // direction == 1 => reg is destination
    let (destination_str, source_str) = if direction {
        (&reg_str, &rm_str)
    } else {
        (&rm_str, &reg_str)
    };

    output.push_str(destination_str);
    output.push_str(", ");
    output.push_str(source_str);
    output.push('\n');

    (length, output)
}
