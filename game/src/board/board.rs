use crate::board::gem::{Gem, GEM_COLORS};
use match3::rect_board::RectBoard;
use match3::SimpleGem;
use tinyrand::RandRange;
use tinyrand_std::thread_rand;

pub type GemBoard = RectBoard<Gem>;
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum BoardState {
    Idle,
    Moving,
    Refilling,
    Matching,
}

pub fn random_board(width: usize, height: usize) -> (RectBoard<Gem>, BoardState) {
    let mut rand = thread_rand();
    let mut board = Vec::with_capacity(width * height);
    for _ in 0..width * height {
        let i = rand.next_range(0..GEM_COLORS.len());
        board.push(SimpleGem(GEM_COLORS[i]));
    }
    let board = GemBoard::new(width, height, board);
    (board, BoardState::Idle)
}
