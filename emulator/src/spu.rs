use std::io;

use crate::bus::FetchWrite;

const NR50_ADDRESS: u16 = 0xFF24;
const NR51_ADDRESS: u16 = 0xFF25;
const NR52_ADDRESS: u16 = 0xFF26;

pub struct Spu {
    nr50: u8,
    nr51: u8,
    nr52: u8,
}

impl Spu {
    pub fn new() -> Self {
        Spu {
            nr50: 0,
            nr51: 0,
            nr52: 0,
        }
    }

    fn get_address_target(&mut self, address: u16) -> io::Result<&mut dyn FetchWrite> {
        match address {
            NR50_ADDRESS => Ok(&mut self.nr50),
            NR51_ADDRESS => Ok(&mut self.nr51),
            NR52_ADDRESS => Ok(&mut self.nr52),
            _ => panic!("Address violation: {:#X}", address),
        }
    }
}

impl FetchWrite for Spu {
    fn fetch8(&mut self, address: u16) -> Result<u8, std::io::Error> {
        let target = self.get_address_target(address)?;

        target.fetch8(address)
    }

    fn fetch16(&mut self, address: u16) -> Result<u16, std::io::Error> {
        let target = self.get_address_target(address)?;

        target.fetch16(address)
    }

    fn write8(&mut self, address: u16, value: u8) -> std::io::Result<()> {
        let target = self.get_address_target(address)?;

        target.write8(address, value)
    }

    fn write16(&mut self, address: u16, value: u16) -> std::io::Result<()> {
        let target = self.get_address_target(address)?;

        target.write16(address, value)
    }
}
