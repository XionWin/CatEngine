extern crate gl_graphic;
extern crate graphic_core;

use gl_graphic::{SDLWindow};
use graphic_core::Renderer;


pub fn test() {
    let mut window = SDLWindow::new(0, 0, 600, 600, "Game");
    
    window.show(&|renderer| renderer.clear(255u8, 64u8, 64u8));
}
