use crate::memory::Memory;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

const SCREEN_WIDTH: usize = 240;
const SCREEN_HEIGHT: usize = 160;
const TILE_SIZE: usize = 8;
const NUM_TILES_X: usize = SCREEN_WIDTH / TILE_SIZE;
const NUM_TILES_Y: usize = SCREEN_HEIGHT / TILE_SIZE;
const NUM_COLORS: usize = 256;

pub struct GPU {
    memory: Arc<Mutex<Memory>>,
    framebuffer: Vec<u32>,
    palette: Vec<u32>,
}

impl GPU {
    pub fn new(memory: Arc<Mutex<Memory>>) -> Self {
        Self {
            memory,
            framebuffer: vec![0; SCREEN_WIDTH * SCREEN_HEIGHT],
            palette: vec![0xFF000000; NUM_COLORS],
        }
    }

    pub fn render_frame(&mut self) {
        self.generate_palette();
        self.draw_background();
        self.draw_sprites();
        
        println!("Framebuffer [0..10]: {:?}", &self.framebuffer[0..10]);
        thread::sleep(Duration::from_millis(16));
    }

    fn generate_palette(&mut self) {
        for i in 0..NUM_COLORS {
            let r = (i as u8) << 3;
            let g = (255 - i as u8) << 1;
            let b = (i as u8) << 2;
            self.palette[i] = (0xFF << 24) | ((r as u32) << 16) | ((g as u32) << 8) | (b as u32);
        }
    }

    fn draw_background(&mut self) {
        for ty in 0..NUM_TILES_Y {
            for tx in 0..NUM_TILES_X {
                let tile_index = (ty * NUM_TILES_X + tx) % NUM_COLORS;
                self.draw_tile(tx, ty, tile_index);
            }
        }
    }

    fn draw_tile(&mut self, tx: usize, ty: usize, tile_index: usize) {
        let color = self.palette[tile_index % NUM_COLORS];
        for y in 0..TILE_SIZE {
            for x in 0..TILE_SIZE {
                let px = tx * TILE_SIZE + x;
                let py = ty * TILE_SIZE + y;
                if px < SCREEN_WIDTH && py < SCREEN_HEIGHT {
                    self.framebuffer[py * SCREEN_WIDTH + px] = color;
                }
            }
        }
    }

    fn draw_sprites(&mut self) {
        let sprite_positions = [(30, 30), (100, 50), (200, 100)];
        let sprite_size = 16;

        for &(sx, sy) in &sprite_positions {
            for y in 0..sprite_size {
                for x in 0..sprite_size {
                    let color_index = ((y + x) % NUM_COLORS) as usize;
                    let color = self.palette[color_index];
                    let px = sx + x;
                    let py = sy + y;
                    if px < SCREEN_WIDTH && py < SCREEN_HEIGHT {
                        self.framebuffer[py * SCREEN_WIDTH + px] = color;
                    }
                }
            }
        }
    }
}
