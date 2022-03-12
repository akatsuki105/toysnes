use once_cell::sync::Lazy;

use super::scheduler;
use crate::{constants::VRAM_SIZE, scheduler::event::Event};

pub struct Ppu {
    pub vram: Box<[u8; VRAM_SIZE]>,
    pub frame_counter: u64,
    pub frame_event: Event,
}

static mut PPU: Lazy<Ppu> = Lazy::new(|| new());

fn new() -> Ppu {
    let mut p = Ppu {
        vram: Box::new([0; VRAM_SIZE]),
        frame_counter: 0,
        frame_event: Event::default(),
    };
    p.frame_event = Event::new("ppu_event".to_string(), 1, increment_frame);
    return p;
}

pub fn get() -> &'static Ppu {
    return unsafe { &PPU };
}

pub fn get_mut() -> &'static mut Ppu {
    return unsafe { &mut PPU };
}

fn increment_frame(_: i64) {
    let p = get_mut();
    p.frame_counter += 1;
    scheduler::schedule(&mut p.frame_event, 700);
}

pub fn frame() -> u64 {
    return get_mut().frame_counter;
}
