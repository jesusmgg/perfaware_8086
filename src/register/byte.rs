pub const AL: u8 = 0b000;
pub const CL: u8 = 0b001;
pub const DL: u8 = 0b010;
pub const BL: u8 = 0b011;

pub const AH: u8 = 0b100;
pub const CH: u8 = 0b101;
pub const DH: u8 = 0b110;
pub const BH: u8 = 0b111;

// TODO: merge this with the effective address calculation module
pub fn get_str(register: u8) -> String {
    match register {
        AL => "AL".to_string(),
        CL => "CL".to_string(),
        DL => "DL".to_string(),
        BL => "BL".to_string(),
        AH => "AH".to_string(),
        CH => "CH".to_string(),
        DH => "DH".to_string(),
        BH => "BH".to_string(),
        _ => "INVALID_REGISTER".to_string(),
    }
}
