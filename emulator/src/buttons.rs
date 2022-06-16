use crate::bus::FetchWrite;

const P10_RIGHT_OR_A_BITMASK: u8 = 1;
const P11_LEFT_OR_B_BITMASK: u8 = 1 << 1;
const P12_UP_OR_SELECT_BITMASK: u8 = 1 << 2;
const P13_DOWN_OR_START_BITMASK: u8 = 1 << 3;
const P14_SELECT_DIRECTION_BITMASK: u8 = 1 << 4;
const P15_SELECT_ACTION_BITMASK: u8 = 1 << 5;

pub struct Buttons {
    left: bool,
    right: bool,
    up: bool,
    down: bool,
    a: bool,
    b: bool,
    start: bool,
    select: bool,
    directions: bool,
    actions: bool,
}

impl Buttons {
    pub fn new() -> Self {
        Buttons {
            left: false,
            right: false,
            up: false,
            down: false,
            a: false,
            b: false,
            start: false,
            select: false,
            directions: false,
            actions: false,
        }
    }

    pub fn set_left(&mut self, value: bool) {
        self.left = value;
    }
    pub fn set_right(&mut self, value: bool) {
        self.right = value;
    }
    pub fn set_up(&mut self, value: bool) {
        self.up = value;
    }
    pub fn set_down(&mut self, value: bool) {
        self.down = value;
    }
    pub fn set_a(&mut self, value: bool) {
        self.a = value;
    }
    pub fn set_b(&mut self, value: bool) {
        self.b = value;
    }
    pub fn set_start(&mut self, value: bool) {
        self.start = value;
    }
    pub fn set_select(&mut self, value: bool) {
        self.select = value;
    }
}

impl FetchWrite for Buttons {
    fn fetch8(&mut self, _: u16) -> Result<u8, std::io::Error> {
        let mut button_register: u8 = 0xC0;
        if self.directions {
            if !self.left {
                button_register |= P11_LEFT_OR_B_BITMASK;
            }
            if !self.right {
                button_register |= P10_RIGHT_OR_A_BITMASK;
            }
            if !self.up {
                button_register |= P12_UP_OR_SELECT_BITMASK;
            }
            if !self.down {
                button_register |= P13_DOWN_OR_START_BITMASK;
            }
        }
        if self.actions {
            if !self.a {
                button_register |= P10_RIGHT_OR_A_BITMASK;
            }
            if !self.b {
                button_register |= P11_LEFT_OR_B_BITMASK;
            }
            if !self.start {
                button_register |= P13_DOWN_OR_START_BITMASK;
            }
            if !self.select {
                button_register |= P12_UP_OR_SELECT_BITMASK;
            }
        }
        Ok(button_register)
    }

    fn fetch16(&mut self, _: u16) -> Result<u16, std::io::Error> {
        panic!("16 bit operations not supported with 8 bit register")
    }

    fn write8(&mut self, _: u16, value: u8) -> std::io::Result<()> {
        self.directions = value & P14_SELECT_DIRECTION_BITMASK == 0;
        self.actions = value & P15_SELECT_ACTION_BITMASK == 0;
        Ok(())
    }

    fn write16(&mut self, _: u16, _: u16) -> std::io::Result<()> {
        panic!("16 bit operations not supported with 8 bit register")
    }
}
