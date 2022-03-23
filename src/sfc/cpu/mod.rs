mod addr;
mod instruction;
mod register;
mod table;

use std::ptr;

use once_cell::sync::Lazy;

use crate::constants::WRAM_SIZE;
use crate::scheduler;

use super::cartridge;

static mut CPU: Lazy<Cpu> = Lazy::new(|| new());

const FAST: i64 = 6; // 6 * (3.58/3.58)
const MEDIUM: i64 = 8; // 6 * (3.58/2.68)
const SLOW: i64 = 12; // 6 * (3.58/1.78)

pub struct Cpu {
    pub cartridge: cartridge::Cartridge,
    pub wram: Box<[u8; WRAM_SIZE]>,

    pub r: register::Register,
    table: table::OpcodeTable,

    pub blocked: bool,

    pub waitstate: i64,
    pub active_region: *mut u8,
}

pub fn new() -> Cpu {
    Cpu {
        cartridge: cartridge::Cartridge::default(),
        wram: Box::new([0; WRAM_SIZE]),
        r: register::Register::new(),
        table: table::OpcodeTable::default(),
        blocked: false,
        waitstate: MEDIUM,
        active_region: ptr::null_mut(),
    }
}

pub fn get() -> &'static Cpu {
    return unsafe { &CPU };
}

pub fn get_mut() -> &'static mut Cpu {
    return unsafe { &mut CPU };
}

pub fn cycles() -> &'static mut i64 {
    return &mut scheduler::get_mut().relative_cycles;
}

pub fn add_cycles(c: i64) {
    scheduler::get_mut().relative_cycles += c;
}

impl Cpu {
    pub fn load_rom(&mut self, rom_data: &[u8]) {
        self.cartridge = cartridge::Cartridge::new(rom_data);
    }

    pub fn run(&mut self) {
        while scheduler::any_event() {
            self.process_event();
        }
        self.step();
    }

    pub fn step(&mut self) {
        let op = self.imm8();
        self.r.pc.offset = self.r.pc.offset.wrapping_add(1);
        self.table.e1[op as usize](self, op);
        add_cycles(2);
    }

    pub fn process_event(&mut self) {
        let s = scheduler::get_mut();
        let mut next_event = s.next_event;
        while s.relative_cycles >= next_event {
            s.next_event = i64::MAX;
            next_event = 0;

            let mut first = true;
            while first || self.blocked {
                first = false;

                let cycles = s.relative_cycles;
                scheduler::get_mut().relative_cycles = 0;

                if cycles < next_event {
                    next_event = scheduler::add(next_event);
                } else {
                    next_event = scheduler::add(cycles);
                }
            }

            s.next_event = next_event;
        }

        if self.blocked {
            s.relative_cycles = s.next_event;
        }
    }

    fn imm8(&mut self) -> u8 {
        let val = unsafe { *self.active_region.add(self.r.pc.offset as usize) };
        add_cycles(self.waitstate);
        self.r.pc.offset += 1;
        return val;
    }

    fn set_imm8(&mut self, val: u8) {
        self.store8(self.r.pc.bank, self.r.pc.offset, val, Some(cycles()));
    }

    fn imm16(&mut self) -> u16 {
        let lo = unsafe { *self.active_region.add(self.r.pc.offset as usize) } as u16;
        let hi = unsafe { *self.active_region.add((self.r.pc.offset + 1) as usize) } as u16;
        add_cycles(self.waitstate * 2);
        self.r.pc.offset += 2;
        return (hi << 8) | lo;
    }

    fn set_imm16(&mut self, val: u16) {
        self.store16(self.r.pc.bank, self.r.pc.offset, val, Some(cycles()));
    }

    fn store8(&mut self, bank: u8, addr: u16, val: u8, cycles: Option<&mut i64>) {
        todo!();
    }

    fn store16(&mut self, bank: u8, addr: u16, val: u16, cycles: Option<&mut i64>) {
        todo!();
    }

    pub fn unimplemented(&self) -> [bool; 256] {
        return self.table.unimplemented();
    }

    /// This func is inspired by snes9x's S9xSetPCBase
    pub fn set_active_region(&mut self, addr: u32) {}
}

/// This func is inspired by breeze-emu's do_io_cycle
fn mem_access_cycles(bank: u8, addr: u16) -> i64 {
    let c = match bank {
        0x00..=0x3f => match addr {
            0x0000..=0x1fff | 0x6000..=0xffff => MEDIUM,
            0x4000..=0x41ff => SLOW,
            _ => FAST,
        },
        0x40..=0x7f => MEDIUM,
        0x80..=0xbf => {
            match addr {
                0x0000..=0x1fff | 0x6000..=0x7fff => MEDIUM,
                0x4000..=0x41ff => SLOW,
                0x8000..=0xffff => MEDIUM, // TODO
                _ => FAST,
            }
        }
        0xc0..=0xff => MEDIUM, // TODO
        _ => FAST,
    };
    return c;
}
