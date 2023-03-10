pub const AX: u8 = 0b000;
pub const CX: u8 = 0b001;
pub const DX: u8 = 0b010;
pub const BX: u8 = 0b011;

pub const SP: u8 = 0b100;
pub const BP: u8 = 0b101;
pub const SI: u8 = 0b110;
pub const DI: u8 = 0b111;

// TODO: merge this with the effective address calculation module
pub fn get_str(register: u8) -> String {
    match register {
        AX => "AX".to_string(),
        CX => "CX".to_string(),
        DX => "DX".to_string(),
        BX => "BX".to_string(),
        SP => "SP".to_string(),
        BP => "BP".to_string(),
        SI => "SI".to_string(),
        DI => "DI".to_string(),
        _ => "INVALID_REGISTER".to_string(),
    }
}
