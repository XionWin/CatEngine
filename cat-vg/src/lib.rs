extern crate gl_graphic;
extern crate graphic_core;

use gl_graphic::{SDLWindow};
use graphic_core::Renderer;

use std::{sync::mpsc, thread};
use std::time::Duration;

pub fn test() {
    let mut window = SDLWindow::new(0, 0, 600, 600, "Game1");
    
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let mut idx = 0u8;
        loop {
            tx.send(idx).ok();
            idx = (idx + 1u8) % 255u8;
            thread::sleep(Duration::from_millis(1));
        }
    });

    window.show(&|renderer| {
        let mut value = 0u8;
        let i = loop {
            match rx.try_recv() {
                Ok(idx) => {value = idx},
                Err(_) => break value
            }
        };
       
        if i > 0u8 {
            println!("{}", i);
            renderer.clear(i, i, i);
        }
    });
}