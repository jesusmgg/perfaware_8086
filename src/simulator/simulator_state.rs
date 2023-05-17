use crate::register;

pub struct SimulatorState {
    pub registers: SimulatorRegisters,
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

impl SimulatorState {
    pub fn new() -> Self {
        let registers = SimulatorRegisters::new();

        Self { registers }
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

    /// Writes data into a register.
    pub fn write(&mut self, data: u16, reg_bytes: u8, is_word: bool) {
        if is_word {
            match reg_bytes {
                register::word::AX => self.ax = data,
                register::word::BX => self.bx = data,
                register::word::CX => self.cx = data,
                register::word::DX => self.dx = data,
                register::word::SP => self.sp = data,
                register::word::BP => self.bp = data,
                register::word::SI => self.si = data,
                register::word::DI => self.di = data,
                _ => (),
            }
        }
        // else {
        //     match reg_bytes {
        //         register::byte::AL => self.ax = data,
        //         register::byte::BL => self.bx = data,
        //         register::byte::CL => self.cx = data,
        //         register::byte::DL => self.dx = data,
        //         register::byte::AH => self.sp = data,
        //         register::byte::BH => self.bp = data,
        //         register::byte::CH => self.si = data,
        //         register::byte::DH => self.di = data,
        //         _ => (),
        //     }
    }

    pub fn print(&self) {
        println!(
            "ax: 0x{:02x}
bx: 0x{:02x}
cx: 0x{:02x}
dx: 0x{:02x}

sp: 0x{:02x}
bp: 0x{:02x}
si: 0x{:02x}
di: 0x{:02x}
",
            self.ax, self.bx, self.cx, self.dx, self.sp, self.bp, self.si, self.di
        );
    }
}
