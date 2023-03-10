use crate::displacement_mode;

pub fn get_str(rm: u8, mode: u8, displacement_low: u8, displacement_high: u8) -> String {
    let mut eac_str = match rm {
        0b000 => "[BX + SI",
        0b001 => "[BX + DI",
        0b010 => "[BP + SI",
        0b011 => "[BP + DI",
        0b100 => "[SI",
        0b101 => "[DI",
        0b110 if mode == displacement_mode::MEM_0_BIT => "[",
        0b110 => "[BP",
        0b111 => "[BX",
        _ => {
            println!(
                "Error decoding effective address calculation, invalid R/M value: {:#b}",
                rm
            );
            return String::from("");
        }
    };

    let mut eac_string = String::from(eac_str);
    match mode {
        displacement_mode::MEM_0_BIT if rm == 0b110 => {
            eac_string.push_str(
                (displacement_low as u16 + displacement_high as u16 * 256)
                    .to_string()
                    .as_str(),
            );
        }
        displacement_mode::MEM_8_BIT => {
            eac_string.push_str(" + ");
            eac_string.push_str(displacement_low.to_string().as_str());
        }
        displacement_mode::MEM_16_BIT => {
            eac_string.push_str(" + ");
            eac_string.push_str(
                (displacement_low as u16 + displacement_high as u16 * 256)
                    .to_string()
                    .as_str(),
            );
        }
        _ => (),
    };

    eac_string.push_str("]");

    eac_string
}
