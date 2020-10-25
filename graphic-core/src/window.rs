use super::common::*;

pub struct Window {
    bound: Rect,
    title: String,
}
impl Window {
    pub fn new(x: i32, y: i32, w: i32, h: i32, title: &str) -> Self {
        Window {
            bound: Rect::new(x, y, w, h),
            title: String::from(title),
        }
    }
    pub fn bound(&self) -> Rect {
        self.bound
    }
    pub fn title(&self) -> String {
        self.title.clone()
    }
}
