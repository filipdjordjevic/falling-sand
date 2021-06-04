use piston_window::types::Color;

pub const BLOCK_SIZE: usize = 10;

pub trait Block {
    fn color(&self) -> Color;
    fn is_falling(&self) -> bool;
    fn change_falling_state(&mut self, state: bool);
}
