use crate::Renderer;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum WindowPos {
    Undefined,
    Centered,
    Positioned(i32)
}

pub trait WindowCreator<R: Renderer> {
    fn new(w: u32, h: u32, t: &str) -> Self;
    fn show(&mut self, f: &dyn Fn(&mut R));

    fn set_size(self, w: u32, h: u32) -> Self;
    fn set_position(self, x: WindowPos, y: WindowPos) -> Self;

    fn get_renderer(&mut self) -> &mut R;
}