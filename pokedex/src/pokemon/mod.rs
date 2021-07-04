use std::ops::Range;

use crate::{
    moves::{
        instance::{MoveInstance, MoveInstanceSet},
        Move, MoveId, MoveRef,
    },
    pokemon::{
        data::{Breeding, Gender, LearnableMove, PokedexData, Training},
        stat::Stats,
    },
    types::PokemonType,
    Dex,
};
use deps::{
    borrow::{Identifiable, StaticRef},
    hash::HashMap,
};
use serde::{Deserialize, Serialize};

pub mod data;
pub mod instance;
pub mod party;
pub mod stat;
pub mod types;

pub struct Pokedex;

static mut POKEDEX: Option<HashMap<PokemonId, Pokemon>> = None;

impl Dex<'static> for Pokedex {
    type DexType = Pokemon;

    fn dex() -> &'static mut Option<
        HashMap<<<Self as Dex<'static>>::DexType as Identifiable<'static>>::Id, Self::DexType>,
    > {
        unsafe { &mut POKEDEX }
    }
}

pub type PokemonId = u16;
pub type Level = u8;
pub type Experience = u32;
pub type Friendship = u8;
pub type Health = stat::BaseStat;

#[derive(Serialize, Deserialize)]
pub struct Pokemon {
    pub id: PokemonId,
    pub name: String,

    pub primary_type: PokemonType,
    pub secondary_type: Option<PokemonType>,

    pub base: Stats,

    pub data: PokedexData,

    pub training: Training,
    pub breeding: Breeding,

    pub moves: Vec<LearnableMove>,
}

impl Pokemon {
    pub fn generate_moves(&self, level: Level) -> MoveInstanceSet {
        let mut moves = self
            .moves
            .iter()
            .filter(|learnable_move| learnable_move.level <= level)
            .map(|learnable_move| learnable_move.id)
            .collect::<Vec<MoveId>>();
        moves.dedup();
        moves.reverse();
        moves.truncate(4);
        moves
            .into_iter()
            .map(|id| Move::get(&id))
            .map(MoveInstance::new)
            .collect()
    }

    pub fn generate_gender(&self) -> Gender {
        match self.breeding.gender {
            Some(percentage) => match crate::RANDOM.gen_range(0, 8) > percentage {
                true => Gender::Male,
                false => Gender::Female,
            },
            None => Gender::None,
        }
    }

    pub fn exp_from(&self, level: Level) -> Experience {
        ((self.training.base_exp * level as u16) / 7) as Experience
    }

    pub fn moves_at_level(&self, level: Level) -> Vec<MoveRef> {
        self.moves
            .iter()
            .filter(|m| m.level == level)
            .map(|m| Move::get(&m.id))
            .collect()
    }

    pub fn moves_at(&self, levels: Range<Level>) -> Vec<MoveRef> {
        let levels = Range {
            start: levels.start + 1,
            end: levels.end + 1,
        };

        let mut moves = Vec::new();

        levels.for_each(|level| moves.extend(self.moves_at_level(level)));

        moves
    }
}

impl<'a> Identifiable<'a> for Pokemon {
    type Id = PokemonId;

    const UNKNOWN: PokemonId = 0;

    fn id(&self) -> &Self::Id {
        &self.id
    }

    fn try_get(id: &Self::Id) -> Option<&'a Self>
    where
        Self: Sized,
    {
        Pokedex::try_get(id)
    }
}

pub type PokemonRef = StaticRef<Pokemon>;

pub fn default_iv() -> Stats {
    Stats::uniform(15)
}

pub const fn default_friendship() -> Friendship {
    70
}

impl core::fmt::Debug for Pokemon {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        core::fmt::Display::fmt(&self, f)
    }
}

impl core::fmt::Display for Pokemon {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({})", self.name, self.id)
    }
}
