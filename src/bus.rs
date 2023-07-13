pub struct Bus {
    ram: [u8; 64 * 1024],
}

impl Bus {
    pub fn new() -> Bus {
        Bus {
            ram: [0; 64 * 1024],
        }
    }

    pub fn write(&mut self, addr: u16, data: u16) {
        match addr {
            0x0000..=0xFFFF => self.ram[addr as usize] = data as u8,
        }
    }

    pub fn read(&self, addr: u16) -> u8 {
        match addr {
            0x0000..=0xFFFF => self.ram[addr as usize],
        }
    }
}
