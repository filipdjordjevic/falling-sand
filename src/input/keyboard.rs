use piston_window::{Button, Key};

use crate::block_type::BlockType;

pub enum InputState {
    Insert(BlockType),
    Remove,
}

pub struct KeyboardInput {}

impl KeyboardInput {
    pub fn handle_press(&self, press_args: Option<Button>) -> Option<InputState> {
        match press_args {
            Some(Button::Keyboard(Key::Delete)) => Some(InputState::Remove),
            Some(Button::Keyboard(Key::S)) => Some(InputState::Insert(BlockType::Sand)),
            Some(_) => None,
            None => None,
        }
    }
}
