use crate::state::combat::gem::{Gem, GemColor};
use enum_decompose::decompose;
use match3::rect_board::RectBoard;
use match3::BoardMatch;
use strum::EnumIs;

pub type GemBoard = RectBoard<Gem>;
#[decompose(prefix = "", suffix = "State", derive = "Debug")]
#[derive(Debug, EnumIs)]
pub enum BoardState {
    Idle,
    Moving {
        spin_end: f64,
        total_time: f64,
    },
    Refilling {
        end: f64,
    },
    Matching {
        groups: Vec<BoardMatch<GemColor>>,
        end: f64,
    },
}

#[derive(Debug)]
pub struct Board {
    pub board: GemBoard,
    pub state: BoardState,
}
