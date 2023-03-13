use super::op::OpCode;

pub const ADD: u8 = 0b000;
pub const SUB: u8 = 0b101;
pub const CMP: u8 = 0b111;

pub fn get_op_code(op_subcode: u8) -> OpCode {
    match op_subcode {
        ADD => OpCode::Add,
        SUB => OpCode::Sub,
        CMP => OpCode::Cmp,
        _ => {
            println!("Invalid sub op code for bytes: {:#b}", op_subcode);
            OpCode::Invalid
        }
    }
}
