use clap::Parser;
use emulator::bus::{Bus, FetchWrite};
use emulator::cartridge::Cartridge;
use emulator::constants::{BATCH_DURATION_MS, GRANULARITY};
use emulator::cpu::Cpu;
use emulator::gpu::{DmgColor, Gpu};
use emulator::register::Register8;
use frontend::display::Sdl2Display;
use frontend::{Frontend, FrontendStatus};
use std::{sync::mpsc::channel, time::Duration};

mod frontend;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    /// Rom file
    #[clap(value_parser)]
    file: String,

    /// Print disassembly
    #[clap(short, long, action)]
    disassemble: bool,
}

fn main() {
    let cli = Cli::parse();

    let frontend = Frontend::new();
    let sdl_context = frontend.get_sdl_context();
    let mut display = frontend.new_display(sdl_context);

    let mut cpu = Cpu::new(cli.disassemble);
    let cartridge = Cartridge::new(cli.file.as_str());
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
            let cpu_cycles = cpu.next(&mut bus).unwrap() as i64;
            cycles += cpu_cycles;
        }

        cycles -= GRANULARITY;

        if let Err(e) = tick_rx.recv() {
            panic!("Timer died: {:?}", e);
        }

        match frontend.update(&mut bus) {
            FrontendStatus::Quit => break 'running,
            _ => {}
        }
    }
}
