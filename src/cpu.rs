use crate::bus::Bus;

use std::collections::HashMap;

enum Flags {
    C = (1 << 0), // carry bit
    Z = (1 << 1), // zero
    I = (1 << 2), // disable interrupts
    D = (1 << 3), // decimal mode
    B = (1 << 4), // break
    U = (1 << 5), // unused
    V = (1 << 6), // overflow
    N = (1 << 7), // negative
}

#[derive(Clone, Copy)]
struct Instruction<'a> {
    name: &'static str,
    operation: fn(&mut Cpu<'a>) -> usize,
    addressmode: fn(&mut Cpu<'a>) -> usize,
    cycles: usize,
}

pub struct Cpu<'a> {
    pub a: u8,
    pub x: u8,
    pub y: u8,
    pub sp: u8,
    pub pc: u16,
    pub status: u8,
    pub bus: &'a mut Bus,

    pub fetched: u8,
    pub addr_abs: u16,
    pub addr_rel: u16,
    pub opcode: u8,
    pub cycles: usize,

    dispatch: HashMap<u8, Instruction<'a>>,
}

impl<'a> Cpu<'a> {
    pub fn new(bus: &'a mut Bus) -> Cpu<'a> {
        Cpu {
            a: 0x00,
            x: 0x00,
            y: 0x00,
            sp: 0xFD,
            pc: 0x0000,
            status: 0x34,
            bus: bus,

            fetched: 0x00,
            addr_abs: 0x0000,
            addr_rel: 0x0000,
            opcode: 0x00,
            cycles: 0,

            dispatch: HashMap::from([
                (
                    0x0,
                    Instruction {
                        name: "brk",
                        operation: Cpu::brk,
                        addressmode: Cpu::imm,
                        cycles: 7,
                    },
                ),
                (
                    0x1,
                    Instruction {
                        name: "ora",
                        operation: Cpu::ora,
                        addressmode: Cpu::izx,
                        cycles: 6,
                    },
                ),
                (
                    0x5,
                    Instruction {
                        name: "ora",
                        operation: Cpu::ora,
                        addressmode: Cpu::zp0,
                        cycles: 3,
                    },
                ),
                (
                    0x6,
                    Instruction {
                        name: "asl",
                        operation: Cpu::asl,
                        addressmode: Cpu::zp0,
                        cycles: 5,
                    },
                ),
                (
                    0x8,
                    Instruction {
                        name: "php",
                        operation: Cpu::php,
                        addressmode: Cpu::imp,
                        cycles: 3,
                    },
                ),
                (
                    0x9,
                    Instruction {
                        name: "ora",
                        operation: Cpu::ora,
                        addressmode: Cpu::imm,
                        cycles: 2,
                    },
                ),
                (
                    0xa,
                    Instruction {
                        name: "asl",
                        operation: Cpu::asl,
                        addressmode: Cpu::imp,
                        cycles: 2,
                    },
                ),
                (
                    0xd,
                    Instruction {
                        name: "ora",
                        operation: Cpu::ora,
                        addressmode: Cpu::abs,
                        cycles: 4,
                    },
                ),
                (
                    0xe,
                    Instruction {
                        name: "asl",
                        operation: Cpu::asl,
                        addressmode: Cpu::abs,
                        cycles: 6,
                    },
                ),
                (
                    0x10,
                    Instruction {
                        name: "bpl",
                        operation: Cpu::bpl,
                        addressmode: Cpu::rel,
                        cycles: 2,
                    },
                ),
                (
                    0x11,
                    Instruction {
                        name: "ora",
                        operation: Cpu::ora,
                        addressmode: Cpu::izy,
                        cycles: 5,
                    },
                ),
                (
                    0x15,
                    Instruction {
                        name: "ora",
                        operation: Cpu::ora,
                        addressmode: Cpu::zpx,
                        cycles: 4,
                    },
                ),
                (
                    0x16,
                    Instruction {
                        name: "asl",
                        operation: Cpu::asl,
                        addressmode: Cpu::zpx,
                        cycles: 6,
                    },
                ),
                (
                    0x18,
                    Instruction {
                        name: "clc",
                        operation: Cpu::clc,
                        addressmode: Cpu::imp,
                        cycles: 2,
                    },
                ),
                (
                    0x19,
                    Instruction {
                        name: "ora",
                        operation: Cpu::ora,
                        addressmode: Cpu::aby,
                        cycles: 4,
                    },
                ),
                (
                    0x1d,
                    Instruction {
                        name: "ora",
                        operation: Cpu::ora,
                        addressmode: Cpu::abx,
                        cycles: 4,
                    },
                ),
                (
                    0x1e,
                    Instruction {
                        name: "asl",
                        operation: Cpu::asl,
                        addressmode: Cpu::abx,
                        cycles: 7,
                    },
                ),
                (
                    0x20,
                    Instruction {
                        name: "jsr",
                        operation: Cpu::jsr,
                        addressmode: Cpu::abs,
                        cycles: 6,
                    },
                ),
                (
                    0x21,
                    Instruction {
                        name: "and",
                        operation: Cpu::and,
                        addressmode: Cpu::izx,
                        cycles: 6,
                    },
                ),
                (
                    0x24,
                    Instruction {
                        name: "bit",
                        operation: Cpu::bit,
                        addressmode: Cpu::zp0,
                        cycles: 3,
                    },
                ),
                (
                    0x25,
                    Instruction {
                        name: "and",
                        operation: Cpu::and,
                        addressmode: Cpu::zp0,
                        cycles: 3,
                    },
                ),
                (
                    0x26,
                    Instruction {
                        name: "rol",
                        operation: Cpu::rol,
                        addressmode: Cpu::zp0,
                        cycles: 5,
                    },
                ),
                (
                    0x28,
                    Instruction {
                        name: "plp",
                        operation: Cpu::plp,
                        addressmode: Cpu::imp,
                        cycles: 4,
                    },
                ),
                (
                    0x29,
                    Instruction {
                        name: "and",
                        operation: Cpu::and,
                        addressmode: Cpu::imm,
                        cycles: 2,
                    },
                ),
                (
                    0x2a,
                    Instruction {
                        name: "rol",
                        operation: Cpu::rol,
                        addressmode: Cpu::imp,
                        cycles: 2,
                    },
                ),
                (
                    0x2c,
                    Instruction {
                        name: "bit",
                        operation: Cpu::bit,
                        addressmode: Cpu::abs,
                        cycles: 4,
                    },
                ),
                (
                    0x2d,
                    Instruction {
                        name: "and",
                        operation: Cpu::and,
                        addressmode: Cpu::abs,
                        cycles: 4,
                    },
                ),
                (
                    0x2e,
                    Instruction {
                        name: "rol",
                        operation: Cpu::rol,
                        addressmode: Cpu::abs,
                        cycles: 6,
                    },
                ),
                (
                    0x30,
                    Instruction {
                        name: "bmi",
                        operation: Cpu::bmi,
                        addressmode: Cpu::rel,
                        cycles: 2,
                    },
                ),
                (
                    0x31,
                    Instruction {
                        name: "and",
                        operation: Cpu::and,
                        addressmode: Cpu::izy,
                        cycles: 5,
                    },
                ),
                (
                    0x35,
                    Instruction {
                        name: "and",
                        operation: Cpu::and,
                        addressmode: Cpu::zpx,
                        cycles: 4,
                    },
                ),
                (
                    0x36,
                    Instruction {
                        name: "rol",
                        operation: Cpu::rol,
                        addressmode: Cpu::zpx,
                        cycles: 6,
                    },
                ),
                (
                    0x38,
                    Instruction {
                        name: "sec",
                        operation: Cpu::sec,
                        addressmode: Cpu::imp,
                        cycles: 2,
                    },
                ),
                (
                    0x39,
                    Instruction {
                        name: "and",
                        operation: Cpu::and,
                        addressmode: Cpu::aby,
                        cycles: 4,
                    },
                ),
                (
                    0x3d,
                    Instruction {
                        name: "and",
                        operation: Cpu::and,
                        addressmode: Cpu::abx,
                        cycles: 4,
                    },
                ),
                (
                    0x3e,
                    Instruction {
                        name: "rol",
                        operation: Cpu::rol,
                        addressmode: Cpu::abx,
                        cycles: 7,
                    },
                ),
                (
                    0x40,
                    Instruction {
                        name: "rti",
                        operation: Cpu::rti,
                        addressmode: Cpu::imp,
                        cycles: 6,
                    },
                ),
                (
                    0x41,
                    Instruction {
                        name: "eor",
                        operation: Cpu::eor,
                        addressmode: Cpu::izx,
                        cycles: 6,
                    },
                ),
                (
                    0x45,
                    Instruction {
                        name: "eor",
                        operation: Cpu::eor,
                        addressmode: Cpu::zp0,
                        cycles: 3,
                    },
                ),
                (
                    0x46,
                    Instruction {
                        name: "lsr",
                        operation: Cpu::lsr,
                        addressmode: Cpu::zp0,
                        cycles: 5,
                    },
                ),
                (
                    0x48,
                    Instruction {
                        name: "pha",
                        operation: Cpu::pha,
                        addressmode: Cpu::imp,
                        cycles: 3,
                    },
                ),
                (
                    0x49,
                    Instruction {
                        name: "eor",
                        operation: Cpu::eor,
                        addressmode: Cpu::imm,
                        cycles: 2,
                    },
                ),
                (
                    0x4a,
                    Instruction {
                        name: "lsr",
                        operation: Cpu::lsr,
                        addressmode: Cpu::imp,
                        cycles: 2,
                    },
                ),
                (
                    0x4c,
                    Instruction {
                        name: "jmp",
                        operation: Cpu::jmp,
                        addressmode: Cpu::abs,
                        cycles: 3,
                    },
                ),
                (
                    0x4d,
                    Instruction {
                        name: "eor",
                        operation: Cpu::eor,
                        addressmode: Cpu::abs,
                        cycles: 4,
                    },
                ),
                (
                    0x4e,
                    Instruction {
                        name: "lsr",
                        operation: Cpu::lsr,
                        addressmode: Cpu::abs,
                        cycles: 6,
                    },
                ),
                (
                    0x50,
                    Instruction {
                        name: "bvc",
                        operation: Cpu::bvc,
                        addressmode: Cpu::rel,
                        cycles: 2,
                    },
                ),
                (
                    0x51,
                    Instruction {
                        name: "eor",
                        operation: Cpu::eor,
                        addressmode: Cpu::izy,
                        cycles: 5,
                    },
                ),
                (
                    0x55,
                    Instruction {
                        name: "eor",
                        operation: Cpu::eor,
                        addressmode: Cpu::zpx,
                        cycles: 4,
                    },
                ),
                (
                    0x56,
                    Instruction {
                        name: "lsr",
                        operation: Cpu::lsr,
                        addressmode: Cpu::zpx,
                        cycles: 6,
                    },
                ),
                (
                    0x58,
                    Instruction {
                        name: "cli",
                        operation: Cpu::cli,
                        addressmode: Cpu::imp,
                        cycles: 2,
                    },
                ),
                (
                    0x59,
                    Instruction {
                        name: "eor",
                        operation: Cpu::eor,
                        addressmode: Cpu::aby,
                        cycles: 4,
                    },
                ),
                (
                    0x5d,
                    Instruction {
                        name: "eor",
                        operation: Cpu::eor,
                        addressmode: Cpu::abx,
                        cycles: 4,
                    },
                ),
                (
                    0x5e,
                    Instruction {
                        name: "lsr",
                        operation: Cpu::lsr,
                        addressmode: Cpu::abx,
                        cycles: 7,
                    },
                ),
                (
                    0x60,
                    Instruction {
                        name: "rts",
                        operation: Cpu::rts,
                        addressmode: Cpu::imp,
                        cycles: 6,
                    },
                ),
                (
                    0x61,
                    Instruction {
                        name: "adc",
                        operation: Cpu::adc,
                        addressmode: Cpu::izx,
                        cycles: 6,
                    },
                ),
                (
                    0x65,
                    Instruction {
                        name: "adc",
                        operation: Cpu::adc,
                        addressmode: Cpu::zp0,
                        cycles: 3,
                    },
                ),
                (
                    0x66,
                    Instruction {
                        name: "ror",
                        operation: Cpu::ror,
                        addressmode: Cpu::zp0,
                        cycles: 5,
                    },
                ),
                (
                    0x68,
                    Instruction {
                        name: "pla",
                        operation: Cpu::pla,
                        addressmode: Cpu::imp,
                        cycles: 4,
                    },
                ),
                (
                    0x69,
                    Instruction {
                        name: "adc",
                        operation: Cpu::adc,
                        addressmode: Cpu::imm,
                        cycles: 2,
                    },
                ),
                (
                    0x6a,
                    Instruction {
                        name: "ror",
                        operation: Cpu::ror,
                        addressmode: Cpu::imp,
                        cycles: 2,
                    },
                ),
                (
                    0x6c,
                    Instruction {
                        name: "jmp",
                        operation: Cpu::jmp,
                        addressmode: Cpu::ind,
                        cycles: 5,
                    },
                ),
                (
                    0x6d,
                    Instruction {
                        name: "adc",
                        operation: Cpu::adc,
                        addressmode: Cpu::abs,
                        cycles: 4,
                    },
                ),
                (
                    0x6e,
                    Instruction {
                        name: "ror",
                        operation: Cpu::ror,
                        addressmode: Cpu::abs,
                        cycles: 6,
                    },
                ),
                (
                    0x70,
                    Instruction {
                        name: "bvs",
                        operation: Cpu::bvs,
                        addressmode: Cpu::rel,
                        cycles: 2,
                    },
                ),
                (
                    0x71,
                    Instruction {
                        name: "adc",
                        operation: Cpu::adc,
                        addressmode: Cpu::izy,
                        cycles: 5,
                    },
                ),
                (
                    0x75,
                    Instruction {
                        name: "adc",
                        operation: Cpu::adc,
                        addressmode: Cpu::zpx,
                        cycles: 4,
                    },
                ),
                (
                    0x76,
                    Instruction {
                        name: "ror",
                        operation: Cpu::ror,
                        addressmode: Cpu::zpx,
                        cycles: 6,
                    },
                ),
                (
                    0x78,
                    Instruction {
                        name: "sei",
                        operation: Cpu::sei,
                        addressmode: Cpu::imp,
                        cycles: 2,
                    },
                ),
                (
                    0x79,
                    Instruction {
                        name: "adc",
                        operation: Cpu::adc,
                        addressmode: Cpu::aby,
                        cycles: 4,
                    },
                ),
                (
                    0x7d,
                    Instruction {
                        name: "adc",
                        operation: Cpu::adc,
                        addressmode: Cpu::abx,
                        cycles: 4,
                    },
                ),
                (
                    0x7e,
                    Instruction {
                        name: "ror",
                        operation: Cpu::ror,
                        addressmode: Cpu::abx,
                        cycles: 7,
                    },
                ),
                (
                    0x81,
                    Instruction {
                        name: "sta",
                        operation: Cpu::sta,
                        addressmode: Cpu::izx,
                        cycles: 6,
                    },
                ),
                (
                    0x84,
                    Instruction {
                        name: "sty",
                        operation: Cpu::sty,
                        addressmode: Cpu::zp0,
                        cycles: 3,
                    },
                ),
                (
                    0x85,
                    Instruction {
                        name: "sta",
                        operation: Cpu::sta,
                        addressmode: Cpu::zp0,
                        cycles: 3,
                    },
                ),
                (
                    0x86,
                    Instruction {
                        name: "stx",
                        operation: Cpu::stx,
                        addressmode: Cpu::zp0,
                        cycles: 3,
                    },
                ),
                (
                    0x88,
                    Instruction {
                        name: "dey",
                        operation: Cpu::dey,
                        addressmode: Cpu::imp,
                        cycles: 2,
                    },
                ),
                (
                    0x8a,
                    Instruction {
                        name: "txa",
                        operation: Cpu::txa,
                        addressmode: Cpu::imp,
                        cycles: 2,
                    },
                ),
                (
                    0x8c,
                    Instruction {
                        name: "sty",
                        operation: Cpu::sty,
                        addressmode: Cpu::abs,
                        cycles: 4,
                    },
                ),
                (
                    0x8d,
                    Instruction {
                        name: "sta",
                        operation: Cpu::sta,
                        addressmode: Cpu::abs,
                        cycles: 4,
                    },
                ),
                (
                    0x8e,
                    Instruction {
                        name: "stx",
                        operation: Cpu::stx,
                        addressmode: Cpu::abs,
                        cycles: 4,
                    },
                ),
                (
                    0x90,
                    Instruction {
                        name: "bcc",
                        operation: Cpu::bcc,
                        addressmode: Cpu::rel,
                        cycles: 2,
                    },
                ),
                (
                    0x91,
                    Instruction {
                        name: "sta",
                        operation: Cpu::sta,
                        addressmode: Cpu::izy,
                        cycles: 6,
                    },
                ),
                (
                    0x94,
                    Instruction {
                        name: "sty",
                        operation: Cpu::sty,
                        addressmode: Cpu::zpx,
                        cycles: 4,
                    },
                ),
                (
                    0x95,
                    Instruction {
                        name: "sta",
                        operation: Cpu::sta,
                        addressmode: Cpu::zpx,
                        cycles: 4,
                    },
                ),
                (
                    0x96,
                    Instruction {
                        name: "stx",
                        operation: Cpu::stx,
                        addressmode: Cpu::zpy,
                        cycles: 4,
                    },
                ),
                (
                    0x98,
                    Instruction {
                        name: "tya",
                        operation: Cpu::tya,
                        addressmode: Cpu::imp,
                        cycles: 2,
                    },
                ),
                (
                    0x99,
                    Instruction {
                        name: "sta",
                        operation: Cpu::sta,
                        addressmode: Cpu::aby,
                        cycles: 5,
                    },
                ),
                (
                    0x9a,
                    Instruction {
                        name: "txs",
                        operation: Cpu::txs,
                        addressmode: Cpu::imp,
                        cycles: 2,
                    },
                ),
                (
                    0x9d,
                    Instruction {
                        name: "sta",
                        operation: Cpu::sta,
                        addressmode: Cpu::abx,
                        cycles: 5,
                    },
                ),
                (
                    0xa0,
                    Instruction {
                        name: "ldy",
                        operation: Cpu::ldy,
                        addressmode: Cpu::imm,
                        cycles: 2,
                    },
                ),
                (
                    0xa1,
                    Instruction {
                        name: "lda",
                        operation: Cpu::lda,
                        addressmode: Cpu::izx,
                        cycles: 6,
                    },
                ),
                (
                    0xa2,
                    Instruction {
                        name: "ldx",
                        operation: Cpu::ldx,
                        addressmode: Cpu::imm,
                        cycles: 2,
                    },
                ),
                (
                    0xa4,
                    Instruction {
                        name: "ldy",
                        operation: Cpu::ldy,
                        addressmode: Cpu::zp0,
                        cycles: 3,
                    },
                ),
                (
                    0xa5,
                    Instruction {
                        name: "lda",
                        operation: Cpu::lda,
                        addressmode: Cpu::zp0,
                        cycles: 3,
                    },
                ),
                (
                    0xa6,
                    Instruction {
                        name: "ldx",
                        operation: Cpu::ldx,
                        addressmode: Cpu::zp0,
                        cycles: 3,
                    },
                ),
                (
                    0xa8,
                    Instruction {
                        name: "tay",
                        operation: Cpu::tay,
                        addressmode: Cpu::imp,
                        cycles: 2,
                    },
                ),
                (
                    0xa9,
                    Instruction {
                        name: "lda",
                        operation: Cpu::lda,
                        addressmode: Cpu::imm,
                        cycles: 2,
                    },
                ),
                (
                    0xaa,
                    Instruction {
                        name: "tax",
                        operation: Cpu::tax,
                        addressmode: Cpu::imp,
                        cycles: 2,
                    },
                ),
                (
                    0xac,
                    Instruction {
                        name: "ldy",
                        operation: Cpu::ldy,
                        addressmode: Cpu::abs,
                        cycles: 4,
                    },
                ),
                (
                    0xad,
                    Instruction {
                        name: "lda",
                        operation: Cpu::lda,
                        addressmode: Cpu::abs,
                        cycles: 4,
                    },
                ),
                (
                    0xae,
                    Instruction {
                        name: "ldx",
                        operation: Cpu::ldx,
                        addressmode: Cpu::abs,
                        cycles: 4,
                    },
                ),
                (
                    0xb0,
                    Instruction {
                        name: "bcs",
                        operation: Cpu::bcs,
                        addressmode: Cpu::rel,
                        cycles: 2,
                    },
                ),
                (
                    0xb1,
                    Instruction {
                        name: "lda",
                        operation: Cpu::lda,
                        addressmode: Cpu::izy,
                        cycles: 5,
                    },
                ),
                (
                    0xb4,
                    Instruction {
                        name: "ldy",
                        operation: Cpu::ldy,
                        addressmode: Cpu::zpx,
                        cycles: 4,
                    },
                ),
                (
                    0xb5,
                    Instruction {
                        name: "lda",
                        operation: Cpu::lda,
                        addressmode: Cpu::zpx,
                        cycles: 4,
                    },
                ),
                (
                    0xb6,
                    Instruction {
                        name: "ldx",
                        operation: Cpu::ldx,
                        addressmode: Cpu::zpy,
                        cycles: 4,
                    },
                ),
                (
                    0xb8,
                    Instruction {
                        name: "clv",
                        operation: Cpu::clv,
                        addressmode: Cpu::imp,
                        cycles: 2,
                    },
                ),
                (
                    0xb9,
                    Instruction {
                        name: "lda",
                        operation: Cpu::lda,
                        addressmode: Cpu::aby,
                        cycles: 4,
                    },
                ),
                (
                    0xba,
                    Instruction {
                        name: "tsx",
                        operation: Cpu::tsx,
                        addressmode: Cpu::imp,
                        cycles: 2,
                    },
                ),
                (
                    0xbc,
                    Instruction {
                        name: "ldy",
                        operation: Cpu::ldy,
                        addressmode: Cpu::abx,
                        cycles: 4,
                    },
                ),
                (
                    0xbd,
                    Instruction {
                        name: "lda",
                        operation: Cpu::lda,
                        addressmode: Cpu::abx,
                        cycles: 4,
                    },
                ),
                (
                    0xbe,
                    Instruction {
                        name: "ldx",
                        operation: Cpu::ldx,
                        addressmode: Cpu::aby,
                        cycles: 4,
                    },
                ),
                (
                    0xc0,
                    Instruction {
                        name: "cpy",
                        operation: Cpu::cpy,
                        addressmode: Cpu::imm,
                        cycles: 2,
                    },
                ),
                (
                    0xc1,
                    Instruction {
                        name: "cmp",
                        operation: Cpu::cmp,
                        addressmode: Cpu::izx,
                        cycles: 6,
                    },
                ),
                (
                    0xc4,
                    Instruction {
                        name: "cpy",
                        operation: Cpu::cpy,
                        addressmode: Cpu::zp0,
                        cycles: 3,
                    },
                ),
                (
                    0xc5,
                    Instruction {
                        name: "cmp",
                        operation: Cpu::cmp,
                        addressmode: Cpu::zp0,
                        cycles: 3,
                    },
                ),
                (
                    0xc6,
                    Instruction {
                        name: "dec",
                        operation: Cpu::dec,
                        addressmode: Cpu::zp0,
                        cycles: 5,
                    },
                ),
                (
                    0xc8,
                    Instruction {
                        name: "iny",
                        operation: Cpu::iny,
                        addressmode: Cpu::imp,
                        cycles: 2,
                    },
                ),
                (
                    0xc9,
                    Instruction {
                        name: "cmp",
                        operation: Cpu::cmp,
                        addressmode: Cpu::imm,
                        cycles: 2,
                    },
                ),
                (
                    0xca,
                    Instruction {
                        name: "dex",
                        operation: Cpu::dex,
                        addressmode: Cpu::imp,
                        cycles: 2,
                    },
                ),
                (
                    0xcc,
                    Instruction {
                        name: "cpy",
                        operation: Cpu::cpy,
                        addressmode: Cpu::abs,
                        cycles: 4,
                    },
                ),
                (
                    0xcd,
                    Instruction {
                        name: "cmp",
                        operation: Cpu::cmp,
                        addressmode: Cpu::abs,
                        cycles: 4,
                    },
                ),
                (
                    0xce,
                    Instruction {
                        name: "dec",
                        operation: Cpu::dec,
                        addressmode: Cpu::abs,
                        cycles: 6,
                    },
                ),
                (
                    0xd0,
                    Instruction {
                        name: "bne",
                        operation: Cpu::bne,
                        addressmode: Cpu::rel,
                        cycles: 2,
                    },
                ),
                (
                    0xd1,
                    Instruction {
                        name: "cmp",
                        operation: Cpu::cmp,
                        addressmode: Cpu::izy,
                        cycles: 5,
                    },
                ),
                (
                    0xd5,
                    Instruction {
                        name: "cmp",
                        operation: Cpu::cmp,
                        addressmode: Cpu::zpx,
                        cycles: 4,
                    },
                ),
                (
                    0xd6,
                    Instruction {
                        name: "dec",
                        operation: Cpu::dec,
                        addressmode: Cpu::zpx,
                        cycles: 6,
                    },
                ),
                (
                    0xd8,
                    Instruction {
                        name: "cld",
                        operation: Cpu::cld,
                        addressmode: Cpu::imp,
                        cycles: 2,
                    },
                ),
                (
                    0xd9,
                    Instruction {
                        name: "cmp",
                        operation: Cpu::cmp,
                        addressmode: Cpu::aby,
                        cycles: 4,
                    },
                ),
                (
                    0xda,
                    Instruction {
                        name: "nop",
                        operation: Cpu::nop,
                        addressmode: Cpu::imp,
                        cycles: 2,
                    },
                ),
                (
                    0xdd,
                    Instruction {
                        name: "cmp",
                        operation: Cpu::cmp,
                        addressmode: Cpu::abx,
                        cycles: 4,
                    },
                ),
                (
                    0xde,
                    Instruction {
                        name: "dec",
                        operation: Cpu::dec,
                        addressmode: Cpu::abx,
                        cycles: 7,
                    },
                ),
                (
                    0xe0,
                    Instruction {
                        name: "cpx",
                        operation: Cpu::cpx,
                        addressmode: Cpu::imm,
                        cycles: 2,
                    },
                ),
                (
                    0xe1,
                    Instruction {
                        name: "sbc",
                        operation: Cpu::sbc,
                        addressmode: Cpu::izx,
                        cycles: 6,
                    },
                ),
                (
                    0xe4,
                    Instruction {
                        name: "cpx",
                        operation: Cpu::cpx,
                        addressmode: Cpu::zp0,
                        cycles: 3,
                    },
                ),
                (
                    0xe5,
                    Instruction {
                        name: "sbc",
                        operation: Cpu::sbc,
                        addressmode: Cpu::zp0,
                        cycles: 3,
                    },
                ),
                (
                    0xe6,
                    Instruction {
                        name: "inc",
                        operation: Cpu::inc,
                        addressmode: Cpu::zp0,
                        cycles: 5,
                    },
                ),
                (
                    0xe8,
                    Instruction {
                        name: "inx",
                        operation: Cpu::inx,
                        addressmode: Cpu::imp,
                        cycles: 2,
                    },
                ),
                (
                    0xe9,
                    Instruction {
                        name: "sbc",
                        operation: Cpu::sbc,
                        addressmode: Cpu::imm,
                        cycles: 2,
                    },
                ),
                (
                    0xea,
                    Instruction {
                        name: "nop",
                        operation: Cpu::nop,
                        addressmode: Cpu::imp,
                        cycles: 2,
                    },
                ),
                (
                    0xec,
                    Instruction {
                        name: "cpx",
                        operation: Cpu::cpx,
                        addressmode: Cpu::abs,
                        cycles: 4,
                    },
                ),
                (
                    0xed,
                    Instruction {
                        name: "sbc",
                        operation: Cpu::sbc,
                        addressmode: Cpu::abs,
                        cycles: 4,
                    },
                ),
                (
                    0xee,
                    Instruction {
                        name: "inc",
                        operation: Cpu::inc,
                        addressmode: Cpu::abs,
                        cycles: 6,
                    },
                ),
                (
                    0xf0,
                    Instruction {
                        name: "beq",
                        operation: Cpu::beq,
                        addressmode: Cpu::rel,
                        cycles: 2,
                    },
                ),
                (
                    0xf1,
                    Instruction {
                        name: "sbc",
                        operation: Cpu::sbc,
                        addressmode: Cpu::izy,
                        cycles: 5,
                    },
                ),
                (
                    0xf5,
                    Instruction {
                        name: "sbc",
                        operation: Cpu::sbc,
                        addressmode: Cpu::zpx,
                        cycles: 4,
                    },
                ),
                (
                    0xf6,
                    Instruction {
                        name: "inc",
                        operation: Cpu::inc,
                        addressmode: Cpu::zpx,
                        cycles: 6,
                    },
                ),
                (
                    0xf8,
                    Instruction {
                        name: "sed",
                        operation: Cpu::sed,
                        addressmode: Cpu::imp,
                        cycles: 2,
                    },
                ),
                (
                    0xf9,
                    Instruction {
                        name: "sbc",
                        operation: Cpu::sbc,
                        addressmode: Cpu::aby,
                        cycles: 4,
                    },
                ),
                (
                    0xfa,
                    Instruction {
                        name: "nop",
                        operation: Cpu::nop,
                        addressmode: Cpu::imp,
                        cycles: 2,
                    },
                ),
                (
                    0xfd,
                    Instruction {
                        name: "sbc",
                        operation: Cpu::sbc,
                        addressmode: Cpu::abx,
                        cycles: 4,
                    },
                ),
                (
                    0xfe,
                    Instruction {
                        name: "inc",
                        operation: Cpu::inc,
                        addressmode: Cpu::abx,
                        cycles: 7,
                    },
                ),
            ]),
        }
    }

    pub fn set_flag(&mut self, flag: Flags, value: bool) {
        match value {
            true => self.status |= flag as u8,
            false => self.status &= !(flag as u8),
        }
    }

    pub fn get_flag(&self, flag: Flags) -> bool {
        self.status & (flag as u8) != 0
    }

    pub fn clock(&mut self) {
        if self.cycles == 0 {
            self.opcode = self.bus.read(self.pc);
            self.pc = self.pc.overflowing_add(1).0;

            self.set_flag(Flags::U, true);

            let instruction = self
                .dispatch
                .get(&self.opcode)
                .copied()
                .expect("opcode should be in dispatch table");
            self.cycles = instruction.cycles;

            let addr_cycles = (instruction.addressmode)(self);
            let op_cycles = (instruction.operation)(self);

            self.cycles += addr_cycles & op_cycles;

            self.set_flag(Flags::U, true);
        }

        self.cycles -= 1;
    }

    pub fn complete(&self) -> bool {
        self.cycles == 0
    }

    pub fn reset(&mut self) {
        self.sp = self.sp.overflowing_sub(3).0;
        self.set_flag(Flags::I, true);
    }

    fn irq(&mut self) {
        if !self.get_flag(Flags::I) {
            self.nmi();
        }
    }

    fn nmi(&mut self) {
        self.bus
            .write(0x0100 + self.sp as u16, (self.pc >> 8) & 0x00FF);
        self.sp = self.sp.overflowing_sub(1).0;
        self.bus.write(0x0100 + self.sp as u16, self.pc & 0x00F);
        self.sp = self.sp.overflowing_sub(1).0;

        self.set_flag(Flags::B, false);
        self.set_flag(Flags::U, true);
        self.set_flag(Flags::I, true);
        self.bus.write(0x0100 + self.sp as u16, self.status as u16);
        self.sp = self.sp.overflowing_sub(1).0;

        self.addr_abs = 0xFFFE;
        let lo: u16 = self.bus.read(self.addr_abs) as u16;
        let hi: u16 = self.bus.read(self.addr_abs + 1) as u16;
        self.pc = (hi << 8) | lo;

        self.cycles = 7;
    }

    // ADDRESSING FUNCTIONS

    /// The addressing is implied in the opcode.
    fn imp(&mut self) -> usize {
        self.fetched = self.a;

        0
    }

    /// The address is supplied as part of the instruction.
    fn imm(&mut self) -> usize {
        self.addr_abs = self.pc;
        self.pc = self.pc.overflowing_add(1).0;

        0
    }

    /// Zero page addressing uses the high byte to address
    /// a specific page and the low byte to offset into that page
    fn zp0(&mut self) -> usize {
        self.addr_abs = self.bus.read(self.pc) as u16;
        self.pc = self.pc.overflowing_add(1).0;

        self.addr_abs &= 0x00FF;

        0
    }

    /// Zero page addressing with register x as extra offset.
    fn zpx(&mut self) -> usize {
        self.addr_abs = self.bus.read(self.pc) as u16 + self.x as u16;
        self.pc = self.pc.overflowing_add(1).0;
        self.addr_abs &= 0x00FF;

        0
    }

    /// Zero page addressing with register y as extra offset.
    fn zpy(&mut self) -> usize {
        self.addr_abs = self.bus.read(self.pc) as u16 + self.y as u16;
        self.pc = self.pc.overflowing_add(1).0;
        self.addr_abs &= 0x00FF;

        0
    }

    /// Relative addressing uses the second byte (signed) as an offset for the next
    /// instruction, which can range from -127 to +127 relative to the program
    /// counter.
    fn rel(&mut self) -> usize {
        self.addr_rel = self.bus.read(self.pc) as u16;
        self.pc = self.pc.overflowing_add(1).0;

        if (self.addr_rel & 0b10000000) > 0 {
            self.addr_rel |= 0xFF00;
        }

        0
    }

    /// Absolute addressing fetches the full 16 bit address
    /// from region in memory at the program counter.
    fn abs(&mut self) -> usize {
        let lo: u16 = self.bus.read(self.pc) as u16;
        self.pc = self.pc.overflowing_add(1).0;
        let hi: u16 = self.bus.read(self.pc) as u16;
        self.pc = self.pc.overflowing_add(1).0;

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
    fn abx(&mut self) -> usize {
        let lo: u16 = self.bus.read(self.pc) as u16;
        self.pc = self.pc.overflowing_add(1).0;
        let hi: u16 = self.bus.read(self.pc) as u16;
        self.pc = self.pc.overflowing_add(1).0;

        self.addr_abs = ((hi << 8) | lo).overflowing_add(self.x as u16).0;

        if (self.addr_abs & 0xFF00) != (hi << 8) {
            1
        } else {
            0
        }
    }

    /// Absolute addressing with y offset.
    fn aby(&mut self) -> usize {
        let lo: u16 = self.bus.read(self.pc) as u16;
        self.pc = self.pc.overflowing_add(1).0;
        let hi: u16 = self.bus.read(self.pc) as u16;
        self.pc = self.pc.overflowing_add(1).0;

        self.addr_abs = ((hi << 8) | lo).overflowing_add(self.y as u16).0;

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
    fn ind(&mut self) -> usize {
        let lo: u16 = self.bus.read(self.pc) as u16;
        self.pc = self.pc.overflowing_add(1).0;
        let hi: u16 = self.bus.read(self.pc) as u16;
        self.pc = self.pc.overflowing_add(1).0;
        let target = hi.overflowing_shl(8).0 | lo;

        self.addr_abs = if lo == 0x00FF {
            let effective_lo = self.bus.read(target) as u16;
            let effective_hi = self.bus.read(target & 0xFF00) as u16;

            effective_hi.overflowing_shl(8).0 | effective_lo
        } else {
            let effective_lo = self.bus.read(target) as u16;
            let effective_hi = self.bus.read(target + 1) as u16;

            effective_hi.overflowing_shl(8).0 | effective_lo
        };

        0
    }

    /// Zero page indirect addressing with register x offset.
    fn izx(&mut self) -> usize {
        let p: u16 = self.bus.read(self.pc) as u16;
        self.pc = self.pc.overflowing_add(1).0;

        let lo: u16 = self.bus.read((p + self.x as u16) & 0x00FF) as u16;
        let hi: u16 = self.bus.read((p + (self.x as u16) + 1) & 0x00FF) as u16;

        self.addr_abs = (hi << 8) | lo;

        0
    }

    /// Zero page indirect addressing with register y offset. Different from izx,
    /// the register y offset is added onto the fetched 16 bits from memory location.
    /// It may overflow into the next page, requiring an extra cpu cycle to complete.
    fn izy(&mut self) -> usize {
        let t: u16 = self.bus.read(self.pc) as u16;
        self.pc = self.pc.overflowing_add(1).0;

        let lo = self.bus.read(t & 0x00FF) as u16;
        let hi = self.bus.read((t + 1) & 0x00FF) as u16;

        self.addr_abs = ((hi << 8) | lo).overflowing_add(self.y as u16).0;

        if (self.addr_abs & 0xFF00) != (hi << 8) {
            1
        } else {
            0
        }
    }

    // OPCODE FUNCTIONS

    fn fetch(&mut self) -> u8 {
        let instruction = self.dispatch.get(&self.opcode).expect("Unknown opcode");
        if instruction.addressmode as usize != Cpu::imp as usize {
            self.fetched = self.bus.read(self.addr_abs);
        }

        self.fetched
    }

    fn conditional_branch(&mut self, flag: Flags, status: bool) {
        if self.get_flag(flag) == status {
            self.cycles += 1;

            self.addr_abs = self.pc.overflowing_add(self.addr_rel).0;

            if (self.addr_abs & 0xFF00) != self.pc & 0xFF00 {
                self.cycles += 1;
            }

            self.pc = self.addr_abs;
        }
    }

    fn brk(&mut self) -> usize {
        self.bus
            .write(0x0100 + self.sp as u16, (self.pc >> 8) & 0x00FF);
        self.sp = self.sp.overflowing_sub(1).0;
        self.bus.write(0x0100 + self.sp as u16, self.pc & 0x00FF);
        self.sp = self.sp.overflowing_sub(1).0;

        self.set_flag(Flags::B, true);
        self.bus.write(0x0100 + self.sp as u16, self.status as u16);
        self.sp = self.sp.overflowing_sub(1).0;
        self.set_flag(Flags::B, false);

        self.pc = self.bus.read(0xFFFE) as u16 | (self.bus.read(0xFFFF as u16) as u16) << 8;

        self.set_flag(Flags::I, true);
        0
    }

    fn ora(&mut self) -> usize {
        self.fetch();

        self.a |= self.fetched;

        self.set_flag(Flags::Z, self.a == 0);
        self.set_flag(Flags::N, (self.a & 0b10000000) > 0);

        0
    }

    fn asl(&mut self) -> usize {
        self.fetch();
        self.set_flag(Flags::C, (self.fetched & 0b10000000) > 0);

        let t: u8 = self.fetched.overflowing_shl(1).0;

        self.set_flag(Flags::Z, t == 0);
        self.set_flag(Flags::N, (t & 0b10000000) > 0);

        if let Some(ins) = self.dispatch.get(&self.opcode) {
            if (ins.addressmode as usize) == (Cpu::imp as usize) {
                self.a = t;
            } else {
                self.bus.write(self.addr_abs, t as u16);
            }
        }

        0
    }

    fn php(&mut self) -> usize {
        self.bus.write(
            0x0100 + self.sp as u16,
            (self.status | Flags::B as u8 | Flags::U as u8) as u16,
        );
        self.sp = self.sp.overflowing_sub(1).0;

        self.set_flag(Flags::B, false);
        self.set_flag(Flags::U, false);

        0
    }

    fn bpl(&mut self) -> usize {
        self.conditional_branch(Flags::N, false);

        0
    }

    fn clc(&mut self) -> usize {
        self.set_flag(Flags::C, false);

        0
    }

    fn jsr(&mut self) -> usize {
        self.pc = self.pc.overflowing_sub(1).0;

        self.bus.write(
            0x0100 + self.sp as u16,
            (self.pc.overflowing_shr(8).0) & 0x00FF,
        );
        self.sp = self.sp.overflowing_sub(1).0;
        self.bus.write(0x0100 + self.sp as u16, self.pc & 0x00FF);
        self.sp = self.sp.overflowing_sub(1).0;

        self.pc = self.addr_abs;

        0
    }

    fn and(&mut self) -> usize {
        self.fetch();
        self.a &= self.fetched;

        self.set_flag(Flags::Z, self.a == 0x00);
        self.set_flag(Flags::N, (self.a & 0b10000000) > 0);

        1
    }

    fn bit(&mut self) -> usize {
        self.fetch();

        self.set_flag(Flags::Z, (self.a & self.fetched) == 0);
        self.set_flag(Flags::N, self.fetched & (1 << 7) > 0);
        self.set_flag(Flags::V, self.fetched & (1 << 6) > 0);

        0
    }

    fn rol(&mut self) -> usize {
        self.fetch();

        let overflow = (self.fetched & 0b10000000) > 0;
        let operand = self.fetched.overflowing_shl(1).0 | self.get_flag(Flags::C) as u8;

        self.set_flag(Flags::C, overflow);
        self.set_flag(Flags::N, (operand & 0b10000000) > 0);
        self.set_flag(Flags::Z, operand == 0);

        if let Some(ins) = self.dispatch.get(&self.opcode) {
            if (ins.addressmode as usize) == (Cpu::imp as usize) {
                self.a = operand;
            } else {
                self.bus.write(self.addr_abs, operand as u16);
            }
        }

        0
    }

    fn plp(&mut self) -> usize {
        self.sp = self.sp.overflowing_add(1).0;
        self.status = self.bus.read(0x0100 + self.sp as u16);

        self.set_flag(Flags::U, true);
        self.set_flag(Flags::B, false);

        0
    }

    fn bmi(&mut self) -> usize {
        self.conditional_branch(Flags::N, true);

        0
    }

    fn sec(&mut self) -> usize {
        self.set_flag(Flags::C, true);

        1
    }

    fn rti(&mut self) -> usize {
        self.sp = self.sp.overflowing_add(1).0;
        self.status = self.bus.read(0x0100 + self.sp as u16);
        self.status &= !(Flags::B as u8);
        self.status &= !(Flags::U as u8);

        self.sp = self.sp.overflowing_add(1).0;
        self.pc = self.bus.read(0x0100 + self.sp as u16) as u16;
        self.sp = self.sp.overflowing_add(1).0;
        self.pc |= (self.bus.read(0x0100 + self.sp as u16) as u16)
            .overflowing_shl(8)
            .0;

        0
    }

    fn eor(&mut self) -> usize {
        self.fetch();

        self.a ^= self.fetched;

        self.set_flag(Flags::Z, self.a == 0);
        self.set_flag(Flags::N, (self.a & 0b10000000) > 1);

        0
    }

    fn lsr(&mut self) -> usize {
        self.fetch();
        self.set_flag(Flags::C, (self.fetched & 0x0001) > 0);

        let t: u8 = self.fetched.overflowing_shr(1).0;

        self.set_flag(Flags::Z, t == 0);
        self.set_flag(Flags::N, (t & 0b10000000) > 1);

        if let Some(ins) = self.dispatch.get(&self.opcode) {
            if (ins.addressmode as usize) == (Cpu::imp as usize) {
                self.a = t;
            } else {
                self.bus.write(self.addr_abs, t as u16);
            }
        }

        0
    }

    fn pha(&mut self) -> usize {
        self.bus.write(0x0100 + self.sp as u16, self.a as u16);
        self.sp = self.sp.overflowing_sub(1).0;

        0
    }

    fn jmp(&mut self) -> usize {
        self.pc = self.addr_abs;

        0
    }

    fn bvc(&mut self) -> usize {
        self.conditional_branch(Flags::V, false);

        0
    }

    fn cli(&mut self) -> usize {
        self.set_flag(Flags::I, false);

        0
    }

    fn rts(&mut self) -> usize {
        self.sp = self.sp.overflowing_add(1).0;
        let lo: u16 = self.bus.read(0x0100 + self.sp as u16) as u16;
        self.sp = self.sp.overflowing_add(1).0;
        let hi: u16 = self.bus.read(0x0100 + self.sp as u16) as u16;

        self.pc = (hi.overflowing_shl(8).0 | lo).overflowing_add(1).0;

        0
    }

    fn adc(&mut self) -> usize {
        self.fetch();

        let result: u16 = self.a as u16 + self.fetched as u16 + self.get_flag(Flags::C) as u16;
        let overflow =
            (!((self.a as u16) ^ (self.fetched as u16))) & ((self.a as u16) ^ result) & 0x80;

        self.set_flag(Flags::C, result > 0xFF);
        self.set_flag(Flags::V, overflow > 0);

        self.a = result as u8;

        self.set_flag(Flags::Z, self.a == 0);
        self.set_flag(Flags::N, self.a & 0b10000000 > 0);

        1
    }

    fn ror(&mut self) -> usize {
        self.fetch();
        let t = ((self.get_flag(Flags::C) as u8) << 7) as u16
            | self.fetched.overflowing_shr(1).0 as u16;

        self.set_flag(Flags::C, (self.fetched & 0x01) > 0);
        self.set_flag(Flags::Z, (t & 0x00FF) == 0x00);
        self.set_flag(Flags::N, (t & 0x0080) > 0);

        if let Some(ins) = self.dispatch.get(&self.opcode) {
            if (ins.addressmode as usize) == (Cpu::imp as usize) {
                self.a = (t & 0x00FF) as u8;
            } else {
                self.bus.write(self.addr_abs, t & 0x00FF);
            }
        }

        0
    }

    fn pla(&mut self) -> usize {
        self.sp = self.sp.overflowing_add(1).0;
        self.a = self.bus.read(0x0100 + self.sp as u16);

        self.set_flag(Flags::Z, self.a == 0);
        self.set_flag(Flags::N, (self.a & 0b10000000) > 0);

        0
    }

    fn bvs(&mut self) -> usize {
        self.conditional_branch(Flags::V, true);

        0
    }

    fn sei(&mut self) -> usize {
        self.set_flag(Flags::I, true);

        0
    }

    fn sta(&mut self) -> usize {
        self.bus.write(self.addr_abs, self.a as u16);

        0
    }

    fn sty(&mut self) -> usize {
        self.bus.write(self.addr_abs, self.y as u16);

        0
    }

    fn stx(&mut self) -> usize {
        self.bus.write(self.addr_abs, self.x as u16);

        0
    }

    fn dey(&mut self) -> usize {
        self.y = self.y.overflowing_sub(1).0;

        self.set_flag(Flags::Z, self.y == 0);
        self.set_flag(Flags::N, (self.y & 0b10000000) > 0);

        0
    }

    fn txa(&mut self) -> usize {
        self.a = self.x;

        self.set_flag(Flags::Z, self.a == 0);
        self.set_flag(Flags::N, (self.a & 0b10000000) > 0);

        0
    }

    fn bcc(&mut self) -> usize {
        self.conditional_branch(Flags::C, false);

        0
    }

    fn tya(&mut self) -> usize {
        self.a = self.y;

        self.set_flag(Flags::Z, self.a == 0);
        self.set_flag(Flags::N, (self.a & 0b10000000) > 0);

        0
    }

    fn txs(&mut self) -> usize {
        self.sp = self.x;

        0
    }

    fn ldy(&mut self) -> usize {
        self.fetch();
        self.y = self.fetched;

        self.set_flag(Flags::Z, self.y == 0);
        self.set_flag(Flags::N, (self.y & 0b10000000) > 0);

        1
    }

    fn lda(&mut self) -> usize {
        self.fetch();
        self.a = self.fetched;

        self.set_flag(Flags::Z, self.a == 0);
        self.set_flag(Flags::N, (self.a & 0b10000000) > 0);

        1
    }

    fn ldx(&mut self) -> usize {
        self.fetch();
        self.x = self.fetched;

        self.set_flag(Flags::Z, self.x == 0);
        self.set_flag(Flags::N, (self.x & 0b10000000) > 0);

        1
    }

    fn tay(&mut self) -> usize {
        self.y = self.a;

        self.set_flag(Flags::Z, self.y == 0);
        self.set_flag(Flags::N, (self.y & 0b10000000) > 0);

        0
    }

    fn tax(&mut self) -> usize {
        self.x = self.a;

        self.set_flag(Flags::Z, self.x == 0);
        self.set_flag(Flags::N, (self.x & 0b10000000) > 0);

        0
    }

    fn bcs(&mut self) -> usize {
        self.conditional_branch(Flags::C, true);

        0
    }

    fn clv(&mut self) -> usize {
        self.set_flag(Flags::V, false);

        0
    }

    fn tsx(&mut self) -> usize {
        self.x = self.sp;

        self.set_flag(Flags::Z, self.x == 0);
        self.set_flag(Flags::N, (self.x & 0b10000000) > 0);

        0
    }

    fn cpy(&mut self) -> usize {
        self.fetch();

        self.set_flag(Flags::C, self.y >= self.fetched);
        self.set_flag(Flags::Z, self.y == self.fetched);
        self.set_flag(
            Flags::N,
            ((self.y.overflowing_sub(self.fetched).0) & 0b10000000) > 0,
        );

        0
    }

    fn cmp(&mut self) -> usize {
        self.fetch();

        self.set_flag(Flags::C, self.a >= self.fetched);
        self.set_flag(Flags::Z, self.a == self.fetched);
        self.set_flag(
            Flags::N,
            ((self.a.overflowing_sub(self.fetched).0) & 0b10000000) > 0,
        );

        0
    }

    fn dec(&mut self) -> usize {
        self.fetch();

        let value = self.fetched.overflowing_sub(1).0;
        self.bus.write(self.addr_abs, value as u16);

        self.set_flag(Flags::Z, value == 0);
        self.set_flag(Flags::N, (value & 0b10000000) > 0);

        0
    }

    fn iny(&mut self) -> usize {
        self.y = self.y.overflowing_add(1).0;

        self.set_flag(Flags::Z, self.y == 0);
        self.set_flag(Flags::N, (self.y & 0b10000000) > 0);

        0
    }

    fn dex(&mut self) -> usize {
        self.x = self.x.overflowing_sub(1).0;

        self.set_flag(Flags::Z, self.x == 0);
        self.set_flag(Flags::N, (self.x & 0b10000000) > 0);

        0
    }

    fn bne(&mut self) -> usize {
        self.conditional_branch(Flags::Z, false);

        0
    }

    fn cld(&mut self) -> usize {
        self.set_flag(Flags::D, false);

        0
    }

    fn nop(&mut self) -> usize {
        0
    }

    fn cpx(&mut self) -> usize {
        self.fetch();

        self.set_flag(Flags::C, self.x >= self.fetched);
        self.set_flag(Flags::Z, self.x == self.fetched);
        self.set_flag(
            Flags::N,
            ((self.x.overflowing_sub(self.fetched).0) & 0b10000000) > 0,
        );

        0
    }

    fn sbc(&mut self) -> usize {
        self.fetch();
        self.fetched = !self.fetched;

        let result: u16 = self.a as u16 + self.fetched as u16 + self.get_flag(Flags::C) as u16;
        let overflow =
            (!((self.a as u16) ^ (self.fetched as u16))) & ((self.a as u16) ^ result) & 0x80;

        self.set_flag(Flags::C, result > 0xFF);
        self.set_flag(Flags::V, overflow > 0);

        self.a = result as u8;

        self.set_flag(Flags::Z, self.a == 0);
        self.set_flag(Flags::N, self.a & 0b10000000 > 0);

        1
    }

    fn inc(&mut self) -> usize {
        self.fetch();

        let value: u8 = self.fetched.overflowing_add(1).0;
        self.bus.write(self.addr_abs, value as u16);

        self.set_flag(Flags::N, (value & 0b10000000) > 0);
        self.set_flag(Flags::Z, value == 0);

        0
    }

    fn inx(&mut self) -> usize {
        self.x = self.x.overflowing_add(1).0;

        self.set_flag(Flags::N, (self.x & 0b10000000) > 0);
        self.set_flag(Flags::Z, self.x == 0);

        1
    }

    fn beq(&mut self) -> usize {
        self.conditional_branch(Flags::Z, true);

        0
    }

    fn sed(&mut self) -> usize {
        self.set_flag(Flags::D, true);

        0
    }
}

#[cfg(test)]
mod tests {
    extern crate test_generator;

    use serde::Deserialize;
    use test_generator::test_resources;

    use std::fs;
    use std::fs::File;
    use std::io::BufReader;
    use std::path::PathBuf;

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
        name: String,
        #[serde(rename = "initial")]
        initial_state: CpuState,
        #[serde(rename = "final")]
        final_state: CpuState,
        cycles: Vec<(u16, u8, String)>,
    }

    #[test_resources("tests/*.json")]
    fn operation(resource: &str) {
        let file = File::open(resource).unwrap();
        let reader = BufReader::new(file);
        let test_cases: Vec<TestCase> =
            serde_json::from_reader(reader).expect("Problem reading file");

        for test_case in test_cases.iter() {
            // Filling CPU state
            let mut bus = Bus::new();
            let mut cpu = Cpu::new(&mut bus);
            cpu.reset();

            cpu.pc = test_case.initial_state.pc;
            cpu.sp = test_case.initial_state.s;
            cpu.a = test_case.initial_state.a;
            cpu.x = test_case.initial_state.x;
            cpu.y = test_case.initial_state.y;
            cpu.status = test_case.initial_state.p;

            cpu.cycles = 0;

            for (address, value) in test_case.initial_state.ram.iter() {
                cpu.bus.write(*address, *value as u16);
            }

            loop {
                cpu.clock();

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
                assert_eq!(cpu.bus.read(*address), *value)
            }
        }
    }
}
