use bevy::prelude::*;
use rand::{Rng, SeedableRng, rng};
use rand_chacha::ChaCha8Rng;

#[derive(Resource)]
pub struct SeededRng {
    rng: ChaCha8Rng,
}

impl Default for SeededRng {
    fn default() -> Self {
        let seed = rng().random();
        Self::new(seed)
    }
}

impl SeededRng {
    pub fn new(seed: u64) -> Self {
        SeededRng {
            rng: ChaCha8Rng::seed_from_u64(seed),
        }
    }

    pub fn rng(&mut self) -> &mut ChaCha8Rng {
        &mut self.rng
    }
}
