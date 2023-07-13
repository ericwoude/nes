#![allow(dead_code)]

mod bus;
mod cpu;

fn main() {
    let mut c = cpu::Cpu::new();
    c.clock();
}
