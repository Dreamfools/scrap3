use crate::state::combat::board::Board;
use crate::state::combat::gem::GemColor;
use match3::BoardMatch;

pub mod gem;

pub mod board;

#[derive(Debug)]
pub struct CombatState {
    pub board: Board,

    pub inputs: Vec<CombatInput>,
    pub pending_inputs: Vec<CombatInput>,

    pub stack: Vec<CombatAction>,
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
