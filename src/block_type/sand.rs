use piston_window::types::Color;

use crate::block_type::Block;

pub struct Sand {
    color: Color,
    falling: bool,
}

impl Sand {
    pub fn new() -> Self {
        Sand {
            color: [1.0, 1.00, 0.00, 1.0],
            falling: true,
        }
    }
}

impl Block for Sand {
    fn color(&self) -> Color {
        self.color
    }

    fn is_falling(&self) -> bool {
        self.falling
    }

    fn change_falling_state(&mut self, state: bool) {
        self.falling = state;
    }
}
