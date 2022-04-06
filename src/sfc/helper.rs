use crate::constants::{FAST, MEDIUM, SLOW};

pub fn split_addr24(addr24: u32) -> (u8, u16) {
    let bank = ((addr24 >> 16) & 0xff) as u8;
    let addr16 = (addr24 & 0xffff) as u16;
    return (bank, addr16);
}

pub fn memory_speed(addr24: u32) -> i64 {
    let (bank, addr) = split_addr24(addr24);

    if (bank & 0x40 != 0) || (addr & 0x8000 != 0) {
        if bank & 0x80 != 0 {
            todo!()
        }
        return MEDIUM;
    }

    if ((addr24 + 0x6000) & 0x4000) != 0 {
        return MEDIUM;
    }

    if ((addr24 - 0x4000) & 0x7e00) != 0 {
        return FAST;
    }

    return SLOW;
}
