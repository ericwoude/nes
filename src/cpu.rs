use crate::bus::Bus;

use std::collections::HashMap;

pub enum Flags {
    C = (1 << 0), // carry bit
    Z = (1 << 1), // zero
    I = (1 << 2), // disable interrupts
    D = (1 << 3), // decimal mode
    B = (1 << 4), // break
    U = (1 << 5), // unused
    V = (1 << 6), // overflow
    N = (1 << 7), // negeative
}

#[derive(Clone, Copy)]
struct Instruction {
    name: &'static str,
    operation: fn(&mut Cpu) -> usize,
    addressmode: fn(&mut Cpu) -> usize,
    cycles: usize,
}

pub struct Cpu {
    a: u8,
    x: u8,
    y: u8,
    sp: u8,
    pc: u16,
    status: u8,
    bus: Bus,

    fetched: u8,
    addr_abs: u16,
    addr_rel: u16,
    opcode: u8,
    cycles: usize,

    dispatch: HashMap<u8, Instruction>,
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            a: 0x00,
            x: 0x00,
            y: 0x00,
            sp: 0x00,
            pc: 0x0000,
            status: 0x00,
            bus: Bus::new(),

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
            let opcode = self.bus.read(self.pc);
            self.pc += 1;

            let instruction = self
                .dispatch
                .get(&opcode)
                .copied()
                .expect("Unknown instruction");
            self.cycles = instruction.cycles;

            let addr_cycles = (instruction.addressmode)(self);
            let op_cycles = (instruction.operation)(self);

            self.cycles += addr_cycles & op_cycles;
        }

        self.cycles -= 1;
    }

    fn reset(&self) {
        todo!()
    }

    fn irq(&self) {
        todo!()
    }

    fn nmi(&self) {
        todo!()
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
        self.pc += 1;

        0
    }

    /// Zero page addressing uses the high byte to address
    /// a specific page and the low byte to offset into that page
    fn zp0(&mut self) -> usize {
        self.addr_abs = self.bus.read(self.pc) as u16;
        self.pc += 1;
        self.addr_abs &= 0x00FF;

        0
    }

    /// Zero page addressing with register x as extra offset.
    fn zpx(&mut self) -> usize {
        self.addr_abs = self.bus.read(self.pc) as u16 + self.x as u16;
        self.pc += 1;
        self.addr_abs &= 0x00FF;

        0
    }

    /// Zero page addressing with register y as extra offset.
    fn zpy(&mut self) -> usize {
        self.addr_abs = self.bus.read(self.pc) as u16 + self.y as u16;
        self.pc += 1;
        self.addr_abs &= 0x00FF;

        0
    }

    /// Relative addressing uses the second byte (signed) as an offset for the next
    /// instruction, which can range from -127 to +127 relative to the program
    /// counter.
    fn rel(&mut self) -> usize {
        self.addr_rel = self.bus.read(self.pc) as u16;
        self.pc += 1;

        if (self.addr_rel & 0b10000000) > 0 {
            self.addr_rel |= 0xFF00;
        }

        0
    }

    /// Absolute addressing fetches the full 16 bit address
    /// from region in memory at the program counter.
    fn abs(&mut self) -> usize {
        let lo: u16 = self.bus.read(self.pc) as u16;
        self.pc += 1;
        let hi: u16 = self.bus.read(self.pc) as u16;
        self.pc += 1;

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
        self.pc += 1;
        let hi: u16 = self.bus.read(self.pc) as u16;
        self.pc += 1;

        self.addr_abs = ((hi << 8) | lo) + self.x as u16;

        if (self.addr_abs & 0xFF00) != (hi << 8) {
            1
        } else {
            0
        }
    }

    /// Absolute addressing with y offset.
    fn aby(&mut self) -> usize {
        let lo: u16 = self.bus.read(self.pc) as u16;
        self.pc += 1;
        let hi: u16 = self.bus.read(self.pc) as u16;
        self.pc += 1;

        self.addr_abs = ((hi << 8) | lo) + self.y as u16;

        if (self.addr_abs & 0xFF00) != (hi << 8) {
            1
        } else {
            0
        }
    }

    /// Indirect addressing first fetches a 16 bit pointer from
    /// location in memory at the program counter. Then it uses the
    /// pointer to locate to the actual address in memory.
    ///
    /// Simulates page boundary hardware bug.
    #[allow(arithmetic_overflow)]
    fn ind(&mut self) -> usize {
        let lo: u16 = self.bus.read(self.pc) as u16;
        self.pc += 1;
        let hi: u16 = self.bus.read(self.pc) as u16;
        self.pc += 1;

        let p: u16 = (hi << 8) | lo;

        self.addr_abs = if lo == 0x00FF {
            ((self.bus.read(p & 0x00FF) << 8) | self.bus.read(p)) as u16
        } else {
            ((self.bus.read(p + 1) << 8) | self.bus.read(p)) as u16
        };

        0
    }

    /// Zero page indirect addressing with register x offset.
    fn izx(&mut self) -> usize {
        let p: u16 = self.bus.read(self.pc) as u16;
        self.pc += 1;

        let lo: u16 = self.bus.read((p + self.x as u16) & 0x00FF) as u16;
        let hi: u16 = self.bus.read((p + (self.x as u16) + 1) & 0x00FF) as u16;

        self.addr_abs = (hi << 8) | lo;

        0
    }

    /// Zero page indirect addressing with register y offset. Different from izx,
    /// the register y offset is added onto the fetched 16 bits from memory location.
    /// It may overflow into the next page, requiring an extra cpu cycle to complete.
    fn izy(&mut self) -> usize {
        let p: u16 = self.bus.read(self.pc) as u16;
        self.pc += 1;

        let lo: u16 = self.bus.read(p & 0x00FF) as u16;
        let hi: u16 = self.bus.read((p + 1) & 0x00FF) as u16;

        self.addr_abs = ((hi << 8) | lo) + self.y as u16;

        if (self.addr_abs & 0xFF00) != (hi << 8) {
            1
        } else {
            0
        }
    }

    // OPCODE FUNCTIONS

    fn fetch(&mut self) -> u8 {
        let instruction = self.dispatch.get(&self.opcode).expect("Unknown opcode");

        if instruction.addressmode as usize != Cpu::imm as usize {
            self.fetched = self.bus.read(self.addr_abs);
        }

        self.fetched
    }

    fn xxx(&mut self) -> usize {
        todo!()
    }

    fn brk(&mut self) -> usize {
        todo!()
    }

    fn ora(&mut self) -> usize {
        todo!()
    }

    fn asl(&mut self) -> usize {
        todo!()
    }

    fn php(&mut self) -> usize {
        todo!()
    }

    fn bpl(&mut self) -> usize {
        todo!()
    }

    fn clc(&mut self) -> usize {
        todo!()
    }

    fn jsr(&mut self) -> usize {
        todo!()
    }

    fn and(&mut self) -> usize {
        self.fetch();
        self.a &= self.fetched;

        self.set_flag(Flags::Z, self.a == 0x00);
        self.set_flag(Flags::N, (self.a & 0b10000000) > 0);

        1
    }
    fn bit(&mut self) -> usize {
        todo!()
    }

    fn rol(&mut self) -> usize {
        todo!()
    }

    fn plp(&mut self) -> usize {
        todo!()
    }

    fn bmi(&mut self) -> usize {
        todo!()
    }

    fn sec(&mut self) -> usize {
        todo!()
    }

    fn rti(&mut self) -> usize {
        todo!()
    }

    fn eor(&mut self) -> usize {
        todo!()
    }

    fn lsr(&mut self) -> usize {
        todo!()
    }

    fn pha(&mut self) -> usize {
        todo!()
    }

    fn jmp(&mut self) -> usize {
        todo!()
    }

    fn bvc(&mut self) -> usize {
        todo!()
    }

    fn cli(&mut self) -> usize {
        todo!()
    }

    fn rts(&mut self) -> usize {
        todo!()
    }

    fn adc(&mut self) -> usize {
        self.fetch();

        let res: u16 = self.a as u16 + self.fetched as u16 + self.get_flag(Flags::C) as u16;
        let overflow: bool =
            ((!((self.a as u16) ^ (self.fetched as u16)) & ((self.a as u16) ^ res)) & 0x0080) > 0;

        self.set_flag(Flags::C, res > 255);
        self.set_flag(Flags::Z, (res & 0x00FF) == 0);
        self.set_flag(Flags::N, (res & 0x80) > 0);
        self.set_flag(Flags::V, overflow);

        self.a = (res & 0x00FF) as u8;

        1
    }

    fn ror(&mut self) -> usize {
        todo!()
    }

    fn pla(&mut self) -> usize {
        todo!()
    }

    fn bvs(&mut self) -> usize {
        todo!()
    }

    fn sei(&mut self) -> usize {
        todo!()
    }

    fn sta(&mut self) -> usize {
        todo!()
    }

    fn sty(&mut self) -> usize {
        todo!()
    }

    fn stx(&mut self) -> usize {
        todo!()
    }

    fn dey(&mut self) -> usize {
        todo!()
    }

    fn txa(&mut self) -> usize {
        self.a = self.x;

        self.set_flag(Flags::Z, self.a == 0);
        self.set_flag(Flags::N, (self.a & 0b10000000) > 0);

        0
    }

    fn bcc(&mut self) -> usize {
        todo!()
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
        todo!()
    }

    fn clv(&mut self) -> usize {
        todo!()
    }

    fn tsx(&mut self) -> usize {
        self.x = self.sp;

        self.set_flag(Flags::Z, self.x == 0);
        self.set_flag(Flags::N, (self.x & 0b10000000) > 0);

        0
    }

    fn cpy(&mut self) -> usize {
        todo!()
    }

    fn cmp(&mut self) -> usize {
        todo!()
    }

    fn dec(&mut self) -> usize {
        todo!()
    }

    fn iny(&mut self) -> usize {
        todo!()
    }

    fn dex(&mut self) -> usize {
        todo!()
    }

    fn bne(&mut self) -> usize {
        todo!()
    }

    fn cld(&mut self) -> usize {
        todo!()
    }

    fn nop(&mut self) -> usize {
        todo!()
    }

    fn cpx(&mut self) -> usize {
        todo!()
    }

    fn sbc(&mut self) -> usize {
        self.fetch();

        let operand: u16 = (self.fetched as u16) ^ 0x00FF;
        let res: u16 = self.a as u16 + operand + self.get_flag(Flags::C) as u16;
        let overflow: bool =
            ((!((self.a as u16) ^ (self.fetched as u16)) & ((self.a as u16) ^ res)) & 0x0080) > 0;

        self.set_flag(Flags::C, res > 255);
        self.set_flag(Flags::Z, (res & 0x00FF) == 0);
        self.set_flag(Flags::N, (res & 0x80) > 0);
        self.set_flag(Flags::V, overflow);

        self.a = (res & 0x00FF) as u8;

        1
    }

    fn inc(&mut self) -> usize {
        todo!()
    }

    fn inx(&mut self) -> usize {
        todo!()
    }

    fn beq(&mut self) -> usize {
        todo!()
    }

    fn sed(&mut self) -> usize {
        todo!()
    }
}
