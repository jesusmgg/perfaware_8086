use crate::register::{self, util::get_register_string};

pub struct SimulatorState {
    pub registers: SimulatorRegisters,
    pub flags_register: SimulatorFlagsRegister,

    pub ip: u16,
}

pub struct SimulatorRegisters {
    ax: u16,
    bx: u16,
    cx: u16,
    dx: u16,

    sp: u16,
    bp: u16,
    si: u16,
    di: u16,
}

pub struct SimulatorFlagsRegister {
    pub sign: bool,
    pub zero: bool,
}

impl SimulatorState {
    pub fn new() -> Self {
        let registers = SimulatorRegisters::new();
        let flags_register = SimulatorFlagsRegister::new();

        let ip = 0;

        Self {
            registers,
            flags_register,

            ip,
        }
    }

    pub fn print_ip(&self) {
        println!("IP: {}", self.ip);
    }
}

impl SimulatorRegisters {
    fn new() -> Self {
        Self {
            ax: 0,
            bx: 0,
            cx: 0,
            dx: 0,

            sp: 0,
            bp: 0,
            si: 0,
            di: 0,
        }
    }

    /// Read data from a register.
    pub fn read(&self, reg_bytes: u8, is_word: bool) -> u16 {
        match (is_word, reg_bytes) {
            (true, register::word::AX) => self.ax,
            (true, register::word::BX) => self.bx,
            (true, register::word::CX) => self.cx,
            (true, register::word::DX) => self.dx,
            (true, register::word::SP) => self.sp,
            (true, register::word::BP) => self.bp,
            (true, register::word::SI) => self.si,
            (true, register::word::DI) => self.di,
            // TODO: use a recognizable default value
            (true, _) => 0,
            // TODO: manager byte registers
            // TODO: use a recognizable default value
            (false, _) => 0,
        }
    }

    /// Writes data into a register.
    pub fn write(&mut self, data: u16, reg_bytes: u8, is_word: bool) {
        let old_data: u16;

        if is_word {
            match reg_bytes {
                register::word::AX => {
                    old_data = self.ax;
                    self.ax = data;
                }
                register::word::BX => {
                    old_data = self.bx;
                    self.bx = data;
                }
                register::word::CX => {
                    old_data = self.cx;
                    self.cx = data;
                }
                register::word::DX => {
                    old_data = self.dx;
                    self.dx = data;
                }
                register::word::SP => {
                    old_data = self.sp;
                    self.sp = data;
                }
                register::word::BP => {
                    old_data = self.bp;
                    self.bp = data;
                }
                register::word::SI => {
                    old_data = self.si;
                    self.si = data;
                }
                register::word::DI => {
                    old_data = self.di;
                    self.di = data;
                }
                // TODO: use a recognizable default value
                _ => old_data = 0,
            }
        }
        // TODO: manage byte registers
        else {
            match reg_bytes {
                register::byte::AL => {
                    old_data = self.ax;
                    // self.ax = data;
                }
                register::byte::BL => {
                    old_data = self.bx;
                    // self.bx = data;
                }
                register::byte::CL => {
                    old_data = self.cx;
                    // self.cx = data;
                }
                register::byte::DL => {
                    old_data = self.dx;
                    // self.dx = data;
                }
                register::byte::AH => {
                    old_data = self.sp;
                    // self.sp = data;
                }
                register::byte::BH => {
                    old_data = self.bp;
                    // self.bp = data;
                }
                register::byte::CH => {
                    old_data = self.si;
                    // self.si = data;
                }
                register::byte::DH => {
                    old_data = self.di;
                    // self.di = data;
                }
                // TODO: use a recognizable default value
                _ => old_data = 0,
            }
        }

        let reg_string = get_register_string(reg_bytes, is_word);

        println!("  {}: 0x{:02x} -> 0x{:02x}", reg_string, old_data, data);
    }

    pub fn print(&self) {
        println!(
            "  AX: 0x{:02x}
  BX: 0x{:02x}
  CX: 0x{:02x}
  DX: 0x{:02x}
  SP: 0x{:02x}
  BP: 0x{:02x}
  SI: 0x{:02x}
  DI: 0x{:02x}",
            self.ax, self.bx, self.cx, self.dx, self.sp, self.bp, self.si, self.di
        );
    }
}

impl SimulatorFlagsRegister {
    pub fn new() -> Self {
        Self {
            zero: false,
            sign: false,
        }
    }

    pub fn print(&self) {
        let mut flags_string = String::new();

        if self.sign {
            flags_string.push('S');
        }
        if self.zero {
            flags_string.push('Z');
        }

        if flags_string.len() == 0 {
            flags_string.push('-');
        }

        println!("  Flags: {}", flags_string);
    }
}
