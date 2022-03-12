use super::Cpu;

/// This func is inspired by snes9x's Op00
pub fn op00(cpu: &mut Cpu, _: u8) {}

/// ORA (nn, X)
pub mod op01 {
    use super::Cpu;

    /// This func is inspired by snes9x's Op01E0M0
    pub fn e0m0(cpu: &mut Cpu, _: u8) {}

    /// This func is inspired by snes9x's Op01E0M1
    pub fn e0m1(cpu: &mut Cpu, _: u8) {}

    /// This func is inspired by snes9x's Op01E1
    pub fn e1(cpu: &mut Cpu, _: u8) {}
}

/// This func is inspired by snes9x's Op02
pub fn op02(cpu: &mut Cpu, _: u8) {}

/// ORA nn,S
/// effect: [nn+S]
pub mod op03 {
    use crate::sfc::cpu::cycles;

    use super::Cpu;

    /// This func is inspired by snes9x's Op03M0
    pub fn m0(cpu: &mut Cpu, _: u8) {
        let (bank, addr) = cpu.stack_rel();
        cpu.r.a |= cpu.load16(bank, addr, Some(cycles()));
        cpu.r.p.set_zn(cpu.r.a);
    }

    /// This func is inspired by snes9x's Op03M1
    pub fn m1(cpu: &mut Cpu, _: u8) {
        let (bank, addr) = cpu.stack_rel();
        cpu.r.a |= cpu.load8(bank, addr, Some(cycles())) as u16;
        cpu.r.p.set_zn(cpu.r.a);
    }
}

/// ORA nn
/// effect: [D+nn]
pub mod op05 {
    use crate::sfc::cpu::cycles;

    use super::Cpu;

    pub fn m0(cpu: &mut Cpu, _: u8) {
        let (bank, addr) = cpu.direct();
        cpu.r.a |= cpu.load16(bank, addr, Some(cycles()));
        cpu.r.p.set_zn(cpu.r.a);
    }

    pub fn m1(cpu: &mut Cpu, _: u8) {
        let (bank, addr) = cpu.direct();
        cpu.r.a |= cpu.load8(bank, addr, Some(cycles())) as u16;
        cpu.r.p.set_zn(cpu.r.a);
    }
}

/// ORA #nn
/// effect: nn
pub mod op09 {
    use super::Cpu;

    /// ORA #nn ; M=0
    /// This func is inspired by snes9x's Op09M0
    pub fn m0(cpu: &mut Cpu, _: u8) {
        cpu.r.a |= cpu.imm16();
        cpu.r.p.set_zn(cpu.r.a);
    }

    /// ORA #nn ; M=1
    /// This func is inspired by snes9x's Op09M1
    pub fn m1(cpu: &mut Cpu, _: u8) {
        cpu.r.a |= cpu.imm8() as u16;
        cpu.r.p.set_zn(cpu.r.a);
    }
}

/// ORA #nn
/// effect: [D+nn+X]
pub mod op15 {
    use crate::sfc::cpu::cycles;

    use super::Cpu;

    pub fn m0(cpu: &mut Cpu, _: u8) {
        let (bank, addr) = cpu.direct_x();
        cpu.r.a |= cpu.load16(bank, addr, Some(cycles()));
        cpu.r.p.set_zn(cpu.r.a);
    }

    pub fn m1(cpu: &mut Cpu, _: u8) {
        let (bank, addr) = cpu.direct_x();
        cpu.r.a |= cpu.load8(bank, addr, Some(cycles())) as u16;
        cpu.r.p.set_zn(cpu.r.a);
    }
}

/// AND #nn
pub mod op29 {
    use super::Cpu;

    /// This func is inspired by snes9x's Op29M0
    pub fn m0(cpu: &mut Cpu, _: u8) {
        cpu.r.a &= cpu.imm16();
        cpu.r.p.set_zn(cpu.r.a);
    }

    /// This func is inspired by snes9x's Op29M1
    pub fn m1(cpu: &mut Cpu, _: u8) {
        cpu.r.a &= cpu.imm8() as u16;
        cpu.r.p.set_zn(cpu.r.a);
    }
}

/// LDA #nn
mod opA9 {
    use super::Cpu;

    /// This func is inspired by snes9x's OpA9M0
    pub fn m0(cpu: &mut Cpu, _: u8) {
        cpu.r.a = cpu.imm16();
        cpu.r.p.set_zn(cpu.r.a);
    }

    /// This func is inspired by snes9x's OpA9M1
    pub fn m1(cpu: &mut Cpu, _: u8) {
        cpu.r.a = cpu.imm8() as u16;
        cpu.r.p.set_zn(cpu.r.a);
    }
}

/// TAX
mod opAA {}
