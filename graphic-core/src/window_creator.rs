pub trait WindowCreator {
    fn new(w: u32, h: u32) -> Self;
    fn show(&mut self);

    fn set_size(self, w: u32, h: u32) -> Self;
}