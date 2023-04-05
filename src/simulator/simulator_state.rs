pub struct SimulatorState {
    registers: SimulatorRegisters,
}

struct SimulatorRegisters {
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
}
