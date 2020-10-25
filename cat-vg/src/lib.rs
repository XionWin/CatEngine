extern crate gl_graphic;
extern crate graphic_core;

use graphic_core::{Window, WindowCreator};

pub fn test() {
    let creator = gl_graphic::SDLWindowCreator::new();
    let mut window = Window::new(0, 0, 800, 800, "Game", creator);
    
    window.show();
}
