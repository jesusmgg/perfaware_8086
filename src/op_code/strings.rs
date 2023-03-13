use super::op::OpCode;

pub fn get_str(op_code: OpCode) -> &'static str {
    match op_code {
        OpCode::Mov => "MOV",
        OpCode::Add => "ADD",
        OpCode::Sub => "SUB",
        OpCode::Cmp => "CMP",
        OpCode::Invalid => "Invalid OpCode",
    }
}
