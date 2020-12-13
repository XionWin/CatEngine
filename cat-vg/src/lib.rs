extern crate gl_graphic;
extern crate graphic_core;

use core::sync::atomic::{AtomicU8, Ordering};
use gl_graphic::{SDLWindow};
use graphic_core::Renderer;

use std::thread;
use std::time::Duration;
use std::sync::Arc;


pub fn test() {
    let mut window = SDLWindow::new(0, 0, 600, 600, "Game");
    
let counter = Arc::new(AtomicU8::new(0u8));
    let ref_rbg = counter.clone();
    thread::spawn(move || {
        // some work here
        loop {
            counter.fetch_add(1, Ordering::SeqCst);
            thread::sleep(Duration::from_millis(1));
        }
    });

    window.show(&|renderer| {
        let i = (*ref_rbg).load(Ordering::SeqCst);
        renderer.clear(i, 255u8, 255u8);
    });
}
