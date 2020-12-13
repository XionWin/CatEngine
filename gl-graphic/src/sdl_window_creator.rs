use crate::SDLRenderer;
use graphic_core::{WindowCreator};

pub struct SDLWindowCreator {
    sdl: sdl2::Sdl,
    renderer: SDLRenderer,
}

impl WindowCreator<SDLRenderer> for SDLWindowCreator {
    fn new(w: u32, h: u32) -> Self {
        let sdl = sdl2::init().unwrap();
        let video_subsystem = sdl.video().unwrap();

        let window = video_subsystem
            .window("Game", w, h)
            .opengl()
            .resizable()
            .build()
            .unwrap();

        let canvas = window
            .into_canvas()
            .present_vsync()
            .build()
            .map_err(|e| e.to_string())
            .unwrap();

        SDLWindowCreator {
            sdl,
             renderer: SDLRenderer{canvas},
        }
    }

    fn show(&mut self, f: &dyn Fn(&mut SDLRenderer)) {
        let mut event_pump = self.sdl.event_pump().unwrap();
        'main: loop {
            for event in event_pump.poll_iter() {
                match event {
                    sdl2::event::Event::Quit { .. } => break 'main,
                    sdl2::event::Event::KeyDown {
                        keycode: Some(sdl2::keyboard::Keycode::Q),
                        ..
                    } => break 'main,
                    sdl2::event::Event::Window {
                        win_event: sdl2::event::WindowEvent::Resized(..),
                        ..
                    } => {}
                    _ => {}
                }
            }

            f(&mut self.renderer);

            self.renderer.canvas.clear();
            self.renderer.canvas.present();
        }
    }

    fn set_size(mut self, w: u32, h: u32) -> Self {
        let mut window = self.renderer.canvas.into_window();
        window.set_size(w, h).unwrap();
        window.set_position(sdl2::video::WindowPos::Centered, sdl2::video::WindowPos::Centered);
        
        self.renderer.canvas = window
            .into_canvas()
            .present_vsync()
            .build()
            .map_err(|e| e.to_string())
            .unwrap();
        self
    }

    fn set_position(mut self, x: graphic_core::WindowPos, y: graphic_core::WindowPos) -> Self {
        let mut window = self.renderer.canvas.into_window();

        let pos_x = match x {
            graphic_core::WindowPos::Undefined => sdl2::video::WindowPos::Undefined,
            graphic_core::WindowPos::Centered => sdl2::video::WindowPos::Centered,
            graphic_core::WindowPos::Positioned(v) => sdl2::video::WindowPos::Positioned(v),
        };
        let pos_y = match y {
            graphic_core::WindowPos::Undefined => sdl2::video::WindowPos::Undefined,
            graphic_core::WindowPos::Centered => sdl2::video::WindowPos::Centered,
            graphic_core::WindowPos::Positioned(v) => sdl2::video::WindowPos::Positioned(v),
        };

        window.set_position(pos_x, pos_y);
        
        self.renderer.canvas = window
            .into_canvas()
            .present_vsync()
            .build()
            .map_err(|e| e.to_string())
            .unwrap();
        self
    }

    fn get_renderer(&mut self) -> &mut SDLRenderer { 
        &mut self.renderer
     }


    fn set_update_handler(&mut self, f: &dyn Fn(&mut SDLRenderer)) {

    }
}
