use crate::chances::chances;
use crate::LuckState;

#[derive(Debug, Clone)]
pub struct RandomPool<Item: Clone> {
    choices: Vec<Item>,
    weights: Vec<u64>,
}

impl<Item: Clone> RandomPool<Item> {
    pub fn new(choices: Vec<Item>, weights: Vec<u64>) -> Self {
        Self { choices, weights }
    }
}

impl<Item: Clone> RandomPool<Item> {
    pub fn from_weights<Weight: PoolWeight>(
        choices: impl IntoIterator<Item = (Weight, Item)>,
    ) -> Self {
        let (weights, items) = choices
            .into_iter()
            .map(|(w, i)| (w.into_weight(), i))
            .unzip();
        Self::new(items, weights)
    }

    pub fn get(&self, state: &mut LuckState) -> &Item {
        &self.choices[chances(state, &self.weights, None)]
    }
}

pub trait PoolWeight {
    fn into_weight(self) -> u64;
}

impl PoolWeight for u64 {
    fn into_weight(self) -> u64 {
        self
    }
}

impl PoolWeight for f64 {
    fn into_weight(self) -> u64 {
        // Cut off at a sane point
        debug_assert!(self < 1e9, "Weights are too high");
        // Good enough
        (self * 1e6).round() as u64
    }
}
