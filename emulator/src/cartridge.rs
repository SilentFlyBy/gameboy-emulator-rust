use std::{
    fs::File,
    io::{self, Read},
};

use crate::{bus::FetchWrite, ram::Ram};

const ERAM_ADDRESS_OFFSET: u16 = 0xA000;

pub struct Cartridge {
    rom: Ram,
    eram: Ram,
}

impl Cartridge {
    pub fn new(path: &str) -> Self {
        let mut rom = Ram::new(0x8000, 0);
        let mut file = File::open(path).unwrap();
        file.read_exact(&mut rom.buffer).unwrap();

        Cartridge {
            rom,
            eram: Ram::new(0x2000, ERAM_ADDRESS_OFFSET),
        }
    }

    // fn parse_header() {}
}

impl FetchWrite for Cartridge {
    fn fetch8(&mut self, address: u16) -> io::Result<u8> {
        if address < ERAM_ADDRESS_OFFSET {
            self.rom.fetch8(address)
        } else {
            self.eram.fetch8(address)
        }
    }

    fn fetch16(&mut self, address: u16) -> io::Result<u16> {
        if address < ERAM_ADDRESS_OFFSET {
            self.rom.fetch16(address)
        } else {
            self.eram.fetch16(address)
        }
    }

    fn write8(&mut self, address: u16, value: u8) -> std::io::Result<()> {
        if address < ERAM_ADDRESS_OFFSET {
            return Ok(());
            //panic!("Cannot write to cartridge ROM!")
        }

        self.eram.write8(address, value)?;

        Ok(())
    }

    fn write16(&mut self, address: u16, value: u16) -> std::io::Result<()> {
        if address < ERAM_ADDRESS_OFFSET {
            panic!("Cannot write to cartridge ROM!")
        }

        self.eram.write16(address, value)?;

        Ok(())
    }
}
