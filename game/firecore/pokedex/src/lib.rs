extern crate firecore_dependencies as deps;

pub mod pokemon;
pub mod moves;
pub mod item;

pub mod types;

pub(crate) static RANDOM: deps::Random = deps::Random::new();

pub fn seed_random(seed: u64) {
    RANDOM.seed(seed);
}

// #[deprecated(note = "todo: logging")]