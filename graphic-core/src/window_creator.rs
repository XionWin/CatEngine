pub trait WindowCreator {
    fn new() -> Self;
    fn show(&mut self);

    fn set_size(&mut self, w: u32, h: u32);
}