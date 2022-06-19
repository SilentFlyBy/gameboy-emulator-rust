use std::{fs::File, io::Read};

use crate::{bus::FetchWrite, ram::Ram};

pub struct BootRom {
    rom: Ram,
}

impl BootRom {
    pub fn new(path: String) -> Self {
        let mut rom = Ram::new(0x100, 0);
        let mut file = File::open(path).unwrap();
        file.read_exact(&mut rom.buffer).unwrap();

        BootRom { rom }
    }
}

impl FetchWrite for BootRom {
    fn fetch8(&mut self, address: u16) -> Result<u8, std::io::Error> {
        self.rom.fetch8(address)
    }

    fn fetch16(&mut self, address: u16) -> Result<u16, std::io::Error> {
        self.rom.fetch16(address)
    }

    fn write8(&mut self, _: u16, _: u8) -> std::io::Result<()> {
        panic!("Can't write to bootrom!");
    }

    fn write16(&mut self, _: u16, _: u16) -> std::io::Result<()> {
        panic!("Can't write to bootrom!");
    }
}
