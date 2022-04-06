use super::Cpu;

impl Cpu {
    /// This func is inspired by snes9x's S9xGetCPU
    pub fn read_io8(&mut self, addr: u16) -> u8 {
        return 0;
    }

    /// This func is inspired by snes9x's S9xSetCPU
    pub fn write_io8(&mut self, addr: u16, val: u8) {
        todo!()
    }
}
