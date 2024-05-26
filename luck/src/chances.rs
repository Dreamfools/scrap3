use crate::LuckState;
use rand::Rng;

pub fn chances(state: &mut LuckState, weights: &[u64], sum: Option<u64>) -> usize {
    let sum = sum.unwrap_or_else(|| weights.iter().sum());

    let generated = state.state.gen_range(0..sum);

    let mut total = 0;
    for (i, x) in weights.iter().enumerate() {
        total += x;
        if generated < total {
            return i;
        }
    }

    unreachable!("Generated value should always be lower than the sum of weights")
}
