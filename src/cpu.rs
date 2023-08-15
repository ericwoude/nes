#![allow(dead_code)]

use super::bus::Bus;

enum Flag {
    C = (1 << 0), // carry bit
    Z = (1 << 1), // zero
    I = (1 << 2), // disable interrupts
    D = (1 << 3), // decimal mode
    B = (1 << 4), // break
    U = (1 << 5), // unused
    V = (1 << 6), // overflow
    N = (1 << 7), // negative`
}

struct Instruction {
    operation: fn(&mut Cpu, &mut Bus) -> usize,
    addressmode: fn(&mut Cpu, &mut Bus) -> usize,
    cycles: usize,
}

#[rustfmt::skip]
const DISPATCH: [Instruction; 256] = [
    Instruction { operation: Cpu::brk, addressmode: Cpu::imm, cycles: 7 },
    Instruction { operation: Cpu::ora, addressmode: Cpu::izx, cycles: 6 },
    Instruction { operation: Cpu::kil, addressmode: Cpu::imp, cycles: 1 },
    Instruction { operation: Cpu::slo, addressmode: Cpu::izx, cycles: 8 },
    Instruction { operation: Cpu::nop, addressmode: Cpu::zp0, cycles: 3 },
    Instruction { operation: Cpu::ora, addressmode: Cpu::zp0, cycles: 3 },
    Instruction { operation: Cpu::asl, addressmode: Cpu::zp0, cycles: 5 },
    Instruction { operation: Cpu::slo, addressmode: Cpu::zp0, cycles: 5 },
    Instruction { operation: Cpu::php, addressmode: Cpu::imp, cycles: 3 },
    Instruction { operation: Cpu::ora, addressmode: Cpu::imm, cycles: 2 },
    Instruction { operation: Cpu::asl, addressmode: Cpu::imp, cycles: 2 },
    Instruction { operation: Cpu::anc, addressmode: Cpu::imm, cycles: 2 },
    Instruction { operation: Cpu::nop, addressmode: Cpu::abs, cycles: 4 },
    Instruction { operation: Cpu::ora, addressmode: Cpu::abs, cycles: 4 },
    Instruction { operation: Cpu::asl, addressmode: Cpu::abs, cycles: 6 },
    Instruction { operation: Cpu::slo, addressmode: Cpu::abs, cycles: 6 },
    Instruction { operation: Cpu::bpl, addressmode: Cpu::rel, cycles: 2 },
    Instruction { operation: Cpu::ora, addressmode: Cpu::izy, cycles: 5 },
    Instruction { operation: Cpu::kil, addressmode: Cpu::imp, cycles: 1 },
    Instruction { operation: Cpu::slo, addressmode: Cpu::izy, cycles: 8 },
    Instruction { operation: Cpu::nop, addressmode: Cpu::zpx, cycles: 4 },
    Instruction { operation: Cpu::ora, addressmode: Cpu::zpx, cycles: 4 },
    Instruction { operation: Cpu::asl, addressmode: Cpu::zpx, cycles: 6 },
    Instruction { operation: Cpu::slo, addressmode: Cpu::zpx, cycles: 6 },
    Instruction { operation: Cpu::clc, addressmode: Cpu::imp, cycles: 2 },
    Instruction { operation: Cpu::ora, addressmode: Cpu::aby, cycles: 4 },
    Instruction { operation: Cpu::nop, addressmode: Cpu::imp, cycles: 2 },
    Instruction { operation: Cpu::slo, addressmode: Cpu::aby, cycles: 7 },
    Instruction { operation: Cpu::nop, addressmode: Cpu::abx, cycles: 4 },
    Instruction { operation: Cpu::ora, addressmode: Cpu::abx, cycles: 4 },
    Instruction { operation: Cpu::asl, addressmode: Cpu::abx, cycles: 7 },
    Instruction { operation: Cpu::slo, addressmode: Cpu::abx, cycles: 7 },
    Instruction { operation: Cpu::jsr, addressmode: Cpu::abs, cycles: 6 },
    Instruction { operation: Cpu::and, addressmode: Cpu::izx, cycles: 6 },
    Instruction { operation: Cpu::kil, addressmode: Cpu::imp, cycles: 1 },
    Instruction { operation: Cpu::rla, addressmode: Cpu::izx, cycles: 8 },
    Instruction { operation: Cpu::bit, addressmode: Cpu::zp0, cycles: 3 },
    Instruction { operation: Cpu::and, addressmode: Cpu::zp0, cycles: 3 },
    Instruction { operation: Cpu::rol, addressmode: Cpu::zp0, cycles: 5 },
    Instruction { operation: Cpu::rla, addressmode: Cpu::zp0, cycles: 5 },
    Instruction { operation: Cpu::plp, addressmode: Cpu::imp, cycles: 4 },
    Instruction { operation: Cpu::and, addressmode: Cpu::imm, cycles: 2 },
    Instruction { operation: Cpu::rol, addressmode: Cpu::imp, cycles: 2 },
    Instruction { operation: Cpu::anc, addressmode: Cpu::imm, cycles: 2 },
    Instruction { operation: Cpu::bit, addressmode: Cpu::abs, cycles: 4 },
    Instruction { operation: Cpu::and, addressmode: Cpu::abs, cycles: 4 },
    Instruction { operation: Cpu::rol, addressmode: Cpu::abs, cycles: 6 },
    Instruction { operation: Cpu::rla, addressmode: Cpu::abs, cycles: 6 },
    Instruction { operation: Cpu::bmi, addressmode: Cpu::rel, cycles: 2 },
    Instruction { operation: Cpu::and, addressmode: Cpu::izy, cycles: 5 },
    Instruction { operation: Cpu::kil, addressmode: Cpu::imp, cycles: 1 },
    Instruction { operation: Cpu::rla, addressmode: Cpu::izy, cycles: 8 },
    Instruction { operation: Cpu::nop, addressmode: Cpu::zpx, cycles: 4 },
    Instruction { operation: Cpu::and, addressmode: Cpu::zpx, cycles: 4 },
    Instruction { operation: Cpu::rol, addressmode: Cpu::zpx, cycles: 6 },
    Instruction { operation: Cpu::rla, addressmode: Cpu::zpx, cycles: 6 },
    Instruction { operation: Cpu::sec, addressmode: Cpu::imp, cycles: 2 },
    Instruction { operation: Cpu::and, addressmode: Cpu::aby, cycles: 4 },
    Instruction { operation: Cpu::nop, addressmode: Cpu::imp, cycles: 2 },
    Instruction { operation: Cpu::rla, addressmode: Cpu::aby, cycles: 7 },
    Instruction { operation: Cpu::nop, addressmode: Cpu::abx, cycles: 4 },
    Instruction { operation: Cpu::and, addressmode: Cpu::abx, cycles: 4 },
    Instruction { operation: Cpu::rol, addressmode: Cpu::abx, cycles: 7 },
    Instruction { operation: Cpu::rla, addressmode: Cpu::abx, cycles: 7 },
    Instruction { operation: Cpu::rti, addressmode: Cpu::imp, cycles: 6 },
    Instruction { operation: Cpu::eor, addressmode: Cpu::izx, cycles: 6 },
    Instruction { operation: Cpu::kil, addressmode: Cpu::imp, cycles: 1 },
    Instruction { operation: Cpu::sre, addressmode: Cpu::izx, cycles: 8 },
    Instruction { operation: Cpu::nop, addressmode: Cpu::zp0, cycles: 3 },
    Instruction { operation: Cpu::eor, addressmode: Cpu::zp0, cycles: 3 },
    Instruction { operation: Cpu::lsr, addressmode: Cpu::zp0, cycles: 5 },
    Instruction { operation: Cpu::sre, addressmode: Cpu::zp0, cycles: 5 },
    Instruction { operation: Cpu::pha, addressmode: Cpu::imp, cycles: 3 },
    Instruction { operation: Cpu::eor, addressmode: Cpu::imm, cycles: 2 },
    Instruction { operation: Cpu::lsr, addressmode: Cpu::imp, cycles: 2 },
    Instruction { operation: Cpu::alr, addressmode: Cpu::imm, cycles: 2 },
    Instruction { operation: Cpu::jmp, addressmode: Cpu::abs, cycles: 3 },
    Instruction { operation: Cpu::eor, addressmode: Cpu::abs, cycles: 4 },
    Instruction { operation: Cpu::lsr, addressmode: Cpu::abs, cycles: 6 },
    Instruction { operation: Cpu::sre, addressmode: Cpu::abs, cycles: 6 },
    Instruction { operation: Cpu::bvc, addressmode: Cpu::rel, cycles: 2 },
    Instruction { operation: Cpu::eor, addressmode: Cpu::izy, cycles: 5 },
    Instruction { operation: Cpu::kil, addressmode: Cpu::imp, cycles: 1 },
    Instruction { operation: Cpu::sre, addressmode: Cpu::izy, cycles: 8 },
    Instruction { operation: Cpu::nop, addressmode: Cpu::zpx, cycles: 4 },
    Instruction { operation: Cpu::eor, addressmode: Cpu::zpx, cycles: 4 },
    Instruction { operation: Cpu::lsr, addressmode: Cpu::zpx, cycles: 6 },
    Instruction { operation: Cpu::sre, addressmode: Cpu::zpx, cycles: 6 },
    Instruction { operation: Cpu::cli, addressmode: Cpu::imp, cycles: 2 },
    Instruction { operation: Cpu::eor, addressmode: Cpu::aby, cycles: 4 },
    Instruction { operation: Cpu::nop, addressmode: Cpu::imp, cycles: 2 },
    Instruction { operation: Cpu::sre, addressmode: Cpu::aby, cycles: 7 },
    Instruction { operation: Cpu::nop, addressmode: Cpu::abx, cycles: 4 },
    Instruction { operation: Cpu::eor, addressmode: Cpu::abx, cycles: 4 },
    Instruction { operation: Cpu::lsr, addressmode: Cpu::abx, cycles: 7 },
    Instruction { operation: Cpu::sre, addressmode: Cpu::abx, cycles: 7 },
    Instruction { operation: Cpu::rts, addressmode: Cpu::imp, cycles: 6 },
    Instruction { operation: Cpu::adc, addressmode: Cpu::izx, cycles: 6 },
    Instruction { operation: Cpu::kil, addressmode: Cpu::imp, cycles: 1 },
    Instruction { operation: Cpu::rra, addressmode: Cpu::izx, cycles: 8 },
    Instruction { operation: Cpu::nop, addressmode: Cpu::zp0, cycles: 3 },
    Instruction { operation: Cpu::adc, addressmode: Cpu::zp0, cycles: 3 },
    Instruction { operation: Cpu::ror, addressmode: Cpu::zp0, cycles: 5 },
    Instruction { operation: Cpu::rra, addressmode: Cpu::zp0, cycles: 5 },
    Instruction { operation: Cpu::pla, addressmode: Cpu::imp, cycles: 4 },
    Instruction { operation: Cpu::adc, addressmode: Cpu::imm, cycles: 2 },
    Instruction { operation: Cpu::ror, addressmode: Cpu::imp, cycles: 2 },
    Instruction { operation: Cpu::arr, addressmode: Cpu::imm, cycles: 2 },
    Instruction { operation: Cpu::jmp, addressmode: Cpu::ind, cycles: 5 },
    Instruction { operation: Cpu::adc, addressmode: Cpu::abs, cycles: 4 },
    Instruction { operation: Cpu::ror, addressmode: Cpu::abs, cycles: 6 },
    Instruction { operation: Cpu::rra, addressmode: Cpu::abs, cycles: 6 },
    Instruction { operation: Cpu::bvs, addressmode: Cpu::rel, cycles: 2 },
    Instruction { operation: Cpu::adc, addressmode: Cpu::izy, cycles: 5 },
    Instruction { operation: Cpu::kil, addressmode: Cpu::imp, cycles: 1 },
    Instruction { operation: Cpu::rra, addressmode: Cpu::izy, cycles: 8 },
    Instruction { operation: Cpu::nop, addressmode: Cpu::zpx, cycles: 4 },
    Instruction { operation: Cpu::adc, addressmode: Cpu::zpx, cycles: 4 },
    Instruction { operation: Cpu::ror, addressmode: Cpu::zpx, cycles: 6 },
    Instruction { operation: Cpu::rra, addressmode: Cpu::zpx, cycles: 6 },
    Instruction { operation: Cpu::sei, addressmode: Cpu::imp, cycles: 2 },
    Instruction { operation: Cpu::adc, addressmode: Cpu::aby, cycles: 4 },
    Instruction { operation: Cpu::nop, addressmode: Cpu::imp, cycles: 2 },
    Instruction { operation: Cpu::rra, addressmode: Cpu::aby, cycles: 7 },
    Instruction { operation: Cpu::nop, addressmode: Cpu::abx, cycles: 4 },
    Instruction { operation: Cpu::adc, addressmode: Cpu::abx, cycles: 4 },
    Instruction { operation: Cpu::ror, addressmode: Cpu::abx, cycles: 7 },
    Instruction { operation: Cpu::rra, addressmode: Cpu::abx, cycles: 7 },
    Instruction { operation: Cpu::nop, addressmode: Cpu::imm, cycles: 2 },
    Instruction { operation: Cpu::sta, addressmode: Cpu::izx, cycles: 6 },
    Instruction { operation: Cpu::nop, addressmode: Cpu::imm, cycles: 2 },
    Instruction { operation: Cpu::sax, addressmode: Cpu::izx, cycles: 6 },
    Instruction { operation: Cpu::sty, addressmode: Cpu::zp0, cycles: 3 },
    Instruction { operation: Cpu::sta, addressmode: Cpu::zp0, cycles: 3 },
    Instruction { operation: Cpu::stx, addressmode: Cpu::zp0, cycles: 3 },
    Instruction { operation: Cpu::sax, addressmode: Cpu::zp0, cycles: 3 },
    Instruction { operation: Cpu::dey, addressmode: Cpu::imp, cycles: 2 },
    Instruction { operation: Cpu::nop, addressmode: Cpu::imm, cycles: 2 },
    Instruction { operation: Cpu::txa, addressmode: Cpu::imp, cycles: 2 },
    Instruction { operation: Cpu::xaa, addressmode: Cpu::imm, cycles: 2 },
    Instruction { operation: Cpu::sty, addressmode: Cpu::abs, cycles: 4 },
    Instruction { operation: Cpu::sta, addressmode: Cpu::abs, cycles: 4 },
    Instruction { operation: Cpu::stx, addressmode: Cpu::abs, cycles: 4 },
    Instruction { operation: Cpu::sax, addressmode: Cpu::abs, cycles: 4 },
    Instruction { operation: Cpu::bcc, addressmode: Cpu::rel, cycles: 2 },
    Instruction { operation: Cpu::sta, addressmode: Cpu::izy, cycles: 6 },
    Instruction { operation: Cpu::kil, addressmode: Cpu::imp, cycles: 1 },
    Instruction { operation: Cpu::sha, addressmode: Cpu::izy, cycles: 6 },
    Instruction { operation: Cpu::sty, addressmode: Cpu::zpx, cycles: 4 },
    Instruction { operation: Cpu::sta, addressmode: Cpu::zpx, cycles: 4 },
    Instruction { operation: Cpu::stx, addressmode: Cpu::zpy, cycles: 4 },
    Instruction { operation: Cpu::sax, addressmode: Cpu::zpy, cycles: 4 },
    Instruction { operation: Cpu::tya, addressmode: Cpu::imp, cycles: 2 },
    Instruction { operation: Cpu::sta, addressmode: Cpu::aby, cycles: 5 },
    Instruction { operation: Cpu::txs, addressmode: Cpu::imp, cycles: 2 },
    Instruction { operation: Cpu::tas, addressmode: Cpu::aby, cycles: 5 },
    Instruction { operation: Cpu::shy, addressmode: Cpu::abx, cycles: 5 },
    Instruction { operation: Cpu::sta, addressmode: Cpu::abx, cycles: 5 },
    Instruction { operation: Cpu::shx, addressmode: Cpu::aby, cycles: 5 },
    Instruction { operation: Cpu::sha, addressmode: Cpu::aby, cycles: 5 },
    Instruction { operation: Cpu::ldy, addressmode: Cpu::imm, cycles: 2 },
    Instruction { operation: Cpu::lda, addressmode: Cpu::izx, cycles: 6 },
    Instruction { operation: Cpu::ldx, addressmode: Cpu::imm, cycles: 2 },
    Instruction { operation: Cpu::lax, addressmode: Cpu::izx, cycles: 6 },
    Instruction { operation: Cpu::ldy, addressmode: Cpu::zp0, cycles: 3 },
    Instruction { operation: Cpu::lda, addressmode: Cpu::zp0, cycles: 3 },
    Instruction { operation: Cpu::ldx, addressmode: Cpu::zp0, cycles: 3 },
    Instruction { operation: Cpu::lax, addressmode: Cpu::zp0, cycles: 3 },
    Instruction { operation: Cpu::tay, addressmode: Cpu::imp, cycles: 2 },
    Instruction { operation: Cpu::lda, addressmode: Cpu::imm, cycles: 2 },
    Instruction { operation: Cpu::tax, addressmode: Cpu::imp, cycles: 2 },
    Instruction { operation: Cpu::lxa, addressmode: Cpu::imm, cycles: 2 },
    Instruction { operation: Cpu::ldy, addressmode: Cpu::abs, cycles: 4 },
    Instruction { operation: Cpu::lda, addressmode: Cpu::abs, cycles: 4 },
    Instruction { operation: Cpu::ldx, addressmode: Cpu::abs, cycles: 4 },
    Instruction { operation: Cpu::lax, addressmode: Cpu::abs, cycles: 4 },
    Instruction { operation: Cpu::bcs, addressmode: Cpu::rel, cycles: 2 },
    Instruction { operation: Cpu::lda, addressmode: Cpu::izy, cycles: 5 },
    Instruction { operation: Cpu::kil, addressmode: Cpu::imp, cycles: 1 },
    Instruction { operation: Cpu::lax, addressmode: Cpu::izy, cycles: 5 },
    Instruction { operation: Cpu::ldy, addressmode: Cpu::zpx, cycles: 4 },
    Instruction { operation: Cpu::lda, addressmode: Cpu::zpx, cycles: 4 },
    Instruction { operation: Cpu::ldx, addressmode: Cpu::zpy, cycles: 4 },
    Instruction { operation: Cpu::lax, addressmode: Cpu::zpy, cycles: 4 },
    Instruction { operation: Cpu::clv, addressmode: Cpu::imp, cycles: 2 },
    Instruction { operation: Cpu::lda, addressmode: Cpu::aby, cycles: 4 },
    Instruction { operation: Cpu::tsx, addressmode: Cpu::imp, cycles: 2 },
    Instruction { operation: Cpu::las, addressmode: Cpu::aby, cycles: 4 },
    Instruction { operation: Cpu::ldy, addressmode: Cpu::abx, cycles: 4 },
    Instruction { operation: Cpu::lda, addressmode: Cpu::abx, cycles: 4 },
    Instruction { operation: Cpu::ldx, addressmode: Cpu::aby, cycles: 4 },
    Instruction { operation: Cpu::lax, addressmode: Cpu::aby, cycles: 4 },
    Instruction { operation: Cpu::cpy, addressmode: Cpu::imm, cycles: 2 },
    Instruction { operation: Cpu::cmp, addressmode: Cpu::izx, cycles: 6 },
    Instruction { operation: Cpu::nop, addressmode: Cpu::imm, cycles: 2 },
    Instruction { operation: Cpu::dcp, addressmode: Cpu::izx, cycles: 8 },
    Instruction { operation: Cpu::cpy, addressmode: Cpu::zp0, cycles: 3 },
    Instruction { operation: Cpu::cmp, addressmode: Cpu::zp0, cycles: 3 },
    Instruction { operation: Cpu::dec, addressmode: Cpu::zp0, cycles: 5 },
    Instruction { operation: Cpu::dcp, addressmode: Cpu::zp0, cycles: 5 },
    Instruction { operation: Cpu::iny, addressmode: Cpu::imp, cycles: 2 },
    Instruction { operation: Cpu::cmp, addressmode: Cpu::imm, cycles: 2 },
    Instruction { operation: Cpu::dex, addressmode: Cpu::imp, cycles: 2 },
    Instruction { operation: Cpu::axs, addressmode: Cpu::imm, cycles: 2 },
    Instruction { operation: Cpu::cpy, addressmode: Cpu::abs, cycles: 4 },
    Instruction { operation: Cpu::cmp, addressmode: Cpu::abs, cycles: 4 },
    Instruction { operation: Cpu::dec, addressmode: Cpu::abs, cycles: 6 },
    Instruction { operation: Cpu::dcp, addressmode: Cpu::abs, cycles: 6 },
    Instruction { operation: Cpu::bne, addressmode: Cpu::rel, cycles: 2 },
    Instruction { operation: Cpu::cmp, addressmode: Cpu::izy, cycles: 5 },
    Instruction { operation: Cpu::kil, addressmode: Cpu::imp, cycles: 1 },
    Instruction { operation: Cpu::dcp, addressmode: Cpu::izy, cycles: 8 },
    Instruction { operation: Cpu::nop, addressmode: Cpu::zpx, cycles: 4 },
    Instruction { operation: Cpu::cmp, addressmode: Cpu::zpx, cycles: 4 },
    Instruction { operation: Cpu::dec, addressmode: Cpu::zpx, cycles: 6 },
    Instruction { operation: Cpu::dcp, addressmode: Cpu::zpx, cycles: 6 },
    Instruction { operation: Cpu::cld, addressmode: Cpu::imp, cycles: 2 },
    Instruction { operation: Cpu::cmp, addressmode: Cpu::aby, cycles: 4 },
    Instruction { operation: Cpu::nop, addressmode: Cpu::imp, cycles: 2 },
    Instruction { operation: Cpu::dcp, addressmode: Cpu::aby, cycles: 7 },
    Instruction { operation: Cpu::nop, addressmode: Cpu::abx, cycles: 4 },
    Instruction { operation: Cpu::cmp, addressmode: Cpu::abx, cycles: 4 },
    Instruction { operation: Cpu::dec, addressmode: Cpu::abx, cycles: 7 },
    Instruction { operation: Cpu::dcp, addressmode: Cpu::abx, cycles: 7 },
    Instruction { operation: Cpu::cpx, addressmode: Cpu::imm, cycles: 2 },
    Instruction { operation: Cpu::sbc, addressmode: Cpu::izx, cycles: 6 },
    Instruction { operation: Cpu::nop, addressmode: Cpu::imm, cycles: 2 },
    Instruction { operation: Cpu::isc, addressmode: Cpu::izx, cycles: 8 },
    Instruction { operation: Cpu::cpx, addressmode: Cpu::zp0, cycles: 3 },
    Instruction { operation: Cpu::sbc, addressmode: Cpu::zp0, cycles: 3 },
    Instruction { operation: Cpu::inc, addressmode: Cpu::zp0, cycles: 5 },
    Instruction { operation: Cpu::isc, addressmode: Cpu::zp0, cycles: 5 },
    Instruction { operation: Cpu::inx, addressmode: Cpu::imp, cycles: 2 },
    Instruction { operation: Cpu::sbc, addressmode: Cpu::imm, cycles: 2 },
    Instruction { operation: Cpu::nop, addressmode: Cpu::imp, cycles: 2 },
    Instruction { operation: Cpu::sbc, addressmode: Cpu::imm, cycles: 2 },
    Instruction { operation: Cpu::cpx, addressmode: Cpu::abs, cycles: 4 },
    Instruction { operation: Cpu::sbc, addressmode: Cpu::abs, cycles: 4 },
    Instruction { operation: Cpu::inc, addressmode: Cpu::abs, cycles: 6 },
    Instruction { operation: Cpu::isc, addressmode: Cpu::abs, cycles: 6 },
    Instruction { operation: Cpu::beq, addressmode: Cpu::rel, cycles: 2 },
    Instruction { operation: Cpu::sbc, addressmode: Cpu::izy, cycles: 5 },
    Instruction { operation: Cpu::kil, addressmode: Cpu::imp, cycles: 1 },
    Instruction { operation: Cpu::isc, addressmode: Cpu::izy, cycles: 8 },
    Instruction { operation: Cpu::nop, addressmode: Cpu::zpx, cycles: 4 },
    Instruction { operation: Cpu::sbc, addressmode: Cpu::zpx, cycles: 4 },
    Instruction { operation: Cpu::inc, addressmode: Cpu::zpx, cycles: 6 },
    Instruction { operation: Cpu::isc, addressmode: Cpu::zpx, cycles: 6 },
    Instruction { operation: Cpu::sed, addressmode: Cpu::imp, cycles: 2 },
    Instruction { operation: Cpu::sbc, addressmode: Cpu::aby, cycles: 4 },
    Instruction { operation: Cpu::nop, addressmode: Cpu::imp, cycles: 2 },
    Instruction { operation: Cpu::isc, addressmode: Cpu::aby, cycles: 7 },
    Instruction { operation: Cpu::nop, addressmode: Cpu::abx, cycles: 4 },
    Instruction { operation: Cpu::sbc, addressmode: Cpu::abx, cycles: 4 },
    Instruction { operation: Cpu::inc, addressmode: Cpu::abx, cycles: 7 },
    Instruction { operation: Cpu::isc, addressmode: Cpu::abx, cycles: 7 },
];

pub struct Cpu {
    pub a: u8,
    pub x: u8,
    pub y: u8,
    pub sp: u8,
    pub pc: u16,
    pub status: u8,

    pub fetched: u8,
    pub addr_abs: u16,
    pub addr_rel: u16,
    pub opcode: u8,
    pub cycles: usize,
}

impl Default for Cpu {
    fn default() -> Self {
        Self {
            a: 0x00,
            x: 0x00,
            y: 0x00,
            sp: 0xFD,
            pc: 0x0000,
            status: 0x34,

            fetched: 0x00,
            addr_abs: 0x0000,
            addr_rel: 0x0000,
            opcode: 0x00,
            cycles: 0,
        }
    }
}

impl Cpu {
    fn set_flag(&mut self, flag: Flag, value: bool) {
        match value {
            true => self.status |= flag as u8,
            false => self.status &= !(flag as u8),
        }
    }

    fn get_flag(&self, flag: Flag) -> bool {
        self.status & (flag as u8) != 0
    }

    pub fn clock(&mut self, bus: &mut Bus) {
        if self.cycles == 0 {
            self.opcode = bus.read(self.pc);
            self.pc = self.pc.wrapping_add(1);
            self.set_flag(Flag::U, true);

            let instruction = &DISPATCH[self.opcode as usize];

            self.cycles = instruction.cycles;
            let addr_cycles = (instruction.addressmode)(self, bus);
            let op_cycles = (instruction.operation)(self, bus);

            self.cycles += addr_cycles & op_cycles;

            self.set_flag(Flag::U, true);
        }

        self.cycles -= 1;
    }

    pub fn complete(&self) -> bool {
        self.cycles == 0
    }

    pub fn reset(&mut self) {
        self.sp = self.sp.wrapping_sub(3);
        self.set_flag(Flag::I, true);
    }

    fn irq(&mut self, bus: &mut Bus) {
        if !self.get_flag(Flag::I) {
            self.nmi(bus);
        }
    }

    fn nmi(&mut self, bus: &mut Bus) {
        bus.write(0x0100 + self.sp as u16, ((self.pc >> 8) & 0x00FF) as u8);
        self.sp = self.sp.wrapping_sub(1);
        bus.write(0x0100 + self.sp as u16, (self.pc & 0x00FF) as u8);
        self.sp = self.sp.wrapping_sub(1);

        self.set_flag(Flag::B, false);
        self.set_flag(Flag::U, true);
        self.set_flag(Flag::I, true);
        bus.write(0x0100 + self.sp as u16, self.status);
        self.sp = self.sp.wrapping_sub(1);

        self.addr_abs = 0xFFFE;
        let lo: u16 = bus.read(self.addr_abs) as u16;
        let hi: u16 = bus.read(self.addr_abs + 1) as u16;
        self.pc = (hi << 8) | lo;

        self.cycles = 7;
    }

    // ADDRESSING FUNCTIONS

    /// The addressing is implied in the opcode.
    fn imp(&mut self, _: &mut Bus) -> usize {
        self.fetched = self.a;

        0
    }

    /// The address is supplied as part of the instruction.
    fn imm(&mut self, _: &mut Bus) -> usize {
        self.addr_abs = self.pc;
        self.pc = self.pc.wrapping_add(1);

        0
    }

    /// Zero page addressing uses the high byte to address
    /// a specific page and the low byte to offset into that page
    fn zp0(&mut self, bus: &mut Bus) -> usize {
        self.addr_abs = bus.read(self.pc) as u16;
        self.pc = self.pc.wrapping_add(1);

        self.addr_abs &= 0x00FF;

        0
    }

    /// Zero page addressing with register x as extra offset.
    fn zpx(&mut self, bus: &mut Bus) -> usize {
        self.addr_abs = bus.read(self.pc) as u16 + self.x as u16;
        self.pc = self.pc.wrapping_add(1);
        self.addr_abs &= 0x00FF;

        0
    }

    /// Zero page addressing with register y as extra offset.
    fn zpy(&mut self, bus: &mut Bus) -> usize {
        self.addr_abs = bus.read(self.pc) as u16 + self.y as u16;
        self.pc = self.pc.wrapping_add(1);
        self.addr_abs &= 0x00FF;

        0
    }

    /// Relative addressing uses the second byte (signed) as an offset for the next
    /// instruction, which can range from -127 to +127 relative to the program
    /// counter.
    fn rel(&mut self, bus: &mut Bus) -> usize {
        self.addr_rel = bus.read(self.pc) as u16;
        self.pc = self.pc.wrapping_add(1);

        if (self.addr_rel & 0b10000000) > 0 {
            self.addr_rel |= 0xFF00;
        }

        0
    }

    /// Absolute addressing fetches the full 16 bit address
    /// from region in memory at the program counter.
    fn abs(&mut self, bus: &mut Bus) -> usize {
        let lo: u16 = bus.read(self.pc) as u16;
        self.pc = self.pc.wrapping_add(1);
        let hi: u16 = bus.read(self.pc) as u16;
        self.pc = self.pc.wrapping_add(1);

        self.addr_abs = (hi << 8) | lo;

        0
    }

    /// Absolute addressing with x offset fetches a memory address
    /// like regular absolute addressing, but offsets the memory with
    /// the contents of register x.
    ///
    /// The paging can overflow, in which case the 6502 requires an
    /// extra cycle to fetch the address. Therefore we return 1 if an
    /// overflow has occured.
    fn abx(&mut self, bus: &mut Bus) -> usize {
        let lo: u16 = bus.read(self.pc) as u16;
        self.pc = self.pc.wrapping_add(1);
        let hi: u16 = bus.read(self.pc) as u16;
        self.pc = self.pc.wrapping_add(1);

        self.addr_abs = ((hi << 8) | lo).wrapping_add(self.x as u16);

        if (self.addr_abs & 0xFF00) != (hi << 8) {
            1
        } else {
            0
        }
    }

    /// Absolute addressing with y offset.
    fn aby(&mut self, bus: &mut Bus) -> usize {
        let lo: u16 = bus.read(self.pc) as u16;
        self.pc = self.pc.wrapping_add(1);
        let hi: u16 = bus.read(self.pc) as u16;
        self.pc = self.pc.wrapping_add(1);

        self.addr_abs = ((hi << 8) | lo).wrapping_add(self.y as u16);

        if (self.addr_abs & 0xFF00) != (hi << 8) {
            1
        } else {
            0
        }
    }

    /// Indirect addressing first fetches a 16 bit pointer `target`
    /// It will use the target location in memory to fetch the effective
    /// target (actual address).
    ///
    /// Simulates boundary hardware bug of page overload.
    fn ind(&mut self, bus: &mut Bus) -> usize {
        let lo: u16 = bus.read(self.pc) as u16;
        self.pc = self.pc.wrapping_add(1);
        let hi: u16 = bus.read(self.pc) as u16;
        self.pc = self.pc.wrapping_add(1);
        let target = hi.wrapping_shl(8) | lo;

        self.addr_abs = if lo == 0x00FF {
            let effective_lo = bus.read(target) as u16;
            let effective_hi = bus.read(target & 0xFF00) as u16;

            effective_hi.wrapping_shl(8) | effective_lo
        } else {
            let effective_lo = bus.read(target) as u16;
            let effective_hi = bus.read(target + 1) as u16;

            effective_hi.wrapping_shl(8) | effective_lo
        };

        0
    }

    /// Zero page indirect addressing with register x offset.
    fn izx(&mut self, bus: &mut Bus) -> usize {
        let p: u16 = bus.read(self.pc) as u16;
        self.pc = self.pc.wrapping_add(1);

        let lo: u16 = bus.read((p + self.x as u16) & 0x00FF) as u16;
        let hi: u16 = bus.read((p + (self.x as u16) + 1) & 0x00FF) as u16;

        self.addr_abs = (hi << 8) | lo;

        0
    }

    /// Zero page indirect addressing with register y offset. Different from izx,
    /// the register y offset is added onto the fetched 16 bits from memory location.
    /// It may overflow into the next page, requiring an extra cpu cycle to complete.
    fn izy(&mut self, bus: &mut Bus) -> usize {
        let t: u16 = bus.read(self.pc) as u16;
        self.pc = self.pc.wrapping_add(1);

        let lo = bus.read(t & 0x00FF) as u16;
        let hi = bus.read((t + 1) & 0x00FF) as u16;

        self.addr_abs = ((hi << 8) | lo).wrapping_add(self.y as u16);

        if (self.addr_abs & 0xFF00) != (hi << 8) {
            1
        } else {
            0
        }
    }

    // OPCODE FUNCTIONS

    fn fetch(&mut self, bus: &Bus) -> u8 {
        let instruction = &DISPATCH[self.opcode as usize];
        if instruction.addressmode as usize != Cpu::imp as usize {
            self.fetched = bus.read(self.addr_abs);
        }

        self.fetched
    }

    fn conditional_branch(&mut self, flag: Flag, status: bool) {
        if self.get_flag(flag) == status {
            self.cycles += 1;

            self.addr_abs = self.pc.wrapping_add(self.addr_rel);

            if (self.addr_abs & 0xFF00) != self.pc & 0xFF00 {
                self.cycles += 1;
            }

            self.pc = self.addr_abs;
        }
    }

    fn brk(&mut self, bus: &mut Bus) -> usize {
        bus.write(0x0100 + self.sp as u16, ((self.pc >> 8) & 0x00FF) as u8);
        self.sp = self.sp.wrapping_sub(1);
        bus.write(0x0100 + self.sp as u16, (self.pc & 0x00FF) as u8);
        self.sp = self.sp.wrapping_sub(1);

        self.set_flag(Flag::B, true);
        bus.write(0x0100 + self.sp as u16, self.status);
        self.sp = self.sp.wrapping_sub(1);
        self.set_flag(Flag::B, false);

        // self.pc = bus.read(0xFFFE) as u16 | (bus.read(0xFFFF as u16) as u16) << 8;
        let lo = bus.read(0xFFFEu16) as u16;
        let hi = bus.read(0xFFFFu16) as u16;
        let addr = hi << 8 | lo;
        self.pc = addr;
        self.set_flag(Flag::I, true);
        0
    }

    fn ora(&mut self, bus: &mut Bus) -> usize {
        self.fetch(bus);

        self.a |= self.fetched;

        self.set_flag(Flag::Z, self.a == 0);
        self.set_flag(Flag::N, (self.a & 0b10000000) > 0);

        0
    }

    fn asl(&mut self, bus: &mut Bus) -> usize {
        self.fetch(bus);
        self.set_flag(Flag::C, (self.fetched & 0b10000000) > 0);

        let t: u8 = self.fetched.wrapping_shl(1);

        self.set_flag(Flag::Z, t == 0);
        self.set_flag(Flag::N, (t & 0b10000000) > 0);

        let instruction = &DISPATCH[self.opcode as usize];
        if (instruction.addressmode as usize) == (Cpu::imp as usize) {
            self.a = t;
        } else {
            bus.write(self.addr_abs, t);
        }

        0
    }

    fn php(&mut self, bus: &mut Bus) -> usize {
        bus.write(
            0x0100 + self.sp as u16,
            self.status | Flag::B as u8 | Flag::U as u8,
        );
        self.sp = self.sp.wrapping_sub(1);

        self.set_flag(Flag::B, false);
        self.set_flag(Flag::U, false);

        0
    }

    fn bpl(&mut self, _: &mut Bus) -> usize {
        self.conditional_branch(Flag::N, false);

        0
    }

    fn clc(&mut self, _: &mut Bus) -> usize {
        self.set_flag(Flag::C, false);

        0
    }

    fn jsr(&mut self, bus: &mut Bus) -> usize {
        self.pc = self.pc.wrapping_sub(1);

        bus.write(
            0x0100 + self.sp as u16,
            (self.pc.wrapping_shr(8) & 0x00FF) as u8,
        );
        self.sp = self.sp.wrapping_sub(1);
        bus.write(0x0100 + self.sp as u16, (self.pc & 0x00FF) as u8);
        self.sp = self.sp.wrapping_sub(1);

        self.pc = self.addr_abs;

        0
    }

    fn and(&mut self, bus: &mut Bus) -> usize {
        self.fetch(bus);
        self.a &= self.fetched;

        self.set_flag(Flag::Z, self.a == 0x00);
        self.set_flag(Flag::N, (self.a & 0b10000000) > 0);

        1
    }

    fn bit(&mut self, bus: &mut Bus) -> usize {
        self.fetch(bus);

        self.set_flag(Flag::Z, (self.a & self.fetched) == 0);
        self.set_flag(Flag::N, self.fetched & (1 << 7) > 0);
        self.set_flag(Flag::V, self.fetched & (1 << 6) > 0);

        0
    }

    fn rol(&mut self, bus: &mut Bus) -> usize {
        self.fetch(bus);

        let overflow = (self.fetched & 0b10000000) > 0;
        let operand = self.fetched.wrapping_shl(1) | self.get_flag(Flag::C) as u8;

        self.set_flag(Flag::C, overflow);
        self.set_flag(Flag::N, (operand & 0b10000000) > 0);
        self.set_flag(Flag::Z, operand == 0);

        let instruction = &DISPATCH[self.opcode as usize];
        if (instruction.addressmode as usize) == (Cpu::imp as usize) {
            self.a = operand;
        } else {
            bus.write(self.addr_abs, operand);
        }

        0
    }

    fn plp(&mut self, bus: &mut Bus) -> usize {
        self.sp = self.sp.wrapping_add(1);
        self.status = bus.read(0x0100 + self.sp as u16);

        self.set_flag(Flag::U, true);
        self.set_flag(Flag::B, false);

        0
    }

    fn bmi(&mut self, _: &mut Bus) -> usize {
        self.conditional_branch(Flag::N, true);

        0
    }

    fn sec(&mut self, _: &mut Bus) -> usize {
        self.set_flag(Flag::C, true);

        1
    }

    fn rti(&mut self, bus: &mut Bus) -> usize {
        self.sp = self.sp.wrapping_add(1);
        self.status = bus.read(0x0100 + self.sp as u16);
        self.status &= !(Flag::B as u8);
        self.status &= !(Flag::U as u8);

        self.sp = self.sp.wrapping_add(1);
        self.pc = bus.read(0x0100 + self.sp as u16) as u16;
        self.sp = self.sp.wrapping_add(1);
        self.pc |= (bus.read(0x0100 + self.sp as u16) as u16).wrapping_shl(8);

        0
    }

    fn eor(&mut self, bus: &mut Bus) -> usize {
        self.fetch(bus);

        self.a ^= self.fetched;

        self.set_flag(Flag::Z, self.a == 0);
        self.set_flag(Flag::N, (self.a & 0b10000000) > 1);

        0
    }

    fn lsr(&mut self, bus: &mut Bus) -> usize {
        self.fetch(bus);
        self.set_flag(Flag::C, (self.fetched & 0x0001) > 0);

        let t: u8 = self.fetched.wrapping_shr(1);

        self.set_flag(Flag::Z, t == 0);
        self.set_flag(Flag::N, (t & 0b10000000) > 1);

        let instruction = &DISPATCH[self.opcode as usize];
        if (instruction.addressmode as usize) == (Cpu::imp as usize) {
            self.a = t;
        } else {
            bus.write(self.addr_abs, t);
        }

        0
    }

    fn pha(&mut self, bus: &mut Bus) -> usize {
        bus.write(0x0100 + self.sp as u16, self.a);
        self.sp = self.sp.wrapping_sub(1);

        0
    }

    fn jmp(&mut self, _: &mut Bus) -> usize {
        self.pc = self.addr_abs;

        0
    }

    fn bvc(&mut self, _: &mut Bus) -> usize {
        self.conditional_branch(Flag::V, false);

        0
    }

    fn cli(&mut self, _: &mut Bus) -> usize {
        self.set_flag(Flag::I, false);

        0
    }

    fn rts(&mut self, bus: &mut Bus) -> usize {
        self.sp = self.sp.wrapping_add(1);
        let lo: u16 = bus.read(0x0100 + self.sp as u16) as u16;
        self.sp = self.sp.wrapping_add(1);
        let hi: u16 = bus.read(0x0100 + self.sp as u16) as u16;

        self.pc = (hi.wrapping_shl(8) | lo).wrapping_add(1);

        0
    }

    fn adc(&mut self, bus: &mut Bus) -> usize {
        self.fetch(bus);

        let result: u16 = self.a as u16 + self.fetched as u16 + self.get_flag(Flag::C) as u16;
        let overflow =
            (!((self.a as u16) ^ (self.fetched as u16))) & ((self.a as u16) ^ result) & 0x80;

        self.set_flag(Flag::C, result > 0xFF);
        self.set_flag(Flag::V, overflow > 0);

        self.a = result as u8;

        self.set_flag(Flag::Z, self.a == 0);
        self.set_flag(Flag::N, self.a & 0b10000000 > 0);

        1
    }

    fn ror(&mut self, bus: &mut Bus) -> usize {
        self.fetch(bus);
        let t = ((self.get_flag(Flag::C) as u8) << 7) as u16 | self.fetched.wrapping_shr(1) as u16;

        self.set_flag(Flag::C, (self.fetched & 0x01) > 0);
        self.set_flag(Flag::Z, (t & 0x00FF) == 0x00);
        self.set_flag(Flag::N, (t & 0x0080) > 0);

        let instruction = &DISPATCH[self.opcode as usize];
        if (instruction.addressmode as usize) == (Cpu::imp as usize) {
            self.a = (t & 0x00FF) as u8;
        } else {
            bus.write(self.addr_abs, (t & 0x00FF) as u8);
        }

        0
    }

    fn pla(&mut self, bus: &mut Bus) -> usize {
        self.sp = self.sp.wrapping_add(1);
        self.a = bus.read(0x0100 + self.sp as u16);

        self.set_flag(Flag::Z, self.a == 0);
        self.set_flag(Flag::N, (self.a & 0b10000000) > 0);

        0
    }

    fn bvs(&mut self, _: &mut Bus) -> usize {
        self.conditional_branch(Flag::V, true);

        0
    }

    fn sei(&mut self, _: &mut Bus) -> usize {
        self.set_flag(Flag::I, true);

        0
    }

    fn sta(&mut self, bus: &mut Bus) -> usize {
        bus.write(self.addr_abs, self.a);

        0
    }

    fn sty(&mut self, bus: &mut Bus) -> usize {
        bus.write(self.addr_abs, self.y);

        0
    }

    fn stx(&mut self, bus: &mut Bus) -> usize {
        bus.write(self.addr_abs, self.x);

        0
    }

    fn dey(&mut self, _: &mut Bus) -> usize {
        self.y = self.y.wrapping_sub(1);

        self.set_flag(Flag::Z, self.y == 0);
        self.set_flag(Flag::N, (self.y & 0b10000000) > 0);

        0
    }

    fn txa(&mut self, _: &mut Bus) -> usize {
        self.a = self.x;

        self.set_flag(Flag::Z, self.a == 0);
        self.set_flag(Flag::N, (self.a & 0b10000000) > 0);

        0
    }

    fn bcc(&mut self, _: &mut Bus) -> usize {
        self.conditional_branch(Flag::C, false);

        0
    }

    fn tya(&mut self, _: &mut Bus) -> usize {
        self.a = self.y;

        self.set_flag(Flag::Z, self.a == 0);
        self.set_flag(Flag::N, (self.a & 0b10000000) > 0);

        0
    }

    fn txs(&mut self, _: &mut Bus) -> usize {
        self.sp = self.x;

        0
    }

    fn ldy(&mut self, bus: &mut Bus) -> usize {
        self.fetch(bus);
        self.y = self.fetched;

        self.set_flag(Flag::Z, self.y == 0);
        self.set_flag(Flag::N, (self.y & 0b10000000) > 0);

        1
    }

    fn lda(&mut self, bus: &mut Bus) -> usize {
        self.fetch(bus);
        self.a = self.fetched;

        self.set_flag(Flag::Z, self.a == 0);
        self.set_flag(Flag::N, (self.a & 0b10000000) > 0);

        1
    }

    fn ldx(&mut self, bus: &mut Bus) -> usize {
        self.fetch(bus);
        self.x = self.fetched;

        self.set_flag(Flag::Z, self.x == 0);
        self.set_flag(Flag::N, (self.x & 0b10000000) > 0);

        1
    }

    fn tay(&mut self, _: &mut Bus) -> usize {
        self.y = self.a;

        self.set_flag(Flag::Z, self.y == 0);
        self.set_flag(Flag::N, (self.y & 0b10000000) > 0);

        0
    }

    fn tax(&mut self, _: &mut Bus) -> usize {
        self.x = self.a;

        self.set_flag(Flag::Z, self.x == 0);
        self.set_flag(Flag::N, (self.x & 0b10000000) > 0);

        0
    }

    fn bcs(&mut self, _: &mut Bus) -> usize {
        self.conditional_branch(Flag::C, true);

        0
    }

    fn clv(&mut self, _: &mut Bus) -> usize {
        self.set_flag(Flag::V, false);

        0
    }

    fn tsx(&mut self, _: &mut Bus) -> usize {
        self.x = self.sp;

        self.set_flag(Flag::Z, self.x == 0);
        self.set_flag(Flag::N, (self.x & 0b10000000) > 0);

        0
    }

    fn cpy(&mut self, bus: &mut Bus) -> usize {
        self.fetch(bus);

        self.set_flag(Flag::C, self.y >= self.fetched);
        self.set_flag(Flag::Z, self.y == self.fetched);
        self.set_flag(
            Flag::N,
            (self.y.wrapping_sub(self.fetched) & 0b10000000) > 0,
        );

        0
    }

    fn cmp(&mut self, bus: &mut Bus) -> usize {
        self.fetch(bus);

        self.set_flag(Flag::C, self.a >= self.fetched);
        self.set_flag(Flag::Z, self.a == self.fetched);
        self.set_flag(
            Flag::N,
            (self.a.wrapping_sub(self.fetched) & 0b10000000) > 0,
        );

        0
    }

    fn dec(&mut self, bus: &mut Bus) -> usize {
        self.fetch(bus);

        let value = self.fetched.wrapping_sub(1);
        bus.write(self.addr_abs, value);

        self.set_flag(Flag::Z, value == 0);
        self.set_flag(Flag::N, (value & 0b10000000) > 0);

        0
    }

    fn iny(&mut self, _: &mut Bus) -> usize {
        self.y = self.y.wrapping_add(1);

        self.set_flag(Flag::Z, self.y == 0);
        self.set_flag(Flag::N, (self.y & 0b10000000) > 0);

        0
    }

    fn dex(&mut self, _: &mut Bus) -> usize {
        self.x = self.x.wrapping_sub(1);

        self.set_flag(Flag::Z, self.x == 0);
        self.set_flag(Flag::N, (self.x & 0b10000000) > 0);

        0
    }

    fn bne(&mut self, _: &mut Bus) -> usize {
        self.conditional_branch(Flag::Z, false);

        0
    }

    fn cld(&mut self, _: &mut Bus) -> usize {
        self.set_flag(Flag::D, false);

        0
    }

    fn nop(&mut self, _: &mut Bus) -> usize {
        0
    }

    fn cpx(&mut self, bus: &mut Bus) -> usize {
        self.fetch(bus);

        self.set_flag(Flag::C, self.x >= self.fetched);
        self.set_flag(Flag::Z, self.x == self.fetched);
        self.set_flag(
            Flag::N,
            (self.x.wrapping_sub(self.fetched) & 0b10000000) > 0,
        );

        0
    }

    fn sbc(&mut self, bus: &mut Bus) -> usize {
        self.fetch(bus);
        self.fetched = !self.fetched;

        let result: u16 = self.a as u16 + self.fetched as u16 + self.get_flag(Flag::C) as u16;
        let overflow =
            (!((self.a as u16) ^ (self.fetched as u16))) & ((self.a as u16) ^ result) & 0x80;

        self.set_flag(Flag::C, result > 0xFF);
        self.set_flag(Flag::V, overflow > 0);

        self.a = result as u8;

        self.set_flag(Flag::Z, self.a == 0);
        self.set_flag(Flag::N, self.a & 0b10000000 > 0);

        1
    }

    fn inc(&mut self, bus: &mut Bus) -> usize {
        self.fetch(bus);

        let value: u8 = self.fetched.wrapping_add(1);
        bus.write(self.addr_abs, value);

        self.set_flag(Flag::N, (value & 0b10000000) > 0);
        self.set_flag(Flag::Z, value == 0);

        0
    }

    fn inx(&mut self, _: &mut Bus) -> usize {
        self.x = self.x.wrapping_add(1);

        self.set_flag(Flag::N, (self.x & 0b10000000) > 0);
        self.set_flag(Flag::Z, self.x == 0);

        1
    }

    fn beq(&mut self, _: &mut Bus) -> usize {
        self.conditional_branch(Flag::Z, true);

        0
    }

    fn sed(&mut self, _: &mut Bus) -> usize {
        self.set_flag(Flag::D, true);

        0
    }

    // Illegal opcodes

    fn lax(&mut self, bus: &mut Bus) -> usize {
        self.lda(bus);
        self.tax(bus);

        1
    }

    fn sax(&mut self, bus: &mut Bus) -> usize {
        bus.write(self.addr_abs, self.a & self.x);

        0
    }

    fn dcp(&mut self, bus: &mut Bus) -> usize {
        self.dec(bus);
        self.cmp(bus);

        0
    }

    fn isc(&mut self, bus: &mut Bus) -> usize {
        self.inc(bus);
        self.sbc(bus);

        1
    }

    fn slo(&mut self, bus: &mut Bus) -> usize {
        self.asl(bus);
        self.ora(bus);

        0
    }

    fn rla(&mut self, bus: &mut Bus) -> usize {
        self.rol(bus);
        self.and(bus);

        1
    }

    fn sre(&mut self, bus: &mut Bus) -> usize {
        self.lsr(bus);
        self.eor(bus);

        0
    }

    fn rra(&mut self, bus: &mut Bus) -> usize {
        self.ror(bus);
        self.adc(bus);

        1
    }

    // Halts the cpu which doesn't increase the program counter.
    // Remove 1 from the pc to mimic this behavior.
    fn kil(&mut self, _: &mut Bus) -> usize {
        self.pc = self.pc.wrapping_sub(1);

        0
    }

    fn anc(&mut self, bus: &mut Bus) -> usize {
        self.and(bus);
        self.set_flag(Flag::C, (self.a & 0b10000000) > 0);

        1
    }

    fn alr(&mut self, bus: &mut Bus) -> usize {
        self.and(bus);

        self.set_flag(Flag::C, (self.a & 0x0001) > 0);
        self.a = self.a.wrapping_shr(1);

        self.set_flag(Flag::Z, self.a == 0);
        self.set_flag(Flag::N, (self.a & 0b10000000) > 1);

        1
    }

    fn arr(&mut self, bus: &mut Bus) -> usize {
        self.fetch(bus);
        self.a &= self.fetched;

        let t = self.a.wrapping_shr(7);
        self.a = self.a.wrapping_shr(1);
        self.a |= (self.get_flag(Flag::C) as u8).wrapping_shl(7);

        self.set_flag(Flag::C, t & 0x01 == 0x01);
        self.set_flag(Flag::Z, self.a == 0);
        self.set_flag(Flag::N, (self.a & 0b10000000) > 1);

        let bit_6 = (self.a >> 6) & 1;
        let bit_5 = (self.a >> 5) & 1;
        self.set_flag(Flag::V, bit_6 ^ bit_5 == 1);

        0
    }

    // Unstable operation; 0xEE could be 0xFF, 0x00, etc.
    // depending on the specific chip or even environmental
    // conditions.
    fn xaa(&mut self, bus: &mut Bus) -> usize {
        self.fetch(bus);
        self.a = (self.a | 0xEE) & self.x & self.fetched;

        self.set_flag(Flag::N, (self.a & 0b10000000) > 0);
        self.set_flag(Flag::Z, self.a == 0);

        0
    }

    fn axs(&mut self, bus: &mut Bus) -> usize {
        self.fetch(bus);

        let res = (self.a & self.x).wrapping_sub(self.fetched);

        self.set_flag(Flag::C, (self.a & self.x) >= res);
        self.set_flag(Flag::Z, res == 0);
        self.set_flag(Flag::N, (res & 0b10000000) > 1);

        self.x = res;

        0
    }

    fn tas(&mut self, bus: &mut Bus) -> usize {
        self.fetch(bus);
        self.sp = self.a & self.x;
        bus.write(
            self.addr_abs,
            self.a & self.x & ((self.addr_abs.wrapping_shr(8) as u8).wrapping_add(1)),
        );

        0
    }

    fn sha(&mut self, bus: &mut Bus) -> usize {
        self.fetch(bus);
        bus.write(
            self.addr_abs,
            self.a & self.x & (self.addr_abs.wrapping_shr(8) as u8).wrapping_add(1),
        );

        0
    }

    fn shx(&mut self, bus: &mut Bus) -> usize {
        self.fetch(bus);
        bus.write(
            self.addr_abs,
            self.x & (self.addr_abs.wrapping_shr(8) as u8).wrapping_add(1),
        );

        0
    }

    fn shy(&mut self, bus: &mut Bus) -> usize {
        self.fetch(bus);
        bus.write(
            self.addr_abs,
            self.y & (self.addr_abs.wrapping_shr(8) as u8).wrapping_add(1),
        );

        0
    }

    fn lxa(&mut self, bus: &mut Bus) -> usize {
        self.fetch(bus);

        let res = (self.a | 0xEE) & self.fetched;
        self.a = res;
        self.x = res;

        self.set_flag(Flag::Z, res == 0);
        self.set_flag(Flag::N, (res & 0b10000000) > 0);

        0
    }

    fn las(&mut self, bus: &mut Bus) -> usize {
        self.fetch(bus);

        let res = self.fetched & self.sp;
        self.a = res;
        self.x = res;
        self.sp = res;

        self.set_flag(Flag::Z, res == 0);
        self.set_flag(Flag::N, (res & 0b10000000) > 0);

        0
    }
}

#[cfg(test)]
mod tests {
    extern crate test_generator;

    use serde::Deserialize;
    use test_generator::test_resources;

    use std::fs::File;
    use std::io::BufReader;

    use crate::bus::Bus;
    use crate::cpu::Cpu;

    #[derive(Debug, Deserialize)]
    struct CpuState {
        pc: u16,
        s: u8,
        a: u8,
        x: u8,
        y: u8,
        p: u8,
        ram: Vec<(u16, u8)>,
    }

    #[derive(Debug, Deserialize)]
    struct TestCase {
        #[serde(rename = "initial")]
        initial_state: CpuState,
        #[serde(rename = "final")]
        final_state: CpuState,
    }

    #[test_resources("tests/*.json")]
    fn operation(resource: &str) {
        let file = File::open(resource).unwrap();
        let reader = BufReader::new(file);
        let test_cases: Vec<TestCase> =
            serde_json::from_reader(reader).expect("Problem reading file");

        for test_case in test_cases.iter() {
            // Filling CPU state
            let mut bus = Bus::default();
            let mut cpu = Cpu::default();

            // let mut cpu = Cpu::new();
            cpu.reset();

            cpu.pc = test_case.initial_state.pc;
            cpu.sp = test_case.initial_state.s;
            cpu.a = test_case.initial_state.a;
            cpu.x = test_case.initial_state.x;
            cpu.y = test_case.initial_state.y;
            cpu.status = test_case.initial_state.p;

            cpu.cycles = 0;

            for (address, value) in test_case.initial_state.ram.iter() {
                bus.write(*address, *value);
            }

            loop {
                cpu.clock(&mut bus);

                if cpu.complete() {
                    break;
                }
            }

            // State comparison
            assert_eq!(cpu.pc, test_case.final_state.pc);
            assert_eq!(cpu.sp, test_case.final_state.s);
            assert_eq!(cpu.a, test_case.final_state.a);
            assert_eq!(cpu.x, test_case.final_state.x);
            assert_eq!(cpu.y, test_case.final_state.y);
            assert_eq!(cpu.status, test_case.final_state.p);

            for (address, value) in test_case.final_state.ram.iter() {
                assert_eq!(bus.read(*address), *value)
            }
        }
    }
}
