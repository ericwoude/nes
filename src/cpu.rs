use crate::bus::Bus;

use std::collections::HashMap;

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
            bus,

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
                    0x01,
                    Instruction {
                        name: "ora",
                        operation: Cpu::ora,
                        addressmode: Cpu::izx,
                        cycles: 6,
                    },
                ),
                (
                    0x02,
                    Instruction {
                        name: "kil",
                        operation: Cpu::kil,
                        addressmode: Cpu::imp,
                        cycles: 1,
                    },
                ),
                (
                    0x03,
                    Instruction {
                        name: "slo",
                        operation: Cpu::slo,
                        addressmode: Cpu::izx,
                        cycles: 8,
                    },
                ),
                (
                    0x04,
                    Instruction {
                        name: "ign d",
                        operation: Cpu::nop,
                        addressmode: Cpu::zp0,
                        cycles: 3,
                    },
                ),
                (
                    0x05,
                    Instruction {
                        name: "ora",
                        operation: Cpu::ora,
                        addressmode: Cpu::zp0,
                        cycles: 3,
                    },
                ),
                (
                    0x06,
                    Instruction {
                        name: "asl",
                        operation: Cpu::asl,
                        addressmode: Cpu::zp0,
                        cycles: 5,
                    },
                ),
                (
                    0x07,
                    Instruction {
                        name: "slo",
                        operation: Cpu::slo,
                        addressmode: Cpu::zp0,
                        cycles: 5,
                    },
                ),
                (
                    0x08,
                    Instruction {
                        name: "php",
                        operation: Cpu::php,
                        addressmode: Cpu::imp,
                        cycles: 3,
                    },
                ),
                (
                    0x09,
                    Instruction {
                        name: "ora",
                        operation: Cpu::ora,
                        addressmode: Cpu::imm,
                        cycles: 2,
                    },
                ),
                (
                    0x0a,
                    Instruction {
                        name: "asl",
                        operation: Cpu::asl,
                        addressmode: Cpu::imp,
                        cycles: 2,
                    },
                ),
                (
                    0x0b,
                    Instruction {
                        name: "anc",
                        operation: Cpu::anc,
                        addressmode: Cpu::imm,
                        cycles: 2,
                    },
                ),
                (
                    0x0c,
                    Instruction {
                        name: "ign a",
                        operation: Cpu::nop,
                        addressmode: Cpu::abs,
                        cycles: 4,
                    },
                ),
                (
                    0x0d,
                    Instruction {
                        name: "ora",
                        operation: Cpu::ora,
                        addressmode: Cpu::abs,
                        cycles: 4,
                    },
                ),
                (
                    0x0e,
                    Instruction {
                        name: "asl",
                        operation: Cpu::asl,
                        addressmode: Cpu::abs,
                        cycles: 6,
                    },
                ),
                (
                    0x0f,
                    Instruction {
                        name: "slo",
                        operation: Cpu::slo,
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
                    0x12,
                    Instruction {
                        name: "kil",
                        operation: Cpu::kil,
                        addressmode: Cpu::imp,
                        cycles: 1,
                    },
                ),
                (
                    0x13,
                    Instruction {
                        name: "slo",
                        operation: Cpu::slo,
                        addressmode: Cpu::izy,
                        cycles: 8,
                    },
                ),
                (
                    0x14,
                    Instruction {
                        name: "ign d,X",
                        operation: Cpu::nop,
                        addressmode: Cpu::zpx,
                        cycles: 4,
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
                    0x17,
                    Instruction {
                        name: "slo",
                        operation: Cpu::slo,
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
                    0x1a,
                    Instruction {
                        name: "nop",
                        operation: Cpu::nop,
                        addressmode: Cpu::imp,
                        cycles: 2,
                    },
                ),
                (
                    0x1b,
                    Instruction {
                        name: "slo",
                        operation: Cpu::slo,
                        addressmode: Cpu::aby,
                        cycles: 7,
                    },
                ),
                (
                    0x1c,
                    Instruction {
                        name: "ign a,X",
                        operation: Cpu::nop,
                        addressmode: Cpu::abx,
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
                    0x1f,
                    Instruction {
                        name: "slo",
                        operation: Cpu::slo,
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
                    0x22,
                    Instruction {
                        name: "kil",
                        operation: Cpu::kil,
                        addressmode: Cpu::imp,
                        cycles: 1,
                    },
                ),
                (
                    0x23,
                    Instruction {
                        name: "rla",
                        operation: Cpu::rla,
                        addressmode: Cpu::izx,
                        cycles: 8,
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
                    0x27,
                    Instruction {
                        name: "rla",
                        operation: Cpu::rla,
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
                    0x2b,
                    Instruction {
                        name: "anc",
                        operation: Cpu::anc,
                        addressmode: Cpu::imm,
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
                    0x2f,
                    Instruction {
                        name: "rla",
                        operation: Cpu::rla,
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
                    0x32,
                    Instruction {
                        name: "kil",
                        operation: Cpu::kil,
                        addressmode: Cpu::imp,
                        cycles: 1,
                    },
                ),
                (
                    0x33,
                    Instruction {
                        name: "rla",
                        operation: Cpu::rla,
                        addressmode: Cpu::izy,
                        cycles: 8,
                    },
                ),
                (
                    0x34,
                    Instruction {
                        name: "ign d,X",
                        operation: Cpu::nop,
                        addressmode: Cpu::zpx,
                        cycles: 4,
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
                    0x37,
                    Instruction {
                        name: "rla",
                        operation: Cpu::rla,
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
                    0x3a,
                    Instruction {
                        name: "NOP",
                        operation: Cpu::nop,
                        addressmode: Cpu::imp,
                        cycles: 2,
                    },
                ),
                (
                    0x3b,
                    Instruction {
                        name: "rla",
                        operation: Cpu::rla,
                        addressmode: Cpu::aby,
                        cycles: 7,
                    },
                ),
                (
                    0x3c,
                    Instruction {
                        name: "ign a,X",
                        operation: Cpu::nop,
                        addressmode: Cpu::abx,
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
                    0x3f,
                    Instruction {
                        name: "rla",
                        operation: Cpu::rla,
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
                    0x42,
                    Instruction {
                        name: "kil",
                        operation: Cpu::kil,
                        addressmode: Cpu::imp,
                        cycles: 1,
                    },
                ),
                (
                    0x43,
                    Instruction {
                        name: "sre",
                        operation: Cpu::sre,
                        addressmode: Cpu::izx,
                        cycles: 8,
                    },
                ),
                (
                    0x44,
                    Instruction {
                        name: "ign d",
                        operation: Cpu::nop,
                        addressmode: Cpu::zp0,
                        cycles: 3,
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
                    0x47,
                    Instruction {
                        name: "sre",
                        operation: Cpu::sre,
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
                    0x4b,
                    Instruction {
                        name: "alr",
                        operation: Cpu::alr,
                        addressmode: Cpu::imm,
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
                    0x4f,
                    Instruction {
                        name: "sre",
                        operation: Cpu::sre,
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
                    0x52,
                    Instruction {
                        name: "kil",
                        operation: Cpu::kil,
                        addressmode: Cpu::imp,
                        cycles: 1,
                    },
                ),
                (
                    0x53,
                    Instruction {
                        name: "sre",
                        operation: Cpu::sre,
                        addressmode: Cpu::izy,
                        cycles: 8,
                    },
                ),
                (
                    0x54,
                    Instruction {
                        name: "ign d,X",
                        operation: Cpu::nop,
                        addressmode: Cpu::zpx,
                        cycles: 4,
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
                    0x57,
                    Instruction {
                        name: "sre",
                        operation: Cpu::sre,
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
                    0x5b,
                    Instruction {
                        name: "sre",
                        operation: Cpu::sre,
                        addressmode: Cpu::aby,
                        cycles: 7,
                    },
                ),
                (
                    0x5c,
                    Instruction {
                        name: "ign a,X",
                        operation: Cpu::nop,
                        addressmode: Cpu::abx,
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
                    0x5a,
                    Instruction {
                        name: "nop",
                        operation: Cpu::nop,
                        addressmode: Cpu::imp,
                        cycles: 2,
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
                    0x5f,
                    Instruction {
                        name: "sre",
                        operation: Cpu::sre,
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
                    0x62,
                    Instruction {
                        name: "kil",
                        operation: Cpu::kil,
                        addressmode: Cpu::imp,
                        cycles: 1,
                    },
                ),
                (
                    0x63,
                    Instruction {
                        name: "rra",
                        operation: Cpu::rra,
                        addressmode: Cpu::izx,
                        cycles: 8,
                    },
                ),
                (
                    0x64,
                    Instruction {
                        name: "ign d",
                        operation: Cpu::nop,
                        addressmode: Cpu::zp0,
                        cycles: 3,
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
                    0x67,
                    Instruction {
                        name: "rra",
                        operation: Cpu::rra,
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
                    0x6b,
                    Instruction {
                        name: "arr",
                        operation: Cpu::arr,
                        addressmode: Cpu::imm,
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
                    0x6f,
                    Instruction {
                        name: "rra",
                        operation: Cpu::rra,
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
                    0x72,
                    Instruction {
                        name: "kil",
                        operation: Cpu::kil,
                        addressmode: Cpu::imp,
                        cycles: 1,
                    },
                ),
                (
                    0x73,
                    Instruction {
                        name: "rra",
                        operation: Cpu::rra,
                        addressmode: Cpu::izy,
                        cycles: 8,
                    },
                ),
                (
                    0x74,
                    Instruction {
                        name: "ign d,X",
                        operation: Cpu::nop,
                        addressmode: Cpu::zpx,
                        cycles: 4,
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
                    0x77,
                    Instruction {
                        name: "rra",
                        operation: Cpu::rra,
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
                    0x7a,
                    Instruction {
                        name: "nop",
                        operation: Cpu::nop,
                        addressmode: Cpu::imp,
                        cycles: 2,
                    },
                ),
                (
                    0x7b,
                    Instruction {
                        name: "rra",
                        operation: Cpu::rra,
                        addressmode: Cpu::aby,
                        cycles: 7,
                    },
                ),
                (
                    0x7c,
                    Instruction {
                        name: "ign a,X",
                        operation: Cpu::nop,
                        addressmode: Cpu::abx,
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
                    0x7f,
                    Instruction {
                        name: "rra",
                        operation: Cpu::rra,
                        addressmode: Cpu::abx,
                        cycles: 7,
                    },
                ),
                (
                    0x80,
                    Instruction {
                        name: "skb #i",
                        operation: Cpu::nop,
                        addressmode: Cpu::imm,
                        cycles: 2,
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
                    0x82,
                    Instruction {
                        name: "skb #i",
                        operation: Cpu::nop,
                        addressmode: Cpu::imm,
                        cycles: 2,
                    },
                ),
                (
                    0x83,
                    Instruction {
                        name: "sax",
                        operation: Cpu::sax,
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
                    0x87,
                    Instruction {
                        name: "sax",
                        operation: Cpu::sax,
                        addressmode: Cpu::zp0,
                        cycles: 3,
                    },
                ),
                (
                    0x89,
                    Instruction {
                        name: "skb #i",
                        operation: Cpu::nop,
                        addressmode: Cpu::imm,
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
                    0x8b,
                    Instruction {
                        name: "xaa",
                        operation: Cpu::xaa,
                        addressmode: Cpu::imm,
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
                    0x8f,
                    Instruction {
                        name: "sax",
                        operation: Cpu::sax,
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
                    0x92,
                    Instruction {
                        name: "kil",
                        operation: Cpu::kil,
                        addressmode: Cpu::imp,
                        cycles: 1,
                    },
                ),
                (
                    0x93,
                    Instruction {
                        name: "ahx",
                        operation: Cpu::sha,
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
                    0x97,
                    Instruction {
                        name: "sax",
                        operation: Cpu::sax,
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
                    0x9F,
                    Instruction {
                        name: "sha",
                        operation: Cpu::sha,
                        addressmode: Cpu::aby,
                        cycles: 5,
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
                    0x9b,
                    Instruction {
                        name: "tas",
                        operation: Cpu::tas,
                        addressmode: Cpu::aby,
                        cycles: 5,
                    },
                ),
                (
                    0x9c,
                    Instruction {
                        name: "shy",
                        operation: Cpu::shy,
                        addressmode: Cpu::abx,
                        cycles: 5,
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
                    0x9e,
                    Instruction {
                        name: "shx",
                        operation: Cpu::shx,
                        addressmode: Cpu::aby,
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
                    0xa3,
                    Instruction {
                        name: "lax",
                        operation: Cpu::lax,
                        addressmode: Cpu::izx,
                        cycles: 6,
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
                    0xa7,
                    Instruction {
                        name: "lax",
                        operation: Cpu::lax,
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
                    0xab,
                    Instruction {
                        name: "lxa",
                        operation: Cpu::lxa,
                        addressmode: Cpu::imm,
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
                    0xaf,
                    Instruction {
                        name: "lax",
                        operation: Cpu::lax,
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
                    0xb2,
                    Instruction {
                        name: "kil",
                        operation: Cpu::kil,
                        addressmode: Cpu::imp,
                        cycles: 1,
                    },
                ),
                (
                    0xb3,
                    Instruction {
                        name: "lax",
                        operation: Cpu::lax,
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
                    0xb7,
                    Instruction {
                        name: "lax",
                        operation: Cpu::lax,
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
                    0xbb,
                    Instruction {
                        name: "las",
                        operation: Cpu::las,
                        addressmode: Cpu::aby,
                        cycles: 4,
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
                    0xbf,
                    Instruction {
                        name: "lax",
                        operation: Cpu::lax,
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
                    0xc2,
                    Instruction {
                        name: "skb #i",
                        operation: Cpu::nop,
                        addressmode: Cpu::imm,
                        cycles: 2,
                    },
                ),
                (
                    0xc3,
                    Instruction {
                        name: "dcp",
                        operation: Cpu::dcp,
                        addressmode: Cpu::izx,
                        cycles: 8,
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
                    0xc7,
                    Instruction {
                        name: "dcp",
                        operation: Cpu::dcp,
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
                    0xcb,
                    Instruction {
                        name: "axs",
                        operation: Cpu::axs,
                        addressmode: Cpu::imm,
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
                    0xcf,
                    Instruction {
                        name: "dcp",
                        operation: Cpu::dcp,
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
                    0xd2,
                    Instruction {
                        name: "kil",
                        operation: Cpu::kil,
                        addressmode: Cpu::imp,
                        cycles: 1,
                    },
                ),
                (
                    0xd3,
                    Instruction {
                        name: "dcp",
                        operation: Cpu::dcp,
                        addressmode: Cpu::izy,
                        cycles: 8,
                    },
                ),
                (
                    0xd4,
                    Instruction {
                        name: "ign d,X",
                        operation: Cpu::nop,
                        addressmode: Cpu::zpx,
                        cycles: 4,
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
                    0xd7,
                    Instruction {
                        name: "dcp",
                        operation: Cpu::dcp,
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
                    0xdb,
                    Instruction {
                        name: "dcp",
                        operation: Cpu::dcp,
                        addressmode: Cpu::aby,
                        cycles: 7,
                    },
                ),
                (
                    0xdc,
                    Instruction {
                        name: "ign a,X",
                        operation: Cpu::nop,
                        addressmode: Cpu::abx,
                        cycles: 4,
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
                    0xdf,
                    Instruction {
                        name: "dcp",
                        operation: Cpu::dcp,
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
                    0xe2,
                    Instruction {
                        name: "skb #i",
                        operation: Cpu::nop,
                        addressmode: Cpu::imm,
                        cycles: 2,
                    },
                ),
                (
                    0xe3,
                    Instruction {
                        name: "isc",
                        operation: Cpu::isc,
                        addressmode: Cpu::izx,
                        cycles: 8,
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
                    0xe7,
                    Instruction {
                        name: "isc",
                        operation: Cpu::isc,
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
                    0xeb,
                    Instruction {
                        name: "nop",
                        operation: Cpu::sbc,
                        addressmode: Cpu::imm,
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
                    0xef,
                    Instruction {
                        name: "isc",
                        operation: Cpu::isc,
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
                    0xf2,
                    Instruction {
                        name: "kil",
                        operation: Cpu::kil,
                        addressmode: Cpu::imp,
                        cycles: 1,
                    },
                ),
                (
                    0xf3,
                    Instruction {
                        name: "isc",
                        operation: Cpu::isc,
                        addressmode: Cpu::izy,
                        cycles: 8,
                    },
                ),
                (
                    0xf4,
                    Instruction {
                        name: "ign d,X",
                        operation: Cpu::nop,
                        addressmode: Cpu::zpx,
                        cycles: 4,
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
                    0xf7,
                    Instruction {
                        name: "isc",
                        operation: Cpu::isc,
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
                    0xfb,
                    Instruction {
                        name: "isc",
                        operation: Cpu::isc,
                        addressmode: Cpu::aby,
                        cycles: 7,
                    },
                ),
                (
                    0xfc,
                    Instruction {
                        name: "ign a,X",
                        operation: Cpu::nop,
                        addressmode: Cpu::abx,
                        cycles: 4,
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
                (
                    0xff,
                    Instruction {
                        name: "isc",
                        operation: Cpu::isc,
                        addressmode: Cpu::abx,
                        cycles: 7,
                    },
                ),
            ]),
        }
    }

    fn set_flag(&mut self, flag: Flag, value: bool) {
        match value {
            true => self.status |= flag as u8,
            false => self.status &= !(flag as u8),
        }
    }

    fn get_flag(&self, flag: Flag) -> bool {
        self.status & (flag as u8) != 0
    }

    pub fn clock(&mut self) {
        if self.cycles == 0 {
            self.opcode = self.bus.read(self.pc);
            self.pc = self.pc.wrapping_add(1);
            self.set_flag(Flag::U, true);

            let instruction = self
                .dispatch
                .get(&self.opcode)
                .copied()
                .expect("opcode should be in dispatch table");
            self.cycles = instruction.cycles;

            let addr_cycles = (instruction.addressmode)(self);
            let op_cycles = (instruction.operation)(self);

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

    fn irq(&mut self) {
        if !self.get_flag(Flag::I) {
            self.nmi();
        }
    }

    fn nmi(&mut self) {
        self.bus
            .write(0x0100 + self.sp as u16, ((self.pc >> 8) & 0x00FF) as u8);
        self.sp = self.sp.wrapping_sub(1);
        self.bus
            .write(0x0100 + self.sp as u16, (self.pc & 0x00FF) as u8);
        self.sp = self.sp.wrapping_sub(1);

        self.set_flag(Flag::B, false);
        self.set_flag(Flag::U, true);
        self.set_flag(Flag::I, true);
        self.bus.write(0x0100 + self.sp as u16, self.status);
        self.sp = self.sp.wrapping_sub(1);

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
        self.pc = self.pc.wrapping_add(1);

        0
    }

    /// Zero page addressing uses the high byte to address
    /// a specific page and the low byte to offset into that page
    fn zp0(&mut self) -> usize {
        self.addr_abs = self.bus.read(self.pc) as u16;
        self.pc = self.pc.wrapping_add(1);

        self.addr_abs &= 0x00FF;

        0
    }

    /// Zero page addressing with register x as extra offset.
    fn zpx(&mut self) -> usize {
        self.addr_abs = self.bus.read(self.pc) as u16 + self.x as u16;
        self.pc = self.pc.wrapping_add(1);
        self.addr_abs &= 0x00FF;

        0
    }

    /// Zero page addressing with register y as extra offset.
    fn zpy(&mut self) -> usize {
        self.addr_abs = self.bus.read(self.pc) as u16 + self.y as u16;
        self.pc = self.pc.wrapping_add(1);
        self.addr_abs &= 0x00FF;

        0
    }

    /// Relative addressing uses the second byte (signed) as an offset for the next
    /// instruction, which can range from -127 to +127 relative to the program
    /// counter.
    fn rel(&mut self) -> usize {
        self.addr_rel = self.bus.read(self.pc) as u16;
        self.pc = self.pc.wrapping_add(1);

        if (self.addr_rel & 0b10000000) > 0 {
            self.addr_rel |= 0xFF00;
        }

        0
    }

    /// Absolute addressing fetches the full 16 bit address
    /// from region in memory at the program counter.
    fn abs(&mut self) -> usize {
        let lo: u16 = self.bus.read(self.pc) as u16;
        self.pc = self.pc.wrapping_add(1);
        let hi: u16 = self.bus.read(self.pc) as u16;
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
    fn abx(&mut self) -> usize {
        let lo: u16 = self.bus.read(self.pc) as u16;
        self.pc = self.pc.wrapping_add(1);
        let hi: u16 = self.bus.read(self.pc) as u16;
        self.pc = self.pc.wrapping_add(1);

        self.addr_abs = ((hi << 8) | lo).wrapping_add(self.x as u16);

        if (self.addr_abs & 0xFF00) != (hi << 8) {
            1
        } else {
            0
        }
    }

    /// Absolute addressing with y offset.
    fn aby(&mut self) -> usize {
        let lo: u16 = self.bus.read(self.pc) as u16;
        self.pc = self.pc.wrapping_add(1);
        let hi: u16 = self.bus.read(self.pc) as u16;
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
    fn ind(&mut self) -> usize {
        let lo: u16 = self.bus.read(self.pc) as u16;
        self.pc = self.pc.wrapping_add(1);
        let hi: u16 = self.bus.read(self.pc) as u16;
        self.pc = self.pc.wrapping_add(1);
        let target = hi.wrapping_shl(8) | lo;

        self.addr_abs = if lo == 0x00FF {
            let effective_lo = self.bus.read(target) as u16;
            let effective_hi = self.bus.read(target & 0xFF00) as u16;

            effective_hi.wrapping_shl(8) | effective_lo
        } else {
            let effective_lo = self.bus.read(target) as u16;
            let effective_hi = self.bus.read(target + 1) as u16;

            effective_hi.wrapping_shl(8) | effective_lo
        };

        0
    }

    /// Zero page indirect addressing with register x offset.
    fn izx(&mut self) -> usize {
        let p: u16 = self.bus.read(self.pc) as u16;
        self.pc = self.pc.wrapping_add(1);

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
        self.pc = self.pc.wrapping_add(1);

        let lo = self.bus.read(t & 0x00FF) as u16;
        let hi = self.bus.read((t + 1) & 0x00FF) as u16;

        self.addr_abs = ((hi << 8) | lo).wrapping_add(self.y as u16);

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

    fn brk(&mut self) -> usize {
        self.bus
            .write(0x0100 + self.sp as u16, ((self.pc >> 8) & 0x00FF) as u8);
        self.sp = self.sp.wrapping_sub(1);
        self.bus
            .write(0x0100 + self.sp as u16, (self.pc & 0x00FF) as u8);
        self.sp = self.sp.wrapping_sub(1);

        self.set_flag(Flag::B, true);
        self.bus.write(0x0100 + self.sp as u16, self.status);
        self.sp = self.sp.wrapping_sub(1);
        self.set_flag(Flag::B, false);

        // self.pc = self.bus.read(0xFFFE) as u16 | (self.bus.read(0xFFFF as u16) as u16) << 8;
        let lo = self.bus.read(0xFFFEu16) as u16;
        let hi = self.bus.read(0xFFFFu16) as u16;
        let addr = hi << 8 | lo;
        self.pc = addr;
        self.set_flag(Flag::I, true);
        0
    }

    fn ora(&mut self) -> usize {
        self.fetch();

        self.a |= self.fetched;

        self.set_flag(Flag::Z, self.a == 0);
        self.set_flag(Flag::N, (self.a & 0b10000000) > 0);

        0
    }

    fn asl(&mut self) -> usize {
        self.fetch();
        self.set_flag(Flag::C, (self.fetched & 0b10000000) > 0);

        let t: u8 = self.fetched.wrapping_shl(1);

        self.set_flag(Flag::Z, t == 0);
        self.set_flag(Flag::N, (t & 0b10000000) > 0);

        if let Some(ins) = self.dispatch.get(&self.opcode) {
            if (ins.addressmode as usize) == (Cpu::imp as usize) {
                self.a = t;
            } else {
                self.bus.write(self.addr_abs, t);
            }
        }

        0
    }

    fn php(&mut self) -> usize {
        self.bus.write(
            0x0100 + self.sp as u16,
            self.status | Flag::B as u8 | Flag::U as u8,
        );
        self.sp = self.sp.wrapping_sub(1);

        self.set_flag(Flag::B, false);
        self.set_flag(Flag::U, false);

        0
    }

    fn bpl(&mut self) -> usize {
        self.conditional_branch(Flag::N, false);

        0
    }

    fn clc(&mut self) -> usize {
        self.set_flag(Flag::C, false);

        0
    }

    fn jsr(&mut self) -> usize {
        self.pc = self.pc.wrapping_sub(1);

        self.bus.write(
            0x0100 + self.sp as u16,
            (self.pc.wrapping_shr(8) & 0x00FF) as u8,
        );
        self.sp = self.sp.wrapping_sub(1);
        self.bus
            .write(0x0100 + self.sp as u16, (self.pc & 0x00FF) as u8);
        self.sp = self.sp.wrapping_sub(1);

        self.pc = self.addr_abs;

        0
    }

    fn and(&mut self) -> usize {
        self.fetch();
        self.a &= self.fetched;

        self.set_flag(Flag::Z, self.a == 0x00);
        self.set_flag(Flag::N, (self.a & 0b10000000) > 0);

        1
    }

    fn bit(&mut self) -> usize {
        self.fetch();

        self.set_flag(Flag::Z, (self.a & self.fetched) == 0);
        self.set_flag(Flag::N, self.fetched & (1 << 7) > 0);
        self.set_flag(Flag::V, self.fetched & (1 << 6) > 0);

        0
    }

    fn rol(&mut self) -> usize {
        self.fetch();

        let overflow = (self.fetched & 0b10000000) > 0;
        let operand = self.fetched.wrapping_shl(1) | self.get_flag(Flag::C) as u8;

        self.set_flag(Flag::C, overflow);
        self.set_flag(Flag::N, (operand & 0b10000000) > 0);
        self.set_flag(Flag::Z, operand == 0);

        if let Some(ins) = self.dispatch.get(&self.opcode) {
            if (ins.addressmode as usize) == (Cpu::imp as usize) {
                self.a = operand;
            } else {
                self.bus.write(self.addr_abs, operand);
            }
        }

        0
    }

    fn plp(&mut self) -> usize {
        self.sp = self.sp.wrapping_add(1);
        self.status = self.bus.read(0x0100 + self.sp as u16);

        self.set_flag(Flag::U, true);
        self.set_flag(Flag::B, false);

        0
    }

    fn bmi(&mut self) -> usize {
        self.conditional_branch(Flag::N, true);

        0
    }

    fn sec(&mut self) -> usize {
        self.set_flag(Flag::C, true);

        1
    }

    fn rti(&mut self) -> usize {
        self.sp = self.sp.wrapping_add(1);
        self.status = self.bus.read(0x0100 + self.sp as u16);
        self.status &= !(Flag::B as u8);
        self.status &= !(Flag::U as u8);

        self.sp = self.sp.wrapping_add(1);
        self.pc = self.bus.read(0x0100 + self.sp as u16) as u16;
        self.sp = self.sp.wrapping_add(1);
        self.pc |= (self.bus.read(0x0100 + self.sp as u16) as u16).wrapping_shl(8);

        0
    }

    fn eor(&mut self) -> usize {
        self.fetch();

        self.a ^= self.fetched;

        self.set_flag(Flag::Z, self.a == 0);
        self.set_flag(Flag::N, (self.a & 0b10000000) > 1);

        0
    }

    fn lsr(&mut self) -> usize {
        self.fetch();
        self.set_flag(Flag::C, (self.fetched & 0x0001) > 0);

        let t: u8 = self.fetched.wrapping_shr(1);

        self.set_flag(Flag::Z, t == 0);
        self.set_flag(Flag::N, (t & 0b10000000) > 1);

        if let Some(ins) = self.dispatch.get(&self.opcode) {
            if (ins.addressmode as usize) == (Cpu::imp as usize) {
                self.a = t;
            } else {
                self.bus.write(self.addr_abs, t);
            }
        }

        0
    }

    fn pha(&mut self) -> usize {
        self.bus.write(0x0100 + self.sp as u16, self.a);
        self.sp = self.sp.wrapping_sub(1);

        0
    }

    fn jmp(&mut self) -> usize {
        self.pc = self.addr_abs;

        0
    }

    fn bvc(&mut self) -> usize {
        self.conditional_branch(Flag::V, false);

        0
    }

    fn cli(&mut self) -> usize {
        self.set_flag(Flag::I, false);

        0
    }

    fn rts(&mut self) -> usize {
        self.sp = self.sp.wrapping_add(1);
        let lo: u16 = self.bus.read(0x0100 + self.sp as u16) as u16;
        self.sp = self.sp.wrapping_add(1);
        let hi: u16 = self.bus.read(0x0100 + self.sp as u16) as u16;

        self.pc = (hi.wrapping_shl(8) | lo).wrapping_add(1);

        0
    }

    fn adc(&mut self) -> usize {
        self.fetch();

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

    fn ror(&mut self) -> usize {
        self.fetch();
        let t = ((self.get_flag(Flag::C) as u8) << 7) as u16 | self.fetched.wrapping_shr(1) as u16;

        self.set_flag(Flag::C, (self.fetched & 0x01) > 0);
        self.set_flag(Flag::Z, (t & 0x00FF) == 0x00);
        self.set_flag(Flag::N, (t & 0x0080) > 0);

        if let Some(ins) = self.dispatch.get(&self.opcode) {
            if (ins.addressmode as usize) == (Cpu::imp as usize) {
                self.a = (t & 0x00FF) as u8;
            } else {
                self.bus.write(self.addr_abs, (t & 0x00FF) as u8);
            }
        }

        0
    }

    fn pla(&mut self) -> usize {
        self.sp = self.sp.wrapping_add(1);
        self.a = self.bus.read(0x0100 + self.sp as u16);

        self.set_flag(Flag::Z, self.a == 0);
        self.set_flag(Flag::N, (self.a & 0b10000000) > 0);

        0
    }

    fn bvs(&mut self) -> usize {
        self.conditional_branch(Flag::V, true);

        0
    }

    fn sei(&mut self) -> usize {
        self.set_flag(Flag::I, true);

        0
    }

    fn sta(&mut self) -> usize {
        self.bus.write(self.addr_abs, self.a);

        0
    }

    fn sty(&mut self) -> usize {
        self.bus.write(self.addr_abs, self.y);

        0
    }

    fn stx(&mut self) -> usize {
        self.bus.write(self.addr_abs, self.x);

        0
    }

    fn dey(&mut self) -> usize {
        self.y = self.y.wrapping_sub(1);

        self.set_flag(Flag::Z, self.y == 0);
        self.set_flag(Flag::N, (self.y & 0b10000000) > 0);

        0
    }

    fn txa(&mut self) -> usize {
        self.a = self.x;

        self.set_flag(Flag::Z, self.a == 0);
        self.set_flag(Flag::N, (self.a & 0b10000000) > 0);

        0
    }

    fn bcc(&mut self) -> usize {
        self.conditional_branch(Flag::C, false);

        0
    }

    fn tya(&mut self) -> usize {
        self.a = self.y;

        self.set_flag(Flag::Z, self.a == 0);
        self.set_flag(Flag::N, (self.a & 0b10000000) > 0);

        0
    }

    fn txs(&mut self) -> usize {
        self.sp = self.x;

        0
    }

    fn ldy(&mut self) -> usize {
        self.fetch();
        self.y = self.fetched;

        self.set_flag(Flag::Z, self.y == 0);
        self.set_flag(Flag::N, (self.y & 0b10000000) > 0);

        1
    }

    fn lda(&mut self) -> usize {
        self.fetch();
        self.a = self.fetched;

        self.set_flag(Flag::Z, self.a == 0);
        self.set_flag(Flag::N, (self.a & 0b10000000) > 0);

        1
    }

    fn ldx(&mut self) -> usize {
        self.fetch();
        self.x = self.fetched;

        self.set_flag(Flag::Z, self.x == 0);
        self.set_flag(Flag::N, (self.x & 0b10000000) > 0);

        1
    }

    fn tay(&mut self) -> usize {
        self.y = self.a;

        self.set_flag(Flag::Z, self.y == 0);
        self.set_flag(Flag::N, (self.y & 0b10000000) > 0);

        0
    }

    fn tax(&mut self) -> usize {
        self.x = self.a;

        self.set_flag(Flag::Z, self.x == 0);
        self.set_flag(Flag::N, (self.x & 0b10000000) > 0);

        0
    }

    fn bcs(&mut self) -> usize {
        self.conditional_branch(Flag::C, true);

        0
    }

    fn clv(&mut self) -> usize {
        self.set_flag(Flag::V, false);

        0
    }

    fn tsx(&mut self) -> usize {
        self.x = self.sp;

        self.set_flag(Flag::Z, self.x == 0);
        self.set_flag(Flag::N, (self.x & 0b10000000) > 0);

        0
    }

    fn cpy(&mut self) -> usize {
        self.fetch();

        self.set_flag(Flag::C, self.y >= self.fetched);
        self.set_flag(Flag::Z, self.y == self.fetched);
        self.set_flag(
            Flag::N,
            (self.y.wrapping_sub(self.fetched) & 0b10000000) > 0,
        );

        0
    }

    fn cmp(&mut self) -> usize {
        self.fetch();

        self.set_flag(Flag::C, self.a >= self.fetched);
        self.set_flag(Flag::Z, self.a == self.fetched);
        self.set_flag(
            Flag::N,
            (self.a.wrapping_sub(self.fetched) & 0b10000000) > 0,
        );

        0
    }

    fn dec(&mut self) -> usize {
        self.fetch();

        let value = self.fetched.wrapping_sub(1);
        self.bus.write(self.addr_abs, value);

        self.set_flag(Flag::Z, value == 0);
        self.set_flag(Flag::N, (value & 0b10000000) > 0);

        0
    }

    fn iny(&mut self) -> usize {
        self.y = self.y.wrapping_add(1);

        self.set_flag(Flag::Z, self.y == 0);
        self.set_flag(Flag::N, (self.y & 0b10000000) > 0);

        0
    }

    fn dex(&mut self) -> usize {
        self.x = self.x.wrapping_sub(1);

        self.set_flag(Flag::Z, self.x == 0);
        self.set_flag(Flag::N, (self.x & 0b10000000) > 0);

        0
    }

    fn bne(&mut self) -> usize {
        self.conditional_branch(Flag::Z, false);

        0
    }

    fn cld(&mut self) -> usize {
        self.set_flag(Flag::D, false);

        0
    }

    fn nop(&mut self) -> usize {
        0
    }

    fn cpx(&mut self) -> usize {
        self.fetch();

        self.set_flag(Flag::C, self.x >= self.fetched);
        self.set_flag(Flag::Z, self.x == self.fetched);
        self.set_flag(
            Flag::N,
            (self.x.wrapping_sub(self.fetched) & 0b10000000) > 0,
        );

        0
    }

    fn sbc(&mut self) -> usize {
        self.fetch();
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

    fn inc(&mut self) -> usize {
        self.fetch();

        let value: u8 = self.fetched.wrapping_add(1);
        self.bus.write(self.addr_abs, value);

        self.set_flag(Flag::N, (value & 0b10000000) > 0);
        self.set_flag(Flag::Z, value == 0);

        0
    }

    fn inx(&mut self) -> usize {
        self.x = self.x.wrapping_add(1);

        self.set_flag(Flag::N, (self.x & 0b10000000) > 0);
        self.set_flag(Flag::Z, self.x == 0);

        1
    }

    fn beq(&mut self) -> usize {
        self.conditional_branch(Flag::Z, true);

        0
    }

    fn sed(&mut self) -> usize {
        self.set_flag(Flag::D, true);

        0
    }

    // Illegal opcodes

    fn lax(&mut self) -> usize {
        self.lda();
        self.tax();

        1
    }

    fn sax(&mut self) -> usize {
        self.bus.write(self.addr_abs, self.a & self.x);

        0
    }

    fn dcp(&mut self) -> usize {
        self.dec();
        self.cmp();

        0
    }

    fn isc(&mut self) -> usize {
        self.inc();
        self.sbc();

        1
    }

    fn slo(&mut self) -> usize {
        self.asl();
        self.ora();

        0
    }

    fn rla(&mut self) -> usize {
        self.rol();
        self.and();

        1
    }

    fn sre(&mut self) -> usize {
        self.lsr();
        self.eor();

        0
    }

    fn rra(&mut self) -> usize {
        self.ror();
        self.adc();

        1
    }

    // Halts the cpu which doesn't increase the program counter.
    // Remove 1 from the pc to mimic this behavior.
    fn kil(&mut self) -> usize {
        self.pc = self.pc.wrapping_sub(1);

        0
    }

    fn anc(&mut self) -> usize {
        self.and();
        self.set_flag(Flag::C, (self.a & 0b10000000) > 0);

        1
    }

    fn alr(&mut self) -> usize {
        self.and();

        self.set_flag(Flag::C, (self.a & 0x0001) > 0);
        self.a = self.a.wrapping_shr(1);

        self.set_flag(Flag::Z, self.a == 0);
        self.set_flag(Flag::N, (self.a & 0b10000000) > 1);

        1
    }

    fn arr(&mut self) -> usize {
        self.fetch();
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
    fn xaa(&mut self) -> usize {
        self.fetch();
        self.a = (self.a | 0xEE) & self.x & self.fetched;

        self.set_flag(Flag::N, (self.a & 0b10000000) > 0);
        self.set_flag(Flag::Z, self.a == 0);

        0
    }

    fn axs(&mut self) -> usize {
        self.fetch();

        let res = (self.a & self.x).wrapping_sub(self.fetched);

        self.set_flag(Flag::C, (self.a & self.x) >= res);
        self.set_flag(Flag::Z, res == 0);
        self.set_flag(Flag::N, (res & 0b10000000) > 1);

        self.x = res;

        0
    }

    fn tas(&mut self) -> usize {
        self.fetch();
        self.sp = self.a & self.x;
        self.bus.write(
            self.addr_abs,
            self.a & self.x & ((self.addr_abs.wrapping_shr(8) as u8).wrapping_add(1)),
        );

        0
    }

    fn sha(&mut self) -> usize {
        self.fetch();
        self.bus.write(
            self.addr_abs,
            self.a & self.x & (self.addr_abs.wrapping_shr(8) as u8).wrapping_add(1),
        );

        0
    }

    fn shx(&mut self) -> usize {
        self.fetch();
        self.bus.write(
            self.addr_abs,
            self.x & (self.addr_abs.wrapping_shr(8) as u8).wrapping_add(1),
        );

        0
    }

    fn shy(&mut self) -> usize {
        self.fetch();
        self.bus.write(
            self.addr_abs,
            self.y & (self.addr_abs.wrapping_shr(8) as u8).wrapping_add(1),
        );

        0
    }

    fn lxa(&mut self) -> usize {
        self.fetch();

        let res = (self.a | 0xEE) & self.fetched;
        self.a = res;
        self.x = res;

        self.set_flag(Flag::Z, res == 0);
        self.set_flag(Flag::N, (res & 0b10000000) > 0);

        0
    }

    fn las(&mut self) -> usize {
        self.fetch();

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
                cpu.bus.write(*address, *value);
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
