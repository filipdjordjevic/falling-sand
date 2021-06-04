use piston_window::{rectangle, Context, G2d};

use crate::block_type::{Block, BlockType, Sand, BLOCK_SIZE};

pub const BOARD_WIDTH: usize = 30;
pub const BOARD_HEIGHT: usize = 30;
pub const UPDATE_TIME: f64 = 0.1;

pub struct Board {
    items: Vec<Vec<Option<Box<dyn Block>>>>,
    time: f64,
}

impl Board {
    pub fn new() -> Self {
        Board {
            items: (0..BOARD_HEIGHT)
                .map(|_| (0..BOARD_HEIGHT).map(|_| None).collect())
                .collect(),
            time: 0.0,
        }
    }

    pub fn iter(&mut self, dt: f64) {
        self.time += dt;

        if self.time < UPDATE_TIME {
            return;
        }
        for i in (0..self.items.len()).rev() {
            for j in 0..self.items[0].len() {
                let under = i + 1 >= BOARD_HEIGHT || self.items[i + 1][j].is_some();
                match &mut self.items[i][j] {
                    Some(_) => {
                        if under {
                            self.time = 0.0;
                            continue;
                        }

                        let temp = std::mem::replace(&mut self.items[i][j], None);
                        self.items[i][j] = std::mem::replace(&mut self.items[i + 1][j], temp);
                        self.time = 0.0;
                    }
                    None => (),
                }
            }
        }
        self.time = 0.0;
    }

    pub fn draw(&self, con: &Context, g: &mut G2d) {
        for (i, row) in self.items.iter().enumerate() {
            for (j, col) in row.iter().enumerate() {
                match col {
                    Some(x) => rectangle(
                        x.color(),
                        [to_pos(j), to_pos(i), BLOCK_SIZE as f64, BLOCK_SIZE as f64],
                        con.transform,
                        g,
                    ),
                    None => (),
                }
            }
        }
    }

    pub fn insert(&mut self, pos: &(usize, usize)) {
        self.items[pos.0][pos.1] = Some(Box::new(Sand::new()));
    }

    pub fn insert_rect(&mut self, block_type: &BlockType, pos: &[(usize, usize); 2]) {
        for i in pos[0].0..=pos[1].0 {
            for j in pos[0].1..=pos[1].1 {
                self.items[i][j] = Some(Box::new(match block_type {
                    BlockType::Sand => Sand::new(),
                }));
            }
        }
    }

    pub fn remove_rect(&mut self, pos: &[(usize, usize); 2]) {
        for i in pos[0].0..pos[1].0 {
            for j in pos[0].1..pos[1].1 {
                self.items[i][j] = None;
            }
        }
    }
}

fn to_pos(i: usize) -> f64 {
    (i * BLOCK_SIZE) as f64
}
