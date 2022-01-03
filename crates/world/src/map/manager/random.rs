use rand::{Rng, SeedableRng};

pub struct WorldRandoms<R: Rng + SeedableRng + Clone> {
    pub general: R,
    pub wild: R,
    pub npc: R,
}

impl<R: Rng + SeedableRng + Clone> Default for WorldRandoms<R> {
    fn default() -> Self {
        let rng = R::seed_from_u64(0);
        Self {
            general: rng.clone(),
            wild: rng.clone(),
            npc: rng,
        }
    }
}

impl<R: Rng + SeedableRng + Clone> WorldRandoms<R> {
    pub fn seed(&mut self, seed: u64) {
        let rng = R::seed_from_u64(seed);
        self.general = rng.clone();
        self.wild = rng.clone();
        self.npc = rng;
    }
}
