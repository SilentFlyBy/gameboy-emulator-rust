use std::io;

use crate::{
    bus::FetchWrite,
    constants::{SYSCLK_FREQ, SYSCLK_FREQ_1024, SYSCLK_FREQ_16, SYSCLK_FREQ_256, SYSCLK_FREQ_64},
    interrupts::Interrupts,
};

const DIV_REGISTER_ADDRESS: u16 = 0xFF04;
const TIMA_REGISTER_ADDRESS: u16 = 0xFF05;
const TMA_REGISTER_ADDRESS: u16 = 0xFF06;
const TAC_REGISTER_ADDRESS: u16 = 0xFF07;

pub struct Timer {
    div: u8,
    tima: u8,
    tma: u8,
    tac: u8,
    div_increment: f64,
    tima_increment: f64,
}

impl Timer {
    pub fn new() -> Self {
        Timer {
            div: 0,
            tima: 0,
            tma: 0,
            tac: 0,
            div_increment: 0.0,
            tima_increment: 0.0,
        }
    }

    pub fn next(&mut self, clock_cycles: u8, interrupts: &mut Interrupts) {
        self.div_increment += clock_cycles as f64 * (SYSCLK_FREQ_256 as f64 / SYSCLK_FREQ as f64);

        if self.div_increment > 1.0 {
            self.div = self.div.wrapping_add(self.div_increment as u8);
            self.div_increment -= self.div_increment.floor();
        }

        let enable_timer = (self.tac & 0b100) != 0;
        if !enable_timer {
            return;
        }

        let selected_frequency = match self.tac & 0b11 {
            0b00 => SYSCLK_FREQ_1024,
            0b01 => SYSCLK_FREQ_16,
            0b10 => SYSCLK_FREQ_64,
            0b11 => SYSCLK_FREQ_256,
            _ => 0,
        };

        self.tima_increment +=
            clock_cycles as f64 * (selected_frequency as f64 / SYSCLK_FREQ as f64);

        if self.tima_increment > 1.0 {
            let (value, did_overflow) = self.tima.overflowing_add(self.tima_increment as u8);
            self.tima_increment -= self.tima_increment.floor();
            if did_overflow {
                interrupts.set_timer_request(true);
                self.tima = self.tma;
            } else {
                self.tima = value;
            }
        }
    }

    fn get_address_target(&mut self, address: u16) -> io::Result<&mut dyn FetchWrite> {
        match address {
            DIV_REGISTER_ADDRESS => Ok(&mut self.div),
            TIMA_REGISTER_ADDRESS => Ok(&mut self.tima),
            TMA_REGISTER_ADDRESS => Ok(&mut self.tma),
            TAC_REGISTER_ADDRESS => Ok(&mut self.tac),
            _ => panic!("Accessing unsupported timer address: {:#X}", address),
        }
    }
}

impl FetchWrite for Timer {
    fn fetch8(&mut self, address: u16) -> Result<u8, std::io::Error> {
        let target = self.get_address_target(address)?;

        target.fetch8(address)
    }

    fn fetch16(&mut self, _: u16) -> Result<u16, std::io::Error> {
        panic!("16 Bit operations unsupported")
    }

    fn write8(&mut self, address: u16, value: u8) -> std::io::Result<()> {
        if address == DIV_REGISTER_ADDRESS {
            self.div = 0;
            self.div_increment = 0.0;
            return Ok(());
        }

        if address == TAC_REGISTER_ADDRESS {
            self.div = 0;
            self.div_increment = 0.0;
        }

        let target = self.get_address_target(address)?;

        target.write8(address, value)
    }

    fn write16(&mut self, _: u16, _: u16) -> std::io::Result<()> {
        panic!("16 Bit operations unsupported")
    }
}
