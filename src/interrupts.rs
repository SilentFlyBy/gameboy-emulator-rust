use crate::{
    bus::FetchWrite,
    constants::{INTERRUPT_ENABLE_ADDRESS, INTERRUPT_REQUEST_ADDRESS},
};

const V_BLANK_BITMASK: u8 = 1;
const LCD_STAT_BITMASK: u8 = 1 << 1;
const TIMER_BITMASK: u8 = 1 << 2;
const SERIAL_BITMASK: u8 = 1 << 3;
const JOYPAD_BITMASK: u8 = 1 << 4;

const V_BLANK_HANDLER_ADDRESS: u16 = 0x40;
const LCD_STAT_HANDLER_ADDRESS: u16 = 0x48;
const TIMER_HANDLER_ADDRESS: u16 = 0x50;
const SERIAL_HANDLER_ADDRESS: u16 = 0x58;
const JOYPAD_HANDLER_ADDRESS: u16 = 0x60;

pub struct Interrupts {
    enable_register: u8,
    request_register: u8,
    master: bool,
}

impl Interrupts {
    pub fn new() -> Self {
        Interrupts {
            enable_register: 0,
            request_register: 0,
            master: true,
        }
    }

    pub fn interrupt_pending(&self) -> bool {
        self.master && (self.enable_register & self.request_register != 0)
    }

    pub fn ack_and_get_pending_address(&mut self) -> Option<u16> {
        if self.v_blank_enable() && self.v_blank_request() {
            self.set_v_blank_request(false);
            Some(V_BLANK_HANDLER_ADDRESS)
        } else if self.lcd_stat_enable() && self.lcd_stat_request() {
            self.set_lcd_stat_request(false);
            Some(LCD_STAT_HANDLER_ADDRESS)
        } else if self.timer_enable() && self.timer_request() {
            self.set_timer_request(false);
            Some(TIMER_HANDLER_ADDRESS)
        } else if self.serial_enable() && self.serial_request() {
            self.set_serial_request(false);
            Some(SERIAL_HANDLER_ADDRESS)
        } else if self.joypad_enable() && self.joypad_request() {
            self.set_joypad_request(false);
            Some(JOYPAD_HANDLER_ADDRESS)
        } else {
            None
        }
    }

    pub fn v_blank_enable(&self) -> bool {
        (self.enable_register & V_BLANK_BITMASK) > 0
    }

    pub fn lcd_stat_enable(&self) -> bool {
        (self.enable_register & LCD_STAT_BITMASK) > 0
    }

    pub fn timer_enable(&self) -> bool {
        (self.enable_register & TIMER_BITMASK) > 0
    }

    pub fn serial_enable(&self) -> bool {
        (self.enable_register & SERIAL_BITMASK) > 0
    }

    pub fn joypad_enable(&self) -> bool {
        (self.enable_register & JOYPAD_BITMASK) > 0
    }

    pub fn v_blank_request(&self) -> bool {
        (self.request_register & V_BLANK_BITMASK) > 0
    }

    pub fn set_v_blank_request(&mut self, value: bool) {
        if value {
            self.request_register |= V_BLANK_BITMASK;
        } else {
            self.request_register &= !V_BLANK_BITMASK;
        }
    }

    pub fn lcd_stat_request(&self) -> bool {
        (self.request_register & LCD_STAT_BITMASK) > 0
    }

    pub fn set_lcd_stat_request(&mut self, value: bool) {
        if value {
            self.request_register |= LCD_STAT_BITMASK;
        } else {
            self.request_register &= !LCD_STAT_BITMASK;
        }
    }

    pub fn timer_request(&self) -> bool {
        (self.request_register & TIMER_BITMASK) > 0
    }

    pub fn set_timer_request(&mut self, value: bool) {
        if value {
            self.request_register |= TIMER_BITMASK;
        } else {
            self.request_register &= !TIMER_BITMASK;
        }
    }

    pub fn serial_request(&self) -> bool {
        (self.request_register & SERIAL_BITMASK) > 0
    }

    pub fn set_serial_request(&mut self, value: bool) {
        if value {
            self.request_register |= SERIAL_BITMASK;
        } else {
            self.request_register &= !SERIAL_BITMASK;
        }
    }

    pub fn joypad_request(&self) -> bool {
        (self.request_register & JOYPAD_BITMASK) > 0
    }

    pub fn set_joypad_request(&mut self, value: bool) {
        if value {
            self.request_register |= JOYPAD_BITMASK;
        } else {
            self.request_register &= !JOYPAD_BITMASK;
        }
    }

    pub fn master_enabled(&self) -> bool {
        self.master
    }

    pub fn enable_master(&mut self) {
        self.master = true
    }

    pub fn disable_master(&mut self) {
        self.master = false
    }

    fn set_enable(&mut self, val: u8) {
        self.enable_register = val;
    }

    fn set_request(&mut self, val: u8) {
        self.request_register = val;
    }
}

impl FetchWrite for Interrupts {
    fn fetch8(&mut self, address: u16) -> Result<u8, std::io::Error> {
        match address {
            INTERRUPT_ENABLE_ADDRESS => Ok(self.enable_register),
            INTERRUPT_REQUEST_ADDRESS => Ok(self.request_register),
            _ => panic!("Accessed unsupported interrupt address"),
        }
    }

    fn fetch16(&mut self, _: u16) -> Result<u16, std::io::Error> {
        panic!("16 bit operations not supported on interrupt register");
    }

    fn write8(&mut self, address: u16, value: u8) -> std::io::Result<()> {
        match address {
            INTERRUPT_ENABLE_ADDRESS => self.set_enable(value),
            INTERRUPT_REQUEST_ADDRESS => self.set_request(value),
            _ => panic!("Accessed unsupported interrupt address"),
        }

        Ok(())
    }

    fn write16(&mut self, _: u16, _: u16) -> std::io::Result<()> {
        panic!("16 bit operations not supported on interrupt register");
    }
}
