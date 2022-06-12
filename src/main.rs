use std::{
    sync::mpsc::channel,
    time::{Duration, Instant},
};

use constants::SYSCLK_FREQ;
use frontend::{Frontend, FrontendStatus};
use gpu::Gpu;
use sdl2::{event::Event, keyboard::Keycode, pixels::Color};

use crate::{bus::Bus, cartridge::Cartridge, cpu::Cpu};

mod boot;
mod bus;
mod buttons;
mod cartridge;
mod constants;
mod cpu;
mod disassembler;
mod frontend;
mod gpu;
mod interrupts;
mod ram;
mod register;
mod spu;
mod timer;

/// Number of instructions executed between sleeps (i.e. giving the
/// hand back to the scheduler). Low values increase CPU usage and can
/// result in poor performance, high values will cause stuttering.
const GRANULARITY: i64 = 0x10000;

const BATCH_DURATION_NS: i64 = GRANULARITY * (1_000_000_000 / SYSCLK_FREQ);
const BATCH_DURATION_MS: u64 = (BATCH_DURATION_NS / 1_000_000) as u64;

fn main() -> std::io::Result<()> {
    let frontend = Frontend::new();
    let sdl_context = frontend.get_sdl_context();
    let mut display = frontend.new_display(sdl_context);

    let mut cpu = Cpu::new();
    let cartridge = Cartridge::new("tetris.gb");
    let gpu = Gpu::new(&mut display);
    let mut bus = Bus::new(cartridge, gpu);

    let (tick_tx, tick_rx) = channel();

    ::std::thread::spawn(move || {
        loop {
            std::thread::sleep(Duration::from_millis(BATCH_DURATION_MS));
            if let Err(_) = tick_tx.send(()) {
                // End thread
                return;
            }
        }
    });

    let mut cycles = 0;

    'running: loop {
        while cycles < GRANULARITY {
            let cpu_cycles = cpu.next(&mut bus)? as i64;
            cycles += cpu_cycles;
        }

        cycles -= GRANULARITY;

        if let Err(e) = tick_rx.recv() {
            panic!("Timer died: {:?}", e);
        }

        match frontend.update() {
            FrontendStatus::Quit => break 'running,
            _ => {}
        }
    }

    Ok(())
}
