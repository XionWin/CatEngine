use super::window::*;

pub trait WindowCreator {
    fn create() -> Window;
}