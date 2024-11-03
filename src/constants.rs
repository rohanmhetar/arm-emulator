// Memory Map Constants
pub const ROM_START: usize = 0x08000000;
pub const ROM_END: usize = 0x0A000000;
pub const RAM_START: usize = 0x02000000;
pub const RAM_END: usize = 0x02040000;
pub const VIDEO_MEM_START: usize = 0x06000000;
pub const VIDEO_MEM_END: usize = 0x06018000;

// Memory Sizes
pub const ROM_SIZE: usize = 32 * 1024 * 1024;  // 32 MB
pub const RAM_SIZE: usize = 256 * 1024;        // 256 KB
pub const VIDEO_MEM_SIZE: usize = 96 * 1024;   // 96 KB

// Screen Dimensions
pub const SCREEN_WIDTH: usize = 240;
pub const SCREEN_HEIGHT: usize = 160;

// Opcodes for ARM7TDMI instruction types
pub const OPCODE_DATA_PROCESSING: u32 = 0b00;
pub const OPCODE_MEMORY_ACCESS: u32 = 0b01;
pub const OPCODE_BRANCH: u32 = 0b10;
