use super::common::*;
use super::window_creator::*;

pub struct Window<T: WindowCreator> {
    creator: T,
    bound: Rect,
    title: String,
}
impl<T> Window<T>
where
    T: WindowCreator,
{
    pub fn new(x: i32, y: i32, w: u32, h: u32, title: &str, creator: T) -> Self {
        let creator = creator.set_size(w, h);
        Window {
            creator,
            bound: Rect::new(x, y, w as i32, h as i32),
            title: String::from(title),
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
    pub fn show(&mut self) {
        self.creator.show();
    }

    pub fn set_size(self, w: u32, h: u32) {
        self.creator.set_size(w, h);
    }
}
