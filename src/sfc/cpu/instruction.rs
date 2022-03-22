use super::Cpu;
use super::{add_cycles, mem_access_cycles};

/// This func is inspired by snes9x's Op00
pub fn op00(c: &mut Cpu, _: u8) {}

/// ORA (nn, X)
pub mod op01 {
    use super::Cpu;
    use crate::sfc::cpu::cycles;

    /// This func is inspired by snes9x's Op01E0M0
    pub fn e0m0(c: &mut Cpu, _: u8) {
        let (bank, addr) = c.indirect_x();
        c.r.a |= c.load16(bank, addr, Some(cycles()));
        c.r.p.set_zn(c.r.a);
    }

    /// This func is inspired by snes9x's Op01E0M1
    pub fn e0m1(c: &mut Cpu, _: u8) {
        let (bank, addr) = c.indirect_x();
        c.r.a |= c.load16(bank, addr, Some(cycles())) & 0xff;
        c.r.p.set_zn(c.r.a);
    }

    /// This func is inspired by snes9x's Op01E1
    pub fn e1(c: &mut Cpu, _: u8) {
        let (bank, addr) = c.indirect_x();
        c.r.a |= c.load8(bank, addr, Some(cycles())) as u16;
        c.r.p.set_zn(c.r.a);
    }
}

/// COP
/// This func is inspired by snes9x's Op02
pub fn op02(c: &mut Cpu, _: u8) {
    todo!()
}

/// ORA nn,S
/// effect: [nn+S]
pub mod op03 {
    use crate::sfc::cpu::cycles;

    use super::Cpu;

    /// This func is inspired by snes9x's Op03M0
    pub fn m0(c: &mut Cpu, _: u8) {
        let (bank, addr) = c.stack_rel();
        c.r.a |= c.load16(bank, addr, Some(cycles()));
        c.r.p.set_zn(c.r.a);
    }

    /// This func is inspired by snes9x's Op03M1
    pub fn m1(c: &mut Cpu, _: u8) {
        let (bank, addr) = c.stack_rel();
        c.r.a |= c.load8(bank, addr, Some(cycles())) as u16;
        c.r.p.set_zn(c.r.a);
    }
}

/// TSB nn
pub fn op04(c: &mut Cpu, _: u8) {
    if c.r.p.m() > 0 {
        let nn = c.imm8();
        c.r.p.set_z(((nn - (c.r.a as u8)) != 0) as u8);
        c.set_imm8(nn | (c.r.a as u8));
        return;
    }
    let nn = c.imm16();
    c.r.p.set_z(((nn - c.r.a) != 0) as u8);
    c.set_imm16(nn | c.r.a);
    return;
}

/// ORA nn
/// effect: [D+nn]
pub mod op05 {
    use super::Cpu;
    use crate::sfc::cpu::cycles;

    pub fn m0(c: &mut Cpu, _: u8) {
        let (bank, addr) = c.direct();
        c.r.a |= c.load16(bank, addr, Some(cycles()));
        c.r.p.set_zn(c.r.a);
    }

    pub fn m1(c: &mut Cpu, _: u8) {
        let (bank, addr) = c.direct();
        c.r.a |= c.load8(bank, addr, Some(cycles())) as u16;
        c.r.p.set_zn(c.r.a);
    }
}

/// ORA [nn]
/// effect: [FAR[D+nn]]
pub mod op07 {
    use super::Cpu;
    use crate::sfc::cpu::cycles;

    pub fn m0(c: &mut Cpu, _: u8) {
        let (bank, addr) = c.far();
        c.r.a |= c.load16(bank, addr, Some(cycles()));
        c.r.p.set_zn(c.r.a);
    }
}

/// ORA #nn
/// effect: nn
pub mod op09 {
    use super::Cpu;

    /// ORA #nn ; M=0
    /// This func is inspired by snes9x's Op09M0
    pub fn m0(c: &mut Cpu, _: u8) {
        c.r.a |= c.imm16();
        c.r.p.set_zn(c.r.a);
    }

    /// ORA #nn ; M=1
    /// This func is inspired by snes9x's Op09M1
    pub fn m1(c: &mut Cpu, _: u8) {
        c.r.a |= c.imm8() as u16;
        c.r.p.set_zn(c.r.a);
    }
}

/// ORA #nn
/// effect: [D+nn+X]
pub mod op15 {
    use super::Cpu;
    use crate::sfc::cpu::cycles;

    pub fn m0(c: &mut Cpu, _: u8) {
        let (bank, addr) = c.direct_x();
        c.r.a |= c.load16(bank, addr, Some(cycles()));
        c.r.p.set_zn(c.r.a);
    }

    pub fn m1(c: &mut Cpu, _: u8) {
        let (bank, addr) = c.direct_x();
        c.r.a |= c.load8(bank, addr, Some(cycles())) as u16;
        c.r.p.set_zn(c.r.a);
    }
}

/// AND #nn
pub mod op29 {
    use super::Cpu;

    /// This func is inspired by snes9x's Op29M0
    pub fn m0(c: &mut Cpu, _: u8) {
        c.r.a &= c.imm16();
        c.r.p.set_zn(c.r.a);
    }

    /// This func is inspired by snes9x's Op29M1
    pub fn m1(c: &mut Cpu, _: u8) {
        c.r.a &= c.imm8() as u16;
        c.r.p.set_zn(c.r.a);
    }
}

/// LDA #nn
pub mod opA9 {
    use super::Cpu;

    /// This func is inspired by snes9x's OpA9M0
    pub fn m0(c: &mut Cpu, _: u8) {
        c.r.a = c.imm16();
        c.r.p.set_zn(c.r.a);
    }

    /// This func is inspired by snes9x's OpA9M1
    pub fn m1(c: &mut Cpu, _: u8) {
        c.r.a = c.imm8() as u16;
        c.r.p.set_zn(c.r.a);
    }
}

/// TAX
pub mod opAA {}

/// DEC nn
pub mod opC6 {
    use super::Cpu;
    use crate::sfc::cpu::cycles;

    pub fn m0(c: &mut Cpu, _: u8) {
        let (bank, addr) = c.direct();
        let incremented = c.load16(bank, addr, Some(cycles())).wrapping_sub(1);
        c.store16(bank, addr, incremented, Some(cycles()));
        c.r.p.set_zn(incremented);
    }

    pub fn m1(c: &mut Cpu, _: u8) {
        let (bank, addr) = c.direct();
        let incremented = c.load8(bank, addr, Some(cycles())).wrapping_sub(1);
        c.store8(bank, addr, incremented, Some(cycles()));
        c.r.p.set_zn(incremented as u16);
    }
}

/// DEC nn,X
pub mod opD6 {
    use super::Cpu;
    use crate::sfc::cpu::cycles;

    pub fn m0(c: &mut Cpu, _: u8) {
        let (bank, addr) = c.direct_x();
        let incremented = c.load16(bank, addr, Some(cycles())).wrapping_sub(1);
        c.store16(bank, addr, incremented, Some(cycles()));
        c.r.p.set_zn(incremented);
    }

    pub fn m1(c: &mut Cpu, _: u8) {
        let (bank, addr) = c.direct_x();
        let incremented = c.load8(bank, addr, Some(cycles())).wrapping_sub(1);
        c.store8(bank, addr, incremented, Some(cycles()));
        c.r.p.set_zn(incremented as u16);
    }
}

/// INC nn
pub mod opE6 {
    use super::Cpu;
    use crate::sfc::cpu::cycles;

    pub fn m0(c: &mut Cpu, _: u8) {
        let (bank, addr) = c.direct();
        let incremented = c.load16(bank, addr, Some(cycles())).wrapping_add(1);
        c.store16(bank, addr, incremented, Some(cycles()));
        c.r.p.set_zn(incremented);
    }

    pub fn m1(c: &mut Cpu, _: u8) {
        let (bank, addr) = c.direct();
        let incremented = c.load8(bank, addr, Some(cycles())).wrapping_add(1);
        c.store8(bank, addr, incremented, Some(cycles()));
        c.r.p.set_zn(incremented as u16);
    }
}

/// INC nnnn
pub mod opEE {
    use super::Cpu;
    use crate::sfc::cpu::cycles;

    pub fn m0(c: &mut Cpu, _: u8) {
        let (bank, addr) = c.absolute();
        let incremented = c.load16(bank, addr, Some(cycles())).wrapping_add(1);
        c.store16(bank, addr, incremented, Some(cycles()));
        c.r.p.set_zn(incremented);
    }

    pub fn m1(c: &mut Cpu, _: u8) {
        let (bank, addr) = c.absolute();
        let incremented = c.load8(bank, addr, Some(cycles())).wrapping_add(1);
        c.store8(bank, addr, incremented, Some(cycles()));
        c.r.p.set_zn(incremented as u16);
    }
}

/// INC nnnn,X
pub mod opFE {
    use super::Cpu;
    use crate::sfc::cpu::cycles;

    pub fn m0(c: &mut Cpu, _: u8) {
        let (bank, addr) = c.absolute_x();
        let incremented = c.load16(bank, addr, Some(cycles())).wrapping_add(1);
        c.store16(bank, addr, incremented, Some(cycles()));
        c.r.p.set_zn(incremented);
    }

    pub fn m1(c: &mut Cpu, _: u8) {
        let (bank, addr) = c.absolute_x();
        let incremented = c.load8(bank, addr, Some(cycles())).wrapping_add(1);
        c.store8(bank, addr, incremented, Some(cycles()));
        c.r.p.set_zn(incremented as u16);
    }
}
