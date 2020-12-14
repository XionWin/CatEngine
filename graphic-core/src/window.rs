use core::marker::PhantomData;
use super::common::*;
use super::window_creator::*;
use super::renderer::*;

pub struct Window<R: Renderer, T: WindowCreator<R>> {
    _marker: PhantomData<R>,
    creator: T,
    bound: Rect,
    title: String,
}

impl<R: Renderer, T: WindowCreator<R>> Window<R, T>
{
    pub fn new(x: i32, y: i32, w: u32, h: u32, t: &str) -> Self {
        let creator = T::new(w, h, t);
        Window {
            _marker: PhantomData,
            creator,
            bound: Rect::new(x, y, w as i32, h as i32),
            title: String::from(t),
        }
    }
    pub fn creator(&mut self) -> &mut T {
        &mut self.creator
    }
    pub fn bound(&self) -> Rect {
        self.bound
    }
    pub fn title(&self) -> String {
        self.title.clone()
    }
    pub fn show(&mut self, f: &dyn Fn(&mut R)) {
        self.creator.show(f);
    }

    pub fn set_size(self, w: u32, h: u32) {
        self.creator.set_size(w, h);
    }

    pub fn get_renderer(&mut self) -> &mut R {
        self.creator().get_renderer()
    }


}
