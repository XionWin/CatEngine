use graphic_core::{Window};

use crate::sdl_window_creator::SDLWindowCreator;
use crate::sdl_renderer::SDLRenderer;

pub type SDLWindow = Window<SDLRenderer, SDLWindowCreator>;