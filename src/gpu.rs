use core::panic;
use std::io;

use sdl2::{rect::Point, render::Canvas, video::Window};

use crate::{
    bus::FetchWrite,
    frontend::display::{Display, DMG_COLOR},
    interrupts::Interrupts,
    register::Register8,
};

#[derive(Debug)]
enum Mode {
    HBLANK = 0,
    VBLANK = 1,
    SCAN_OAM = 2,
    SCAN_VRAM = 3,
}

const OAM_CYCLES: u32 = 80;
const VRAM_CYCLES: u32 = 172;
const HBLANK_CYCLES: u32 = 204;
const SCANLINE_CYCLES: u32 = 456;

const VBLANK_START_LINE: u8 = 143;
const VBLANK_END_LINE: u8 = 153;

const LCDC_ADDRESS: u16 = 0xFF40;
const STAT_ADDRESS: u16 = 0xFF41;
const SCY_ADDRESS: u16 = 0xFF42;
const SCX_ADDRESS: u16 = 0xFF43;
const LY_ADDRESS: u16 = 0xFF44;
const LYC_ADDRESS: u16 = 0xFF45;
const DMA_ADDRESS: u16 = 0xFF46;
const BGP_ADDRESS: u16 = 0xFF47;
const OBP0_ADDRESS: u16 = 0xFF48;
const OBP1_ADDRESS: u16 = 0xFF49;
const WY_ADDRESS: u16 = 0xFF4A;
const WX_ADDRESS: u16 = 0xFF4B;

pub struct Gpu<'a> {
    display: &'a mut Display,
    mode: Mode,
    modeclock: u32,
    lcdc: Register8,
    stat: Register8,
    scy: Register8,
    scx: Register8,
    ly: Register8,
    lyc: Register8,
    dma: Register8,
    bgp: Register8,
    obp0: Register8,
    obp1: Register8,
    wy: Register8,
    wx: Register8,
}

impl<'a> Gpu<'a> {
    pub fn new(display: &'a mut Display) -> Self {
        Gpu {
            display,
            mode: Mode::HBLANK,
            modeclock: 0,
            lcdc: 0,
            stat: 0,
            scy: 0,
            scx: 0,
            ly: 0,
            lyc: 0,
            dma: 0,
            bgp: 0,
            obp0: 0,
            obp1: 0,
            wy: 0,
            wx: 0,
        }
    }
    fn get_address_target(&mut self, address: u16) -> io::Result<&mut dyn FetchWrite> {
        match address {
            LCDC_ADDRESS => Ok(&mut self.lcdc),
            STAT_ADDRESS => Ok(&mut self.stat),
            SCY_ADDRESS => Ok(&mut self.scy),
            SCX_ADDRESS => Ok(&mut self.scx),
            LY_ADDRESS => Ok(&mut self.ly),
            LYC_ADDRESS => Ok(&mut self.lyc),
            DMA_ADDRESS => Ok(&mut self.dma),
            BGP_ADDRESS => Ok(&mut self.bgp),
            OBP0_ADDRESS => Ok(&mut self.obp0),
            OBP1_ADDRESS => Ok(&mut self.obp1),
            WY_ADDRESS => Ok(&mut self.wy),
            WX_ADDRESS => Ok(&mut self.wx),
            _ => panic!("Address violation: {:#X}", address),
        }
    }

    pub fn next(&mut self, cycles: u8, interrupts: &mut Interrupts) {
        self.modeclock += cycles as u32;

        match self.mode {
            Mode::SCAN_OAM => {
                if self.modeclock >= OAM_CYCLES {
                    self.modeclock = self.modeclock % OAM_CYCLES;
                    self.stat |= 0b11;
                    self.mode = Mode::SCAN_VRAM;
                }
            }
            Mode::SCAN_VRAM => {
                if self.modeclock >= VRAM_CYCLES {
                    self.modeclock = self.modeclock % VRAM_CYCLES;
                    self.mode = Mode::HBLANK;

                    let hblank_interrupt = (self.stat & (1 << 3)) != 0;
                    if hblank_interrupt {
                        interrupts.set_lcd_stat_request(true);
                    }

                    let lyc_interrupt = (self.stat & (1 << 6)) != 0;
                    let lyc = self.ly == self.lyc;
                    if lyc_interrupt && lyc {
                        interrupts.set_lcd_stat_request(true);
                    }

                    if lyc {
                        self.stat |= 1 << 2;
                    } else {
                        self.stat &= !(1 << 2);
                    }

                    self.stat &= !0b11;
                }
            }
            Mode::HBLANK => {
                if self.modeclock >= HBLANK_CYCLES {
                    self.modeclock = self.modeclock % HBLANK_CYCLES;

                    self.render_line();
                    self.ly = self.ly.wrapping_add(1);

                    if self.ly == VBLANK_START_LINE {
                        self.mode = Mode::VBLANK;
                        self.present_image();

                        self.stat |= 0b1;
                        self.stat &= !0b10;
                        interrupts.set_v_blank_request(true);
                    } else {
                        self.mode = Mode::SCAN_OAM;
                        self.stat &= !0b1;
                        self.stat |= 0b10;
                    }
                }
            }
            Mode::VBLANK => {
                if self.modeclock >= SCANLINE_CYCLES {
                    self.modeclock = self.modeclock % SCANLINE_CYCLES;
                    self.ly = self.ly.wrapping_add(1);

                    if self.ly > VBLANK_END_LINE {
                        self.mode = Mode::SCAN_OAM;
                        self.ly = 0;
                        self.stat &= !0b1;
                        self.stat |= 0b10;
                    }
                }
            }
        }
    }

    fn present_image(&mut self) {
        self.display.present();
    }

    fn render_line(&mut self) {
        for x in 0u8..160 {
            self.render_pixel(x, self.ly);
        }
    }

    fn render_pixel(&mut self, x: u8, y: u8) {
        self.display.render_pixel(x, y, DMG_COLOR::DarkGrey);
    }

    fn get_window_color(&self, x: u8, y: u8) -> DMG_COLOR {
        let wx = self.wx.wrapping_sub(7);
        let px = x.wrapping_sub(wx);
        let py = y.wrapping_sub(self.wy);
    }

    pub fn lcdc_bg_display(&self) -> bool {
        (self.lcdc & BG_DISPLAY_BITMASK) > 0
    }

    pub fn lcdc_obj_display(&self) -> bool {
        (self.lcdc & OBJ_DISPLAY_BITMASK) > 0
    }

    pub fn lcdc_obj_block(&self) -> bool {
        (self.lcdc & OBJ_BLOCK_BITMASK) > 0
    }

    pub fn lcdc_bg_area(&self) -> bool {
        (self.lcdc & BG_AREA_BITMASK) > 0
    }

    pub fn lcdc_bg_characters(&self) -> bool {
        (self.lcdc & BG_CHARACTERS_BITMASK) > 0
    }

    pub fn lcdc_window(&self) -> bool {
        (self.lcdc & WINDOW_BITMASK) > 0
    }

    pub fn lcdc_win_area(&self) -> bool {
        (self.lcdc & WIN_AREA_BITMASK) > 0
    }

    pub fn lcdc_controller(&self) -> bool {
        (self.lcdc & CONTROLLER_BITMASK) > 0
    }
}

impl<'a> FetchWrite for Gpu<'a> {
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

const BG_DISPLAY_BITMASK: u8 = 1;
const OBJ_DISPLAY_BITMASK: u8 = 1 << 1;
const OBJ_BLOCK_BITMASK: u8 = 1 << 2;
const BG_AREA_BITMASK: u8 = 1 << 3;
const BG_CHARACTERS_BITMASK: u8 = 1 << 4;
const WINDOW_BITMASK: u8 = 1 << 5;
const WIN_AREA_BITMASK: u8 = 1 << 6;
const CONTROLLER_BITMASK: u8 = 1 << 7;

const MODE_BITMASK: u8 = 1 | 1 << 1;
