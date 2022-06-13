use std::io;

use crate::{
    boot::{BootRom, DMG_BOOT},
    buttons::Buttons,
    cartridge::Cartridge,
    constants::{
        INTERRUPT_ENABLE_ADDRESS, INTERRUPT_REQUEST_ADDRESS, VRAM_END_ADDRESS, VRAM_START_ADDRESS,
    },
    gpu::Gpu,
    interrupts::Interrupts,
    ram::Ram,
    register::Register8,
    spu::Spu,
    timer::Timer,
};

pub trait FetchWrite {
    fn fetch8(&mut self, address: u16) -> Result<u8, io::Error>;
    fn fetch16(&mut self, address: u16) -> Result<u16, io::Error>;
    fn write8(&mut self, address: u16, value: u8) -> std::io::Result<()>;
    fn write16(&mut self, address: u16, value: u16) -> std::io::Result<()>;
}

pub struct Bus<'a> {
    boot_rom: BootRom,
    cartridge: Cartridge,
    wram: Ram,
    hram: Ram,
    pub interrupts: Interrupts,
    gpu: Gpu<'a>,
    serial_transfer: Register8,
    serial_control: Register8,
    spu: Spu,
    null: u8,
    timer: Timer,
    buttons: Buttons,
}

const BOOT_ROM_START_ADDRESS: u16 = 0x0;
const BOOT_ROM_END_ADDRESS: u16 = 0xFF;

const ROM_START_ADDRESS: u16 = 0x0;
const ROM_END_ADDRESS: u16 = 0x7FFF;

const ERAM_START_ADDRESS: u16 = 0xA000;
const ERAM_END_ADDRESS: u16 = 0xBFFF;

const WRAM_START_ADDRESS: u16 = 0xC000;
const WRAM_END_ADDRESS: u16 = 0xDFFF;

const ECHO_WRAM_START_ADDRESS: u16 = 0xE000;
const ECHO_WRAM_END_ADDRESS: u16 = 0xFDFF;

const TIMER_START_ADDRESS: u16 = 0xFF04;
const TIMER_END_ADDRESS: u16 = 0xFF07;

const GPU_REGISTER_START_ADDRESS: u16 = 0xFF40;
const GPU_REGISTER_END_ADDRESS: u16 = 0xFF4B;

const SPU_REGISTER_START_ADDRESS: u16 = 0xFF24;
const SPU_REGISTER_END_ADDRESS: u16 = 0xFF26;

const SERIAL_TRANSFER_REGISTER_ADDRESS: u16 = 0xFF01;
const SERIAL_CONTROL_REGISTER_ADDRESS: u16 = 0xFF02;

const HRAM_START_ADDRESS: u16 = 0xFF80;
const HRAM_END_ADDRESS: u16 = 0xFFFE;

const BUTTONS_REGISTER_ADDRESS: u16 = 0xFF00;

impl<'a> Bus<'a> {
    pub fn new(cartridge: Cartridge, gpu: Gpu<'a>) -> Self {
        let wram = Ram::new(0x2000, WRAM_START_ADDRESS);
        let hram = Ram::new(0x7F, HRAM_START_ADDRESS);
        let serial_transfer = 0u8;
        let serial_control = 0u8;
        let spu = Spu::new();
        Bus {
            boot_rom: BootRom {},
            cartridge,
            wram,
            hram,
            interrupts: Interrupts::new(),
            gpu,
            serial_control,
            serial_transfer,
            spu,
            null: 0,
            timer: Timer::new(),
            buttons: Buttons::new(),
        }
    }

    pub fn next(&mut self, clock_cycles: u8) {
        self.gpu.next(clock_cycles, &mut self.interrupts);
        self.timer.next(clock_cycles, &mut self.interrupts);
    }

    fn get_address_target(&mut self, address: u16) -> io::Result<&mut dyn FetchWrite> {
        match address {
            //BOOT_ROM_START_ADDRESS..=BOOT_ROM_END_ADDRESS => Ok(&mut self.boot_rom),
            ROM_START_ADDRESS..=ROM_END_ADDRESS => Ok(&mut self.cartridge),
            ERAM_START_ADDRESS..=ERAM_END_ADDRESS => Ok(&mut self.cartridge),
            WRAM_START_ADDRESS..=WRAM_END_ADDRESS => Ok(&mut self.wram),
            ECHO_WRAM_START_ADDRESS..=ECHO_WRAM_END_ADDRESS => Ok(&mut self.wram),
            VRAM_START_ADDRESS..=VRAM_END_ADDRESS => Ok(&mut self.gpu),
            HRAM_START_ADDRESS..=HRAM_END_ADDRESS => Ok(&mut self.hram),
            INTERRUPT_REQUEST_ADDRESS => Ok(&mut self.interrupts),
            INTERRUPT_ENABLE_ADDRESS => Ok(&mut self.interrupts),
            SERIAL_TRANSFER_REGISTER_ADDRESS => Ok(&mut self.serial_transfer),
            SERIAL_CONTROL_REGISTER_ADDRESS => Ok(&mut self.serial_control),
            BUTTONS_REGISTER_ADDRESS => Ok(&mut self.buttons),
            GPU_REGISTER_START_ADDRESS..=GPU_REGISTER_END_ADDRESS => Ok(&mut self.gpu),
            SPU_REGISTER_START_ADDRESS..=SPU_REGISTER_END_ADDRESS => Ok(&mut self.spu),
            TIMER_START_ADDRESS..=TIMER_END_ADDRESS => Ok(&mut self.timer),
            _ => Ok(&mut self.null),
        }
    }
}

impl<'a> FetchWrite for Bus<'a> {
    fn fetch8(&mut self, address: u16) -> io::Result<u8> {
        let target = self.get_address_target(address)?;

        target.fetch8(address)
    }

    fn fetch16(&mut self, address: u16) -> io::Result<u16> {
        let target = self.get_address_target(address)?;

        target.fetch16(address)
    }

    fn write8(&mut self, address: u16, value: u8) -> std::io::Result<()> {
        let target = self.get_address_target(address)?;
        target.write8(address, value)?;

        Ok(())
    }

    fn write16(&mut self, address: u16, value: u16) -> std::io::Result<()> {
        todo!()
    }
}
