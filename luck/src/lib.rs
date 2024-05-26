//! Random for use in games, supporting various of "luck" alterations
use rand::Rng;
use rand_pcg::Pcg32;

pub mod chances;
pub mod pool;

#[derive(Debug)]
pub struct LuckState {
    state: Pcg32,
}
