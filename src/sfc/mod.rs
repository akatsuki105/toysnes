mod cartridge;
mod cpu;
mod ppu;

use super::scheduler;

pub struct SuperFamicom {}

impl SuperFamicom {
    pub fn new(rom_data: &[u8]) -> Self {
        let s = Self {};
        scheduler::schedule(&mut ppu::get_mut().frame_event, 700);
        cpu::get_mut().load_rom(rom_data);
        return s;
    }

    pub fn run_frame(&mut self) {
        let old = ppu::frame();
        while old == ppu::frame() {
            cpu::get_mut().run();
        }
    }

    pub fn frame_count(&self) -> u64 {
        return ppu::frame();
    }

    pub fn cartridge_info(&self) {
        println!("{}", cpu::get().cartridge);
    }

    pub fn unimplemented_opcode(&self) -> [bool; 256] {
        return cpu::get().unimplemented();
    }
}

/// This is is inspired by snes9x's S9xGetByte
pub fn load8(bank: u8, ofs: u16, cycles: Option<&mut i64>) -> u8 {
    todo!()
}

/// This is is inspired by snes9x's S9xGetWord
pub fn load16(bank: u8, ofs: u16, cycles: Option<&mut i64>) -> u16 {
    todo!()
}

/// This is is inspired by snes9x's S9xSetByte
fn store8(bank: u8, addr: u16, val: u8, cycles: Option<&mut i64>) {
    todo!();
}

/// This is is inspired by snes9x's S9xSetWord
fn store16(bank: u8, addr: u16, val: u16, cycles: Option<&mut i64>) {
    todo!();
}
