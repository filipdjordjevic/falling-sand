use piston_window::rectangle::rectangle_by_corners;
use piston_window::{rectangle, Context, G2d};

use crate::block_type::BLOCK_SIZE;

pub struct MouseInput {
    hold: bool,
    start: [f64; 2],
    cursor: [f64; 2],
}

impl MouseInput {
    pub fn new() -> Self {
        MouseInput {
            hold: false,
            start: [0.0; 2],
            cursor: [0.0; 2],
        }
    }
    pub fn handle_click(&self) -> (usize, usize) {
        click_to_pos(&self.cursor)
    }

    pub fn set_cursor(&mut self, cursor: [f64; 2]) {
        self.cursor = cursor;
    }

    pub fn draw_select(&self, con: &Context, g: &mut G2d) {
        if self.hold {
            rectangle(
                [1.0, 0.0, 0.0, 0.5],
                rectangle_by_corners(self.start[0], self.start[1], self.cursor[0], self.cursor[1]),
                con.transform,
                g,
            )
        }
    }

    pub fn right_click(&mut self) -> Option<[(usize, usize); 2]> {
        if !self.hold {
            self.start = self.cursor;
            self.hold = !self.hold;
            None
        } else {
            self.hold = !self.hold;
            Some([click_to_pos(&self.start), self.handle_click()])
        }
    }
}

fn click_to_pos(click: &[f64; 2]) -> (usize, usize) {
    let i = click[1] as usize / BLOCK_SIZE;
    let j = click[0] as usize / BLOCK_SIZE;
    (i, j)
}
