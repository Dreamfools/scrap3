use crate::board::gem::{Gem, GemColor, GEM_COLORS};
use enum_decompose::decompose;
use match3::rect_board::RectBoard;
use match3::{BoardMatch, SimpleGem};
use strum::EnumIs;
use tinyrand::RandRange;
use tinyrand_std::thread_rand;

pub type GemBoard = RectBoard<Gem>;
#[decompose(prefix = "", suffix = "State", derive = "Debug")]
#[derive(Debug, EnumIs)]
pub enum BoardState {
    Idle,
    Moving {
        spin_end: f64,
    },
    Refilling,
    Matching {
        groups: Vec<BoardMatch<GemColor>>,
        end: f64,
    },
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
