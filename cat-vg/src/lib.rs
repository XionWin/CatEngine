extern crate gl_graphic;
extern crate graphic_core;

use graphic_core::{Window, WindowCreator};

pub fn test() {
    let mut creator = gl_graphic::SDLWindowCreator::new();
    let mut window = Window::new(0, 0, 800, 800, "Game", &mut creator);
    // window.set_size(200, 200);
    window.show();
}
