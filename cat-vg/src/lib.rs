extern crate gl_graphic;
extern crate graphic_core;

use gl_graphic::{SDLWindow};
pub fn test() {
    let mut window = SDLWindow::new(0, 0, 800, 800, "Game");
    window.show();
}
