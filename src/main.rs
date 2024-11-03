mod cpu;
mod gpu;
mod audio;
mod memory;
mod graphics;
mod constants;
mod utils;

use cpu::ARM7TDMI;
use gpu::GPU;
use audio::Audio;
use memory::Memory;
use graphics::Graphics;

use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let mut graphics = Graphics::new(&sdl_context);

    let shared_memory = Arc::new(Mutex::new(Memory::new()));
    let shared_audio_buffer = Arc::new(Mutex::new(vec![0; 1024]));

    let cpu_memory = Arc::clone(&shared_memory);
    let gpu_memory = Arc::clone(&shared_memory);
    let audio_buffer = Arc::clone(&shared_audio_buffer);

    // Start CPU thread
    let cpu_thread = thread::spawn(move || {
        let mut cpu = ARM7TDMI::new(cpu_memory);
        loop {
            cpu.execute_instruction();
        }
    });

    // Start GPU thread
    let gpu_thread = thread::spawn(move || {
        let mut gpu = GPU::new(gpu_memory);
        loop {
            gpu.render_frame();
        }
    });

    // Start Audio thread
    let audio = Audio::new(44100, 2); // Stereo, 44.1kHz sample rate
    let audio_thread = thread::spawn(move || {
        audio.play_audio(audio_buffer);
    });

    // Main rendering loop
    loop {
        let framebuffer = shared_memory.lock().unwrap().get_framebuffer();
        graphics.render_frame(framebuffer);
    }

    // Wait for threads to complete
    let _ = cpu_thread.join();
    let _ = gpu_thread.join();
    let _ = audio_thread.join();
}
