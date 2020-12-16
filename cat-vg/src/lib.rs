extern crate gl_graphic;
extern crate graphic_core;

use gl_graphic::{SDLWindow};    
use graphic_core::Renderer;

use std::{sync::mpsc, thread};
use std::time::Duration;

mod color;

pub fn test() {
    let mut window = SDLWindow::new(0, 0, 320, 320, "Game1");
    
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let mut angle = 0u16;
        loop {
            tx.send(angle).ok();
            angle = (angle + 2u16) % 360u16;
            thread::sleep(Duration::from_millis(1));
        }
    });

    window.show(&|renderer| {
        // let mut value = 0u16;
        // let i = loop {
        //     match rx.try_recv() {
        //         Ok(idx) => {value = idx},
        //         Err(_) => break value
        //     }
        // };


        let i = rx.recv().unwrap();
        println!("{}", i);
        let (r, g, b) = color::hsl2rgb(i as f32, 0.5f32, 0.5f32);
        renderer.clear(r, g, b);
    });
}