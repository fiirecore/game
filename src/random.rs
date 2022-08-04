use rand::prelude::{RngCore, SeedableRng, SmallRng};
use worldcli::worldlib::random::WorldRandoms;

pub type GameWorldRandoms = WorldRandoms<GamePseudoRandom>;

#[derive(Debug, Clone)]
pub struct GamePseudoRandom(SmallRng);

impl GamePseudoRandom {

    pub fn seed(&mut self, seed: u64) {
        *self = Self::seed_from_u64(seed);
    }

}

impl RngCore for GamePseudoRandom {
    fn next_u32(&mut self) -> u32 {
        self.0.next_u32()
    }

    fn next_u64(&mut self) -> u64 {
        self.0.next_u64()
    }

    fn fill_bytes(&mut self, dest: &mut [u8]) {
        self.0.fill_bytes(dest)
    }

    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand::Error> {
        self.0.try_fill_bytes(dest)
    }
}

impl SeedableRng for GamePseudoRandom {
    type Seed = <SmallRng as SeedableRng>::Seed;

    fn from_seed(seed: Self::Seed) -> Self {
        Self(SmallRng::from_seed(seed))
    }
}

impl Default for GamePseudoRandom {
    fn default() -> Self {
        Self(SmallRng::from_seed(Default::default()))
    }
}
