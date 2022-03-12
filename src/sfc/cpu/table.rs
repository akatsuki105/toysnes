use super::instruction::*;
use super::Cpu;

type Opcode = fn(&mut Cpu, u8);

fn _default_opcode(_: &mut Cpu, _: u8) {}

pub struct OpcodeTable {
    /// This table is inspired by snes9x's S9xOpcodesE1.
    /// In 6502 emulation mode(E is set), use this as optable.
    pub e1: [Opcode; 256],

    /// This table is inspired by snes9x's S9xOpcodesM1X1
    /// In both M and X are set, use this as optable.
    pub m1x1: [Opcode; 256],

    /// This table is inspired by snes9x's S9xOpcodesM1X0
    pub m1x0: [Opcode; 256],

    /// This table is inspired by snes9x's S9xOpcodesM0X0
    pub m0x0: [Opcode; 256],

    /// This table is inspired by snes9x's S9xOpcodesM0X1
    pub m0x1: [Opcode; 256],
}

impl Default for OpcodeTable {
    fn default() -> Self {
        let mut ot = Self {
            e1: [_default_opcode; 256],
            m1x1: [_default_opcode; 256],
            m1x0: [_default_opcode; 256],
            m0x0: [_default_opcode; 256],
            m0x1: [_default_opcode; 256],
        };
        setup_e1(&mut ot.e1);
        setup_m1x1(&mut ot.m1x1);
        setup_m1x0(&mut ot.m1x0);
        setup_m0x0(&mut ot.m0x0);
        setup_m0x1(&mut ot.m0x1);

        return ot;
    }
}

fn setup_e1(tbl: &mut [Opcode; 256]) {
    tbl[0x00] = op00;
    tbl[0x01] = op01::e1;
    tbl[0x02] = op02;
    tbl[0x03] = op03::m1;
    tbl[0x09] = op09::m1;
    tbl[0x29] = op29::m1;
}

fn setup_m1x1(tbl: &mut [Opcode; 256]) {
    tbl[0x00] = op00;
    tbl[0x01] = op01::e0m1;
    tbl[0x02] = op02;
    tbl[0x03] = op03::m1;
    tbl[0x09] = op09::m1;
    tbl[0x29] = op29::m1;
}

fn setup_m1x0(tbl: &mut [Opcode; 256]) {
    tbl[0x00] = op00;
    tbl[0x01] = op01::e0m1;
    tbl[0x02] = op02;
    tbl[0x03] = op03::m1;
    tbl[0x09] = op09::m1;
    tbl[0x29] = op29::m1;
}

fn setup_m0x0(tbl: &mut [Opcode; 256]) {
    tbl[0x00] = op00;
    tbl[0x01] = op01::e0m0;
    tbl[0x02] = op02;
    tbl[0x03] = op03::m0;
    tbl[0x09] = op09::m0;
    tbl[0x29] = op29::m0;
}

fn setup_m0x1(tbl: &mut [Opcode; 256]) {
    tbl[0x00] = op00;
    tbl[0x01] = op01::e0m0;
    tbl[0x02] = op02;
    tbl[0x03] = op03::m0;
    tbl[0x09] = op09::m0;
    tbl[0x29] = op29::m0;
}
