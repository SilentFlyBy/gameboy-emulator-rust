use sdl2::{pixels::Color, rect::Rect, render::Canvas, video::Window};

const SCREEN_WIDTH: u8 = 160;
const SCREEN_HEIGHT: u8 = 144;

pub enum DMG_COLOR {
    Black,
    DarkGrey,
    LightGrey,
    White,
}

pub struct Display {
    canvas: Canvas<Window>,
}

impl Display {
    pub fn new(sdl: &sdl2::Sdl) -> Self {
        let video_subsystem = sdl.video().unwrap();

        let window = video_subsystem
            .window("rust-sdl2 demo", 800, 600)
            .position_centered()
            .resizable()
            .build()
            .unwrap();

        let canvas = window.into_canvas().build().unwrap();
        Display { canvas }
    }

    pub fn render_pixel(&mut self, x: u8, y: u8, dmg_color: DMG_COLOR) {
        let (window_width, window_height) = self.canvas.window().size();
        let viewport_x = ((x as f64 / SCREEN_WIDTH as f64) * window_width as f64).ceil() as i32;
        let viewport_y = ((y as f64 / SCREEN_HEIGHT as f64) * window_height as f64).ceil() as i32;
        let viewport_w = ((1.0 / SCREEN_WIDTH as f64) * window_width as f64).ceil() as u32;
        let viewport_h = ((1.0 / SCREEN_HEIGHT as f64) * window_height as f64).ceil() as u32;

        let color = match dmg_color {
            DMG_COLOR::Black => Color::RGB(0x00, 0x00, 0x00),
            DMG_COLOR::DarkGrey => Color::RGB(0x55, 0x55, 0x55),
            DMG_COLOR::LightGrey => Color::RGB(0xAB, 0xAB, 0xAB),
            DMG_COLOR::White => Color::RGB(0xFF, 0xFF, 0xFF),
        };

        let rect = Rect::new(viewport_x, viewport_y, viewport_w, viewport_h);

        self.canvas.set_draw_color(color);
        self.canvas.fill_rect(rect).unwrap();
    }

    pub fn present(&mut self) {
        self.canvas.present();
        self.canvas.set_draw_color(Color::RGB(255, 255, 255));
        self.canvas.clear();
    }
}
