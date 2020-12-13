use graphic_core::{Renderer};

pub struct SDLRenderer {
    pub(crate) canvas: sdl2::render::Canvas<sdl2::video::Window>
}

impl Renderer for SDLRenderer {
    fn clear(&mut self, r: u8, g: u8, b: u8) {
        self.canvas.set_draw_color(sdl2::pixels::Color::RGB(r, g, b));
    }
}
