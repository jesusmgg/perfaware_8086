use crate::register;

pub fn get_register_str(reg_bytes: u8, is_word: bool) -> String {
    if is_word {
        register::word::get_str(reg_bytes)
    } else {
        register::byte::get_str(reg_bytes)
    }
}
