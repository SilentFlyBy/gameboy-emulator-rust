#![allow(dead_code)]
#![allow(unused_variables)]

use crate::{bus::FetchWrite, register::Register8};

const PRIORITY_BITMASK: u8 = 1;
const OBJ_ENABLE_BITMASK: u8 = 1 << 1;
const OBJ_SIZE_BITMASK: u8 = 1 << 2;
const BG_AREA_BITMASK: u8 = 1 << 3;
const BG_CHARACTERS_BITMASK: u8 = 1 << 4;
const WINDOW_ENABLE_BITMASK: u8 = 1 << 5;
const WINDOW_AREA_BITMASK: u8 = 1 << 6;
const LCD_ENABLE_BITMASK: u8 = 1 << 7;

pub struct LCDC {
    value: Register8,
}

impl LCDC {
    pub fn new() -> Self {
        LCDC { value: 0 }
    }

    pub fn get_priority(&self) -> bool {
        self.value & PRIORITY_BITMASK != 0
    }

    pub fn get_obj_enable(&self) -> bool {
        self.value & OBJ_ENABLE_BITMASK != 0
    }

    pub fn get_obj_size(&self) -> bool {
        self.value & OBJ_SIZE_BITMASK != 0
    }

    pub fn get_bg_area(&self) -> bool {
        self.value & BG_AREA_BITMASK != 0
    }

    pub fn get_bg_characters(&self) -> bool {
        self.value & BG_CHARACTERS_BITMASK != 0
    }

    pub fn get_window_enable(&self) -> bool {
        self.value & WINDOW_ENABLE_BITMASK != 0
    }

    pub fn get_window_area(&self) -> bool {
        self.value & WINDOW_AREA_BITMASK != 0
    }

    pub fn get_lcd_enable(&self) -> bool {
        self.value & LCD_ENABLE_BITMASK != 0
    }
}

impl FetchWrite for LCDC {
    fn fetch8(&mut self, address: u16) -> Result<u8, std::io::Error> {
        self.value.fetch8(address)
    }

    fn fetch16(&mut self, address: u16) -> Result<u16, std::io::Error> {
        panic!("16 bit operations not supported with 8 bit register")
    }

    fn write8(&mut self, address: u16, value: u8) -> std::io::Result<()> {
        self.value.write8(address, value)
    }

    fn write16(&mut self, address: u16, value: u16) -> std::io::Result<()> {
        panic!("16 bit operations not supported with 8 bit register")
    }
}

const MODE_BITMASK: u8 = 0b11;
const LYC_MATCH_BITMASK: u8 = 1 << 2;
const MODE_HBLANK_INTERRUPT_BITMASK: u8 = 1 << 3;
const MODE_VBLANK_INTERRUPT_BITMASK: u8 = 1 << 4;
const MODE_OAM_INTERRUPT_BITMASK: u8 = 1 << 5;
const LYC_MATCH_INTERRUPT_BITMASK: u8 = 1 << 6;

pub enum Mode {
    HBlank = 0,
    VBlank = 1,
    ScanOam = 2,
    ScanVram = 3,
}

pub struct STAT {
    value: Register8,
}

impl STAT {
    pub fn new() -> Self {
        STAT { value: 0 }
    }

    pub fn get_mode(&self) -> Mode {
        match self.value & MODE_BITMASK {
            0 => Mode::HBlank,
            1 => Mode::VBlank,
            2 => Mode::ScanOam,
            3 => Mode::ScanVram,
            _ => panic!("Enum out of bounds"),
        }
    }

    pub fn get_lyc_coincidence(&self) -> bool {
        self.value & LYC_MATCH_BITMASK != 0
    }

    pub fn get_mode_hblank_interrupt(&self) -> bool {
        self.value & MODE_HBLANK_INTERRUPT_BITMASK != 0
    }

    pub fn get_mode_vblank_interrupt(&self) -> bool {
        self.value & MODE_VBLANK_INTERRUPT_BITMASK != 0
    }

    pub fn get_mode_oam_interrupt(&self) -> bool {
        self.value & MODE_OAM_INTERRUPT_BITMASK != 0
    }

    pub fn get_lyc_coincidence_interrupt(&self) -> bool {
        self.value & LYC_MATCH_INTERRUPT_BITMASK != 0
    }

    pub fn set_mode(&mut self, mode: Mode) {
        let mode_num: u8 = mode as u8;

        if mode_num & 1 == 0 {
            self.value &= !1;
        } else {
            self.value |= 1;
        }

        if mode_num & 1 << 1 == 0 {
            self.value &= !(1 << 1);
        } else {
            self.value |= 1 << 1;
        }
    }

    pub fn set_lyc_match(&mut self, value: bool) {
        if value {
            self.value |= LYC_MATCH_BITMASK
        } else {
            self.value &= !LYC_MATCH_BITMASK
        }
    }
}

impl FetchWrite for STAT {
    fn fetch8(&mut self, address: u16) -> Result<u8, std::io::Error> {
        self.value.fetch8(address)
    }

    fn fetch16(&mut self, address: u16) -> Result<u16, std::io::Error> {
        panic!("16 bit operations not supported with 8 bit register")
    }

    fn write8(&mut self, address: u16, value: u8) -> std::io::Result<()> {
        self.value.write8(address, value)
    }

    fn write16(&mut self, address: u16, value: u16) -> std::io::Result<()> {
        panic!("16 bit operations not supported with 8 bit register")
    }
}
