use piston_window::{Button, Context, Event, G2d, MouseButton, MouseCursorEvent, PressEvent};

use keyboard::{InputState, KeyboardInput};
use mouse::MouseInput;

use crate::board::Board;

mod keyboard;
mod mouse;

pub struct InputHandler {
    mouse: MouseInput,
    keyboard: KeyboardInput,
    state: InputState,
}

impl InputHandler {
    pub fn new() -> Self {
        InputHandler {
            mouse: MouseInput::new(),
            keyboard: KeyboardInput {},
            state: InputState::Remove,
        }
    }

    pub fn handle_events(&mut self, event: &Event, board: &mut Board) {
        if let Some(x) = self.keyboard.handle_press(event.press_args()) {
            self.state = x;
        }

        if let Some(pos) = event.mouse_cursor_args() {
            self.mouse.set_cursor(pos);
        }

        if let Some(Button::Mouse(MouseButton::Left)) = event.press_args() {
            board.insert(&self.mouse.handle_click());
        }
        if let Some(Button::Mouse(MouseButton::Right)) = event.press_args() {
            if let Some(pos) = self.mouse.right_click() {
                match &self.state {
                    InputState::Insert(x) => board.insert_rect(x, &pos),
                    InputState::Remove => board.remove_rect(&pos),
                }
            }
        }
    }

    pub fn draw(&self, c: &Context, g: &mut G2d) {
        self.mouse.draw_select(c, g);
    }
}
