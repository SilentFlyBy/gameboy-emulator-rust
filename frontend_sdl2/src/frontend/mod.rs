use std::borrow::{Borrow, BorrowMut};

use sdl2::{
    event::{Event, WindowEvent},
    keyboard::Keycode,
    pixels::Color,
    render::Canvas,
    video::Window,
    Sdl,
};

use self::{controller::Controller, display::Sdl2Display};

pub mod controller;
pub mod display;

pub enum FrontendStatus {
    Ok,
    Quit,
    Error,
}
pub struct Frontend {
    context: Sdl,
    controller: Controller,
}

impl Frontend {
    pub fn new() -> Self {
        let sdl_context = sdl2::init().unwrap();
        let display = Sdl2Display::new(&sdl_context);
        let controller = Controller::new();
        Frontend {
            context: sdl_context,
            controller,
        }
    }

    pub fn update(&self) -> FrontendStatus {
        let mut event_pump = self.context.event_pump().unwrap();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => return FrontendStatus::Quit,
                Event::Window {
                    win_event: WindowEvent::Resized(..),
                    ..
                } => {}
                _ => {}
            }
        }

        FrontendStatus::Ok
    }

    pub fn new_display(&self, sdl: &sdl2::Sdl) -> Sdl2Display {
        Sdl2Display::new(sdl)
    }

    pub fn get_controller(&self) -> &Controller {
        self.controller.borrow()
    }

    pub fn get_sdl_context(&self) -> &sdl2::Sdl {
        self.context.borrow()
    }
}
