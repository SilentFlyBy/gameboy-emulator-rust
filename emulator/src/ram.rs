use crate::bus::FetchWrite;

pub struct Ram {
    pub buffer: Vec<u8>,
    address_offset: u16,
}

impl Ram {
    pub fn new(size: usize, address_offset: u16) -> Self {
        let buffer = vec![0u8; size];

        Ram {
            buffer,
            address_offset,
        }
    }
}

impl FetchWrite for Ram {
    fn fetch8(&mut self, address: u16) -> Result<u8, std::io::Error> {
        let address = address - self.address_offset;
        Ok((*self.buffer)[(address) as usize])
    }

    fn fetch16(&mut self, address: u16) -> Result<u16, std::io::Error> {
        let address = address - self.address_offset;

        let val1 = (*self.buffer)[(address + 1) as usize];
        let val2 = (*self.buffer)[(address) as usize];

        let value = ((val1 as u16) << 8) | (val2 as u16);

        Ok(value)
    }

    fn write8(&mut self, address: u16, value: u8) -> std::io::Result<()> {
        let address = address - self.address_offset;
        (*self.buffer)[(address) as usize] = value;

        Ok(())
    }

    fn write16(&mut self, address: u16, value: u16) -> std::io::Result<()> {
        let address = address - self.address_offset;
        let val1 = (value & 0x00FF) as u8;
        let val2 = ((value & 0xFF00) >> 8) as u8;

        (*self.buffer)[address as usize] = val1;
        (*self.buffer)[(address + 1) as usize] = val2;

        Ok(())
    }
}
