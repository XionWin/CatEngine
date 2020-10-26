#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum WindowPos {
    Undefined,
    Centered,
    Positioned(i32)
}

pub trait WindowCreator {
    fn new(w: u32, h: u32) -> Self;
    fn show(&mut self);

    fn set_size(self, w: u32, h: u32) -> Self;
    fn set_position(self, x: WindowPos, y: WindowPos) -> Self;
}