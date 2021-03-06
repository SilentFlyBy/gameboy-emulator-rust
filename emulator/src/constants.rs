/// Gameboy sysclk frequency: 4.19Mhz
pub const SYSCLK_FREQ: i64 = 0x400000;

pub const SYSCLK_FREQ_16: i64 = SYSCLK_FREQ / 0x10; // 262144 Hz
pub const SYSCLK_FREQ_64: i64 = SYSCLK_FREQ / 0x40; // 65536 Hz
pub const SYSCLK_FREQ_256: i64 = SYSCLK_FREQ / 0x100; // 16384 Hz
pub const SYSCLK_FREQ_1024: i64 = SYSCLK_FREQ / 0x400; // 4096 Hz

/// Number of instructions executed between sleeps (i.e. giving the
/// hand back to the scheduler). Low values increase CPU usage and can
/// result in poor performance, high values will cause stuttering.
pub const GRANULARITY: i64 = 0x10000;

pub const BATCH_DURATION_NS: i64 = GRANULARITY * (1_000_000_000 / SYSCLK_FREQ);
pub const BATCH_DURATION_MS: u64 = (BATCH_DURATION_NS / 1_000_000) as u64;

pub const INTERRUPT_REQUEST_ADDRESS: u16 = 0xFF0F;
pub const INTERRUPT_ENABLE_ADDRESS: u16 = 0xFFFF;

pub const VRAM_START_ADDRESS: u16 = 0x8000;
pub const VRAM_END_ADDRESS: u16 = 0x9FFF;

pub const OAM_START_ADDRESS: u16 = 0xFE00;
pub const OAM_END_ADDRESS: u16 = 0xFE9F;
