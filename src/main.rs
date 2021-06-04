mod block_type;
mod board;
mod input;

use piston_window::types::Color;
use piston_window::*;

use block_type::BLOCK_SIZE;
use board::{Board, BOARD_HEIGHT, BOARD_WIDTH};
use input::InputHandler;

const BACK_COLOR: Color = [0.5, 0.5, 0.5, 1.0];

fn main() {
    let mut board: Board = Board::new();
    let mut input: InputHandler = InputHandler::new();
    let mut window: PistonWindow = WindowSettings::new(
        "Falling Sand",
        [
            (BOARD_WIDTH * BLOCK_SIZE) as u32,
            (BOARD_HEIGHT * BLOCK_SIZE) as u32,
        ],
    )
    .exit_on_esc(true)
    .resizable(false)
    .vsync(true)
    .build()
    .unwrap();

    while let Some(event) = window.next() {
        input.handle_events(&event, &mut board);

        window.draw_2d(&event, |c, g, _| {
            clear(BACK_COLOR, g);
            board.draw(&c, g);
            input.draw(&c, g);
        });

        event.update(|arg| {
            board.iter(arg.dt);
        });
    }
}
