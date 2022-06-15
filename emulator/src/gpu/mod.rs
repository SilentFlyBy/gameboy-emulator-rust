use core::{num, panic};
use std::io;

use crate::{
    bus::FetchWrite,
    constants::{OAM_END_ADDRESS, OAM_START_ADDRESS, VRAM_END_ADDRESS, VRAM_START_ADDRESS},
    interrupts::Interrupts,
    ram::Ram,
    register::Register8,
};

use self::registers::{Mode, LCDC, STAT};

mod registers;

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

const TILE_DATA_BLOCK_0_ADDRESS: u16 = 0x8000;
const TILE_DATA_BLOCK_1_ADDRESS: u16 = 0x8800;
const TILE_DATA_BLOCK_2_ADDRESS: u16 = 0x9000;

const TILE_MAP_BLOCK_0_ADDRESS: u16 = 0x9800;
const TILE_MAP_BLOCK_1_ADDRESS: u16 = 0x9C00;

const TILE_LEN: u8 = 16;

pub enum DmgColor {
    White,
    LightGrey,
    DarkGrey,
    Black,
}

pub trait Display {
    fn render_pixel(&mut self, x: u8, y: u8, dmg_color: DmgColor);
    fn present(&mut self);
}

pub struct Gpu<'a> {
    display: &'a mut dyn Display,
    vram: Ram,
    oam: Ram,
    modeclock: u32,
    lcdc: LCDC,
    stat: STAT,
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
    selected_oam_objects: [u16; 10],
}

impl<'a> Gpu<'a> {
    pub fn new(display: &'a mut dyn Display) -> Self {
        let vram: Ram = Ram::new(0x2000, VRAM_START_ADDRESS);
        let oam: Ram = Ram::new(0xA0, OAM_START_ADDRESS);
        Gpu {
            display,
            vram,
            oam,
            modeclock: 0,
            lcdc: LCDC::new(),
            stat: STAT::new(),
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
            selected_oam_objects: [0x0; 10],
        }
    }
    fn get_address_target(&mut self, address: u16) -> io::Result<&mut dyn FetchWrite> {
        match address {
            VRAM_START_ADDRESS..=VRAM_END_ADDRESS => Ok(&mut self.vram),
            OAM_START_ADDRESS..=OAM_END_ADDRESS => Ok(&mut self.oam),
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

    pub fn get_dma(&self) -> u8 {
        self.dma
    }

    pub fn next(&mut self, cycles: u8, interrupts: &mut Interrupts) {
        self.modeclock += cycles as u32;

        match self.stat.get_mode() {
            Mode::SCAN_OAM => {
                if self.modeclock >= OAM_CYCLES {
                    self.modeclock = self.modeclock % OAM_CYCLES;
                    self.stat.set_mode(Mode::SCAN_VRAM);
                }
            }
            Mode::SCAN_VRAM => {
                if self.modeclock >= VRAM_CYCLES {
                    self.modeclock = self.modeclock % VRAM_CYCLES;
                    self.stat.set_mode(Mode::HBLANK);

                    if self.stat.get_mode_hblank_interrupt() {
                        interrupts.set_lcd_stat_request(true);
                    }

                    if self.stat.get_lyc_coincidence_interrupt() && self.stat.get_lyc_coincidence()
                    {
                        interrupts.set_lcd_stat_request(true);
                    }

                    self.stat.set_lyc_match(self.stat.get_lyc_coincidence());
                }
            }
            Mode::HBLANK => {
                if self.modeclock >= HBLANK_CYCLES {
                    self.modeclock = self.modeclock % HBLANK_CYCLES;

                    self.render_line();
                    self.ly = self.ly.wrapping_add(1);

                    if self.ly == VBLANK_START_LINE {
                        self.stat.set_mode(Mode::VBLANK);
                        self.present_image();

                        interrupts.set_v_blank_request(true);

                        if self.stat.get_mode_vblank_interrupt() {
                            interrupts.set_lcd_stat_request(true);
                        }
                    } else {
                        self.stat.set_mode(Mode::SCAN_OAM);
                        if self.stat.get_mode_oam_interrupt() {
                            interrupts.set_lcd_stat_request(true);
                        }
                    }
                }
            }
            Mode::VBLANK => {
                if self.modeclock >= SCANLINE_CYCLES {
                    self.modeclock = self.modeclock % SCANLINE_CYCLES;
                    self.ly = self.ly.wrapping_add(1);

                    if self.ly > VBLANK_END_LINE {
                        self.stat.set_mode(Mode::SCAN_OAM);
                        self.ly = 0;

                        if self.stat.get_mode_oam_interrupt() {
                            interrupts.set_lcd_stat_request(true);
                        }
                    }
                }
            }
        }
    }

    fn present_image(&mut self) {
        self.display.present();
    }

    fn render_line(&mut self) {
        if !self.lcdc.get_lcd_enable() {
            return;
        }

        self.select_oam_objects();

        for x in 0u8..160 {
            self.render_pixel(x, self.ly);
        }
    }

    fn select_oam_objects(&mut self) {
        for object in self.selected_oam_objects.iter_mut() {
            *object = 0x0;
        }

        let mut num_obj = 0;

        for address in (OAM_START_ADDRESS..OAM_END_ADDRESS).step_by(4) {
            if num_obj > 9 {
                break;
            }

            let obj_size = if self.lcdc.get_obj_size() { 16 } else { 8 };
            let y_position = self.oam.fetch8(address).unwrap();
            if self.ly as i32 >= (y_position as i32 - 16)
                && self.ly as i32 <= (y_position as i32 - 16 + obj_size)
            {
                self.selected_oam_objects[num_obj] = address;
                num_obj += 1;
            }
        }
    }

    fn render_pixel(&mut self, x: u8, y: u8) {
        let mut color = self.get_bg_color(x, y);
        if self.lcdc.get_obj_enable() {
            color = self.get_obj_color(x, y, color);
        }

        self.display.render_pixel(x, y, color);
    }

    fn get_obj_color(&mut self, x: u8, y: u8, color: DmgColor) -> DmgColor {
        for address in self.selected_oam_objects {
            if address == 0x0 {
                break;
            }

            let y_position = self.oam.fetch8(address).unwrap() as i32 - 16;
            let x_position = self.oam.fetch8(address + 1).unwrap() as i32 - 8;
            let mut tile_index = self.oam.fetch8(address + 2).unwrap();
            let attrs = self.oam.fetch8(address + 3).unwrap();

            if x as i32 >= x_position && x as i32 <= x_position + 8 {
                if self.lcdc.get_obj_size() && y as i32 > y_position + 8 {
                    tile_index += 1;
                }

                // println!("{:#X} {:#X}", tile_index, attrs);

                let line_x = x as i32 - x_position;
                let line_y = y as i32 - y_position;
                let tile_address =
                    TILE_DATA_BLOCK_0_ADDRESS + (tile_index as u16 * TILE_LEN as u16);
                let line_address = tile_address + (line_y as u16 * 2);
                let tile_line = self.vram.fetch16(line_address).unwrap();

                return self.get_line_pixel_color(tile_line, line_x as u8);
            }
        }

        color
    }

    fn get_bg_color(&mut self, x: u8, y: u8) -> DmgColor {
        let tile_index = self.get_bg_tile_index(x, y);
        let pixel_x = x % 8;
        let pixel_y = y % 8;

        let line_pixels = self.get_bg_tile_line(tile_index, pixel_y);

        self.get_line_pixel_color(line_pixels, pixel_x)
    }

    fn get_bg_tile_line(&mut self, tile_index: u8, line: u8) -> u16 {
        let addressing_mode = self.lcdc.get_bg_characters();
        let tile_address = match addressing_mode {
            true => TILE_DATA_BLOCK_0_ADDRESS + (tile_index as u16 * TILE_LEN as u16),
            false => {
                if tile_index <= 127 {
                    TILE_DATA_BLOCK_2_ADDRESS + (tile_index as u16 * TILE_LEN as u16)
                } else {
                    TILE_DATA_BLOCK_1_ADDRESS + (tile_index as u16 * TILE_LEN as u16)
                }
            }
        };

        let line_address = tile_address + (line as u16 * 2);
        self.vram.fetch16(line_address).unwrap()
    }

    fn get_bg_tile_index(&mut self, x: u8, y: u8) -> u8 {
        let bg_x = x + self.scx;
        let bg_y = y + self.scy;

        let tile_x = bg_x / 8;
        let tile_y = bg_y / 8;

        let tile_no = (tile_y as u16 * 32) + (tile_x as u16);

        let tile_map_block = self.lcdc.get_bg_area();
        let address = match tile_map_block {
            true => TILE_MAP_BLOCK_1_ADDRESS + tile_no,
            false => TILE_MAP_BLOCK_0_ADDRESS + tile_no,
        };

        self.vram.fetch8(address).unwrap()
    }

    fn get_line_pixel_color(&self, line: u16, line_x: u8) -> DmgColor {
        let lsb = (line >> (15 - line_x as u16)) & 1;
        let msb = (line >> (7 - line_x as u16)) & 1;

        let color = lsb | (msb << 1);

        let palette_color = (self.bgp >> (color * 2)) & 0b11;

        match palette_color {
            0 => DmgColor::White,
            1 => DmgColor::LightGrey,
            2 => DmgColor::DarkGrey,
            3 => DmgColor::Black,
            _ => panic!("Unsupported color value"),
        }
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
