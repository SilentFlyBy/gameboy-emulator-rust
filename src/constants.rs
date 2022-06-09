/// Gameboy sysclk frequency: 4.19Mhz
pub const SYSCLK_FREQ: i64 = 0x400000;

pub const SYSCLK_FREQ_16: i64 = SYSCLK_FREQ / 0x10; // 262144 Hz
pub const SYSCLK_FREQ_64: i64 = SYSCLK_FREQ / 0x40; // 65536 Hz
pub const SYSCLK_FREQ_256: i64 = SYSCLK_FREQ / 0x100; // 16384 Hz
pub const SYSCLK_FREQ_1024: i64 = SYSCLK_FREQ / 0x400; // 4096 Hz

pub const INTERRUPT_REQUEST_ADDRESS: u16 = 0xFF0F;
pub const INTERRUPT_ENABLE_ADDRESS: u16 = 0xFFFF;
