pub use self::block::{Block, BLOCK_SIZE};
pub use self::sand::Sand;

mod block;
mod sand;

pub enum BlockType {
    Sand,
}
