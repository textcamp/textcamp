use rand::prelude::*;

#[derive(Debug, Clone, Default)]
pub struct Dice {
    rng: rand::rngs::ThreadRng,
}

impl Dice {
    pub fn new() -> Self {
        Self {
            rng: rand::thread_rng(),
        }
    }

    pub fn range<T: rand::distributions::uniform::SampleUniform>(&mut self, low: T, high: T) -> T {
        self.rng.gen_range(low, high)
    }
}
