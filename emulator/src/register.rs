use std::io;

use crate::bus::FetchWrite;

pub type Register8 = u8;

impl FetchWrite for Register8 {
    fn fetch8(&mut self, _: u16) -> Result<u8, io::Error> {
        Ok(*self)
    }

    fn fetch16(&mut self, _: u16) -> Result<u16, io::Error> {
        panic!("16 bit operations not supported with 8 bit register")
    }

    fn write8(&mut self, _: u16, value: u8) -> std::io::Result<()> {
        *self = value;

        Ok(())
    }

    fn write16(&mut self, _: u16, _: u16) -> std::io::Result<()> {
        panic!("16 bit operations not supported with 8 bit register")
    }
}
