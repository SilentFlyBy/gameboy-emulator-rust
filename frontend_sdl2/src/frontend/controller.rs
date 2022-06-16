use emulator::bus::Bus;
use sdl2::{event::Event, keyboard::Keycode};

pub struct Controller {}

impl Controller {
    pub fn new() -> Self {
        Controller {}
    }

    pub fn update(&self, event: Event, bus: &mut Bus) {
        let buttons = &mut bus.buttons;
        match event {
            Event::KeyDown {
                keycode: Some(code),
                ..
            } => {
                match code {
                    Keycode::Left => buttons.set_left(true),
                    Keycode::Right => buttons.set_right(true),
                    Keycode::Up => buttons.set_up(true),
                    Keycode::Down => buttons.set_down(true),
                    Keycode::A => buttons.set_start(true),
                    Keycode::S => buttons.set_select(true),
                    Keycode::X => buttons.set_a(true),
                    Keycode::Y => buttons.set_b(true),
                    _ => {}
                };
                bus.interrupts.joypad_request();
            }
            Event::KeyUp {
                keycode: Some(code),
                ..
            } => {
                match code {
                    Keycode::Left => buttons.set_left(false),
                    Keycode::Right => buttons.set_right(false),
                    Keycode::Up => buttons.set_up(false),
                    Keycode::Down => buttons.set_down(false),
                    Keycode::A => buttons.set_start(false),
                    Keycode::S => buttons.set_select(false),
                    Keycode::X => buttons.set_a(false),
                    Keycode::Y => buttons.set_b(false),
                    _ => {}
                };

                bus.interrupts.joypad_request();
            }
            _ => {}
        }
    }
}
