use crate::state::combat::anim::GemAnimation;
use crate::state::combat::board::{Board, BoardState};
use crate::state::combat::gem::{Gem, GemColor};
use luck::pool::RandomPool;
use luck::LuckState;
use match3::rect_board::RectBoard;
use match3::{BoardMatch, SimpleGem};
use math::board::CellIndex;
use model::{GemColorId, Registry};

pub mod gem;

pub mod anim;
pub mod board;

#[derive(Debug)]
pub struct CombatState {
    pub board: Board,

    pub animations: Vec<GemAnimation>,
    /// Extra animations for gems that are not on the board (like the held gem)
    pub extra_animations: Vec<(CellIndex, Gem, GemAnimation)>,

    pub inputs: Vec<CombatInput>,
    pub pending_inputs: Vec<CombatInput>,

    pub stack: Vec<CombatAction>,

    pub luck: LuckState,
}

impl CombatState {
    pub fn new(registry: &Registry) -> Self {
        let mut luck = LuckState::new(9870);
        let mut board = Board {
            board: RectBoard::from_fn(6, 5, |_| SimpleGem(GemColor::Empty)),
            state: BoardState::Idle,
        };
        let colors: Vec<GemColorId> = registry.gem_color.ids().collect();

        let pool = RandomPool::equal(colors);

        for gem in board.board.board.iter_mut() {
            gem.0 = GemColor::Color(*pool.get(&mut luck))
        }

        Self {
            animations: board
                .board
                .board
                .iter()
                .map(|_| GemAnimation::default())
                .collect(),
            board,
            extra_animations: vec![],
            inputs: vec![],
            pending_inputs: vec![],
            stack: vec![],
            luck,
        }
    }
}

#[enum_decompose::decompose]
#[derive(Debug, Clone)]
pub enum CombatInput {}

#[derive(Debug, Clone)]
pub struct CombatAction {
    /// List of effects of this action
    pub effects: Vec<CombatActionEffect>,
    /// Whenever this action can be canceled
    pub cancelable: bool,
}

#[derive(Debug, Clone)]
pub enum CombatActionEffect {
    // Board effects
    /// One gem was matched
    MatchGem { pos: usize },
    /// Group of gems was matched
    MatchGroup { group: BoardMatch<GemColor> },
    /// New gem appears in an empty spot
    GemAppears { pos: usize },

    // Action-related effects
    /// Pushes a new action to the stack
    PushAction { action: CombatAction },
    /// Counters the action directly below this on the stack
    Counter,
}
