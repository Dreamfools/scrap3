use crate::{BoardMatch, MatchColor};
use enum_decompose::decompose;

/// Removes matched gems from the board, replacing them with the [empty_gem]
pub fn remove_matched<Gem: crate::BoardGem>(
    board: &mut [Gem],
    matches: &[BoardMatch<Gem::Color>],
    mut empty_gem: impl FnMut() -> Gem,
) {
    for &c in matches.iter().flat_map(|g| g.cells.iter()) {
        board[c] = empty_gem();
    }
}

#[decompose]
#[derive(Debug, Clone)]
pub enum GravityRefillAction {
    Fall {
        from: usize,
        to: usize,
        height: usize,
    },
    FallIn {
        pos: usize,
        height: usize,
    },
}

impl GravityRefillAction {
    pub fn target(&self) -> usize {
        match self {
            GravityRefillAction::Fall(fall) => fall.to,
            GravityRefillAction::FallIn(fall_in) => fall_in.pos,
        }
    }

    pub fn height(&self) -> usize {
        match self {
            GravityRefillAction::Fall(fall) => fall.height,
            GravityRefillAction::FallIn(fall_in) => fall_in.height,
        }
    }

    pub fn apply<Gem: crate::BoardGem>(
        &self,
        board: &mut [Gem],
        mut random_gem: impl FnMut(usize) -> Gem,
    ) {
        match self {
            GravityRefillAction::Fall(fall) => board.swap(fall.from, fall.to),
            GravityRefillAction::FallIn(fall) => board[fall.pos] = random_gem(fall.pos),
        }
    }
}

pub trait RefillableGem: MatchColor {
    fn is_empty(&self) -> bool;
}

pub struct GravityRefill;

impl GravityRefill {
    pub fn refill<Gem: crate::BoardGem, Line: AsRef<[usize]>>(
        board: &[Gem],
        gravity_lines: &[Line],
    ) -> Vec<GravityRefillAction>
    where
        Gem::Color: RefillableGem,
    {
        let mut actions = vec![];
        for line in gravity_lines {
            let line = line.as_ref();
            let mut last_empty = Option::<usize>::None;
            for (i, pos) in line.iter().copied().enumerate().rev() {
                if board[pos].color().is_empty() {
                    if last_empty.is_none() {
                        last_empty = Some(i);
                    }
                } else if let Some(empty) = &mut last_empty {
                    actions.push(GravityRefillAction::Fall(GravityRefillActionFall {
                        from: pos,
                        to: line[*empty],
                        height: empty.abs_diff(i),
                    }));
                    *empty -= 1;
                }
            }

            if let Some(empty) = last_empty {
                let height = empty + 1;

                for &pos in &line[..=empty] {
                    actions.push(GravityRefillAction::FallIn(GravityRefillActionFallIn {
                        pos,
                        height,
                    }))
                }
            }
        }

        actions
    }
}
