mod header;

use header::Header;
use std::fmt;

use crate::constants::*;

pub struct Cartridge {
    header: Header,

    pub rom: Vec<u8>,
    pub sram: Box<[u8; SRAM_SIZE]>,
}

impl Cartridge {
    pub fn new(rom_data: &[u8]) -> Self {
        Self {
            header: Header::new(rom_data),
            rom: Vec::with_capacity(ROM_SIZE),
            sram: Box::new([0; SRAM_SIZE]),
        }
    }
}

impl Default for Cartridge {
    fn default() -> Self {
        return Self {
            header: Header::default(),
            rom: Vec::with_capacity(0),
            sram: Box::new([0; SRAM_SIZE]),
        };
    }
}

impl fmt::Display for Cartridge {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.header)
    }
}
