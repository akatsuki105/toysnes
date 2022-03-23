use super::{add_cycles, cycles, Cpu};
use crate::sfc::load16;

impl Cpu {
    /// `[DB:nnnn]`
    pub fn absolute(&mut self) -> (u8, u16) {
        return (self.r.db, self.imm16());
    }

    /// `[DB:nnnn+X]`
    pub fn absolute_x(&mut self) -> (u8, u16) {
        if self.r.p.x() == 0 {
            // Add one cycle if indexing crosses a page boundary.
            add_cycles(1);
        }
        return (self.r.db, self.imm16() + self.r.x);
    }

    /// `[DB:nnnn+Y]`
    pub fn absolute_y(&mut self) -> (u8, u16) {
        if self.r.p.x() == 0 {
            // Add one cycle if indexing crosses a page boundary.
            add_cycles(1);
        }
        return (self.r.db, self.imm16() + self.r.y);
    }

    /// `[D+nn]`
    pub fn direct(&mut self) -> (u8, u16) {
        let addr = self.imm8() as u16 + self.r.d;
        return (0, addr);
    }

    /// `[D+nn+X]`
    pub fn direct_x(&mut self) -> (u8, u16) {
        let nn = self.imm8() as u16;
        let addr = self.r.d + nn + self.r.x;
        return (0, addr);
    }

    /// `[nn+S]`
    pub fn stack_rel(&mut self) -> (u8, u16) {
        let addr = self.imm8() as u16 + self.r.s;
        return (0, addr);
    }

    /// `[WORD[D+nn+X]]`
    pub fn indirect_x(&mut self) -> (u8, u16) {
        let (_, iaddr) = self.direct_x();
        let addr = load16(0, iaddr, Some(cycles()));
        return (self.r.db, addr);
    }

    /// `[FAR[D+nn]]`
    pub fn far(&mut self) -> (u8, u16) {
        todo!()
    }

    /// `[FAR[D+nn]+Y]`
    pub fn far_y(&mut self) -> (u8, u16) {
        todo!()
    }
}
