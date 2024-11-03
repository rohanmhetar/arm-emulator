extern crate sdl2;
use sdl2::pixels::PixelFormatEnum;
use sdl2::render::WindowCanvas;
use sdl2::Sdl;

pub struct Graphics {
    canvas: WindowCanvas,
}

impl Graphics {
    pub fn new(sdl_context: &Sdl) -> Self {
        let video_subsystem = sdl_context.video().unwrap();
        let window = video_subsystem
            .window("GBA Emulator", 240, 160)
            .position_centered()
            .build()
            .expect("Could not initialize video subsystem");

        let canvas = window
            .into_canvas()
            .present_vsync()
            .build()
            .expect("Could not make canvas");

        Self { canvas }
    }

    pub fn render_frame(&mut self, framebuffer: &[u32]) {
        let texture_creator = self.canvas.texture_creator();
        let mut texture = texture_creator
            .create_texture_streaming(PixelFormatEnum::ARGB8888, 240, 160)
            .expect("Could not create texture");

        texture
            .update(None, bytemuck::cast_slice(framebuffer), 240 * 4)
            .expect("Could not update texture");

        self.canvas.clear();
        self.canvas.copy(&texture, None, None).expect("Render failed");
        self.canvas.present();
    }
}
