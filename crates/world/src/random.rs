use rand::{Rng, SeedableRng};

#[derive(Debug, Default, Clone)]
pub struct WorldRandoms<R: Rng> {
    pub general: R,
    pub wild: R,
    pub npc: R,
}

impl<R: Rng + Default + Clone> WorldRandoms<R> {
    pub fn new() -> Self {
        Self::from(R::default())
    }
}

impl<R: Rng + SeedableRng + Clone> WorldRandoms<R> {
    pub fn new_zero_seed() -> Self {
        Self::from(R::seed_from_u64(0))
    }

    pub fn seed(&mut self, seed: u64) {
        let rng = R::seed_from_u64(seed);
        self.general = rng.clone();
        self.wild = rng.clone();
        self.npc = rng;
    }
}

impl<R: Rng + Clone> From<R> for WorldRandoms<R> {
    fn from(rand: R) -> Self {
        Self {
            general: rand.clone(),
            wild: rand.clone(),
            npc: rand,
        }
    }
}
