use crate::constants::*;

pub struct Memory {
    pub rom: Vec<u8>,
    pub ram: Vec<u8>,
    pub video_memory: Vec<u8>,
    pub framebuffer: Vec<u32>,
}

impl Memory {
    pub fn new() -> Self {
        Self {
            rom: vec![0; ROM_SIZE],
            ram: vec![0; RAM_SIZE],
            video_memory: vec![0; VIDEO_MEM_SIZE],
            framebuffer: vec![0; SCREEN_WIDTH * SCREEN_HEIGHT],
        }
    }

    pub fn load_rom(&mut self, data: &[u8]) {
        self.rom[..data.len()].copy_from_slice(data);
    }

    pub fn read_word(&self, address: usize) -> u32 {
        if ROM_START <= address && address < ROM_END {
            let offset = address - ROM_START;
            u32::from_le_bytes([
                self.rom[offset],
                self.rom[offset + 1],
                self.rom[offset + 2],
                self.rom[offset + 3],
            ])
        } else if RAM_START <= address && address < RAM_END {
            let offset = address - RAM_START;
            u32::from_le_bytes([
                self.ram[offset],
                self.ram[offset + 1],
                self.ram[offset + 2],
                self.ram[offset + 3],
            ])
        } else if VIDEO_MEM_START <= address && address < VIDEO_MEM_END {
            let offset = address - VIDEO_MEM_START;
            u32::from_le_bytes([
                self.video_memory[offset],
                self.video_memory[offset + 1],
                self.video_memory[offset + 2],
                self.video_memory[offset + 3],
            ])
        } else {
            panic!("Invalid memory read at address: {:08x}", address);
        }
    }

    pub fn write_word(&mut self, address: usize, value: u32) {
        let bytes = value.to_le_bytes();
        if RAM_START <= address && address < RAM_END {
            let offset = address - RAM_START;
            self.ram[offset..offset + 4].copy_from_slice(&bytes);
        } else if VIDEO_MEM_START <= address && address < VIDEO_MEM_END {
            let offset = address - VIDEO_MEM_START;
            self.video_memory[offset..offset + 4].copy_from_slice(&bytes);
        } else {
            panic!("Invalid memory write at address: {:08x}", address);
        }
    }

    pub fn get_framebuffer(&self) -> &[u32] {
        &self.framebuffer
    }
}
