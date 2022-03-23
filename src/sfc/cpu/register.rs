use modular_bitfield::prelude::*;

pub struct Register {
    /// Data bank
    pub db: u8,

    /// PSR: Processor Status Register
    pub p: Psr,

    /// Accumulator
    pub a: u16,

    /// Zeropage Offset
    pub d: u16,

    /// Stack pointer
    pub s: u16,

    /// Index X
    pub x: u16,

    /// Index Y
    pub y: u16,

    /// Program counter(24bit)
    pub pc: PC,

    /// 6502 Emulation Flag(E)
    pub emulation: bool,
}

#[derive(Default)]
pub struct PC {
    pub bank: u8,
    pub offset: u16,
}

impl PC {
    pub fn word(&self) -> u32 {
        return ((self.bank as u32) << 16) | (self.offset as u32);
    }

    pub fn set(&mut self, bank: u8, offset: u16) {
        self.bank = bank;
        self.offset = offset;
    }
}

impl Register {
    pub fn new() -> Self {
        Self {
            db: 0,
            p: Psr::new(),
            a: 0,
            d: 0,
            s: 0,
            x: 0,
            y: 0,
            pc: PC::default(),
            emulation: false,
        }
    }

    fn in_6502_emulation(&self) -> bool {
        return self.emulation;
    }
}

#[bitfield]
pub struct Psr {
    /// carry
    pub c: B1,

    /// zero
    pub z: B1,

    /// interrupt disable
    pub i: B1,

    /// decimal mode
    pub d: B1,

    /// index register mode
    pub x: B1,

    /// accumulator mode
    /// In this mode, switches A to 8bit mode
    pub m: B1,

    /// overflow
    pub v: B1,

    /// negative
    pub n: B1,
}

impl Psr {
    pub fn set_zn(&mut self, val: u16) {
        self.set_z((val == 0) as u8);
        self.set_n(((val >> 8 & 0xff) > 0) as u8);
    }
}
