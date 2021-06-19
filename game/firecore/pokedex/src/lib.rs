extern crate firecore_dependencies as deps;

use deps::random::{Random, RandomState, GLOBAL_STATE};

pub mod pokemon;
pub mod moves;
pub mod item;

pub mod types;

pub mod trainer;

pub(crate) static RANDOM: Random = Random::new(RandomState::Static(&GLOBAL_STATE));

// #[deprecated(note = "todo: logging")]