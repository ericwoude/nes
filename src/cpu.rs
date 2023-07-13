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

struct Instruction {
    name: &'static str,
    operation: fn(&mut Cpu) -> u8,
    addressmode: fn(&Cpu) -> u8,
    cycles: usize,
}

pub struct Cpu {
    a: u8,
    x: u8,
    y: u8,
    sp: u16,
    pc: u16,
    status: u8,
    bus: Bus,

    addr_abs: u16,
    addr_rel: u8,
    opcode: u8,
    cycles: usize,

    opcode_table: HashMap<u8, Instruction>,
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            a: 0x00,
            x: 0x00,
            y: 0x00,
            sp: 0x0000,
            pc: 0x0000,
            status: 0x00,
            bus: Bus::new(),

            addr_abs: 0,
            addr_rel: 0,
            opcode: 0,
            cycles: 0,

            opcode_table: HashMap::from([
                (
                    0x00,
                    Instruction {
                        name: "BRK",
                        operation: |c| c.brk(),
                        addressmode: |c| c.imm(),
                        cycles: 7,
                    },
                ),
                (
                    0x01,
                    Instruction {
                        name: "ORA",
                        operation: |c| c.ora(),
                        addressmode: |c| c.ind(),
                        cycles: 6,
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

    fn fetch(&self) -> u8 {
        todo!()
    }

    pub fn clock(&mut self) {
        if self.cycles == 0 {
            let opcode = self.bus.read(self.pc);
            self.pc += 1;

            match self.opcode_table.get(&opcode) {
                Some(instruction) => {
                    self.cycles = instruction.cycles;

                    let addr_cycles = (instruction.addressmode)(self);
                    let op_cycles = (instruction.operation)(self);

                    self.cycles += (addr_cycles & op_cycles) as usize;
                }
                None => println!("[Error] No opcode known: {:X}", opcode),
            }
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

    fn imp(&self) -> u8 {
        todo!()
    }

    fn imm(&self) -> u8 {
        todo!()
    }

    fn zp0(&self) -> u8 {
        todo!()
    }

    fn zpx(&self) -> u8 {
        todo!()
    }

    fn zpy(&self) -> u8 {
        todo!()
    }

    fn rel(&self) -> u8 {
        todo!()
    }

    fn abs(&self) -> u8 {
        todo!()
    }

    fn abx(&self) -> u8 {
        todo!()
    }

    fn aby(&self) -> u8 {
        todo!()
    }

    fn ind(&self) -> u8 {
        todo!()
    }

    fn izx(&self) -> u8 {
        todo!()
    }

    fn izy(&self) -> u8 {
        todo!()
    }

    // OPCODE FUNCTIONS

    fn xxx(&self) -> u8 {
        todo!()
    }

    fn brk(&self) -> u8 {
        todo!()
    }

    fn ora(&self) -> u8 {
        todo!()
    }
}
