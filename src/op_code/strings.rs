use super::op::OpCode;

pub fn get_str(op_code: OpCode) -> &'static str {
    match op_code {
        OpCode::Mov => "MOV",
        OpCode::Add => "ADD",
        OpCode::Sub => "SUB",
        OpCode::Cmp => "CMP",
        OpCode::Jnz => "JNZ",
        OpCode::Je => "JE",
        OpCode::Jl => "JL",
        OpCode::Jle => "JLE",
        OpCode::Jb => "JB",
        OpCode::Jbe => "JBE",
        OpCode::Jp => "JP",
        OpCode::Jo => "JO",
        OpCode::Js => "JS",
        OpCode::Jnl => "JNL",
        OpCode::Jg => "JG",
        OpCode::Jnb => "JNB",
        OpCode::Ja => "JA",
        OpCode::Jnp => "JNP",
        OpCode::Jno => "JNO",
        OpCode::Jns => "JNS",
        OpCode::Loop => "LOOP",
        OpCode::Loopz => "LOOPZ",
        OpCode::Loopnz => "LOOPNZ",
        OpCode::Jcxz => "JCXZ",
        OpCode::Invalid => "Invalid OpCode",
    }
}
