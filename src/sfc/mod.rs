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
}
