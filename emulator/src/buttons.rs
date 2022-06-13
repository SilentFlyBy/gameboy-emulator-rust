use crate::bus::FetchWrite;

pub struct Buttons {
    value: u8,
}

impl Buttons {
    pub fn new() -> Self {
        Buttons { value: 0b11001111 }
    }
}

impl FetchWrite for Buttons {
    fn fetch8(&mut self, address: u16) -> Result<u8, std::io::Error> {
        Ok(self.value)
    }

    fn fetch16(&mut self, address: u16) -> Result<u16, std::io::Error> {
        panic!("16 bit operations not supported with 8 bit register")
    }

    fn write8(&mut self, address: u16, value: u8) -> std::io::Result<()> {
        Ok(())
    }

    fn write16(&mut self, address: u16, value: u16) -> std::io::Result<()> {
        panic!("16 bit operations not supported with 8 bit register")
    }
}
