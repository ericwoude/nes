pub struct Bus {
    ram: [u8; 64 * 1024],
}

impl Bus {
    pub fn new() -> Bus {
        Bus {
            ram: [0; 64 * 1024],
        }
    }

    pub fn write<T: Into<u16>>(&mut self, addr: T, data: u8) {
        let address = addr.into();
        match address {
            0x0000..=0xFFFF => self.ram[address as usize] = data,
        }
    }

    pub fn read<T: Into<u16>>(&self, addr: T) -> u8 {
        let address = addr.into();
        match address {
            0x0000..=0xFFFF => self.ram[address as usize],
        }
    }
}
