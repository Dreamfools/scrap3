use std::ops::Add;
use std::time::{Duration, Instant};

#[derive(Debug)]
pub struct Staggerer {
    /// Stagger delay before activation after signal is received
    pub delay: Duration,
    /// Minimal interval between activations
    pub min_interval: Duration,

    /// Last emitted activation
    earliest_next_activation: Instant,
    /// Last received trigger
    last_trigger: Option<Instant>,
}

impl Staggerer {
    pub fn new(delay: Duration, min_interval: Duration) -> Self {
        Self {
            delay,
            min_interval,
            earliest_next_activation: Instant::now(),
            last_trigger: None,
        }
    }

    /// Emits trigger to staggerer
    pub fn trigger(&mut self) {
        self.last_trigger = Some(Instant::now());
    }

    /// Checks if staggerer is emitting activation signal, consuming it if true
    pub fn activated(&mut self) -> bool {
        if Instant::now() < self.earliest_next_activation {
            return false;
        }

        let Some(last_trigger) = &self.last_trigger else {
            return false;
        };

        return if Instant::now() > last_trigger.add(self.delay) {
            self.last_trigger = None;
            self.earliest_next_activation = Instant::now() + self.min_interval;
            true
        } else {
            false
        };
    }
}
