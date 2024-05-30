//! Random for use in games, supporting various of "luck" alterations
use rand::{Rng, SeedableRng};
use rand_pcg::Pcg32;

pub mod chances;
pub mod pool;

#[derive(Debug)]
pub struct LuckState {
    state: Pcg32,
}

impl LuckState {
    pub fn new(seed: u64) -> Self {
        Self {
            state: Pcg32::seed_from_u64(seed),
        }
    }
}
