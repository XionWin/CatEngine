use graphic_core::WindowCreator;
use sdl2::pixels::Color;
pub struct SDLWindowCreator {
    sdl: sdl2::Sdl,
    video_subsystem: sdl2::VideoSubsystem,
    canvas: sdl2::render::Canvas<sdl2::video::Window>,
}

impl WindowCreator for SDLWindowCreator {
    fn new() -> Self {
        let sdl = sdl2::init().unwrap();
        let video_subsystem = sdl.video().unwrap();

        let (wnd_width, wnd_height) = (640, 480);
        let window = video_subsystem
            .window("Game", wnd_width, wnd_height)
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

        SDLWindowCreator { sdl, video_subsystem, canvas }
    }
    fn show(&mut self) {
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

            
            self.canvas.set_draw_color(Color::RGB(255, 0, 0));
            self.canvas.clear();
            self.canvas.present();
        }
    }

    fn set_size(&mut self, w: u32, h: u32) {
        let window = self.video_subsystem
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
        self.canvas = canvas;
    }
}
