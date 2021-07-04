use serde::Serialize;

use deps::borrow::{BorrowableMut, Identifiable};

use crate::{
    item::ItemRef,
    moves::{instance::*, MoveCategory, MoveRef},
    pokemon::{
        stat::{BaseStats, Stats},
        Experience, Friendship, Gender, Health, Level, Pokemon, PokemonId, PokemonRef,
    },
    status::StatusEffectInstance,
    types::{Effective, PokemonType},
};

mod deserialize;

mod exp;
mod item;
mod moves;

// pub mod instance_template;

pub type Nickname = Option<String>;

#[derive(Clone, Serialize)]
pub struct PokemonInstance {
    #[serde(rename = "id")]
    pub pokemon: PokemonRef,

    #[serde(default)]
    pub nickname: Nickname,
    pub level: Level,
    pub gender: Gender,

    #[serde(default = "default_iv")]
    pub ivs: Stats,
    #[serde(default)]
    pub evs: Stats,

    #[serde(default)]
    pub experience: Experience,

    #[serde(default = "default_friendship")]
    pub friendship: Friendship,

    pub moves: MoveInstanceSet,

    #[serde(default)]
    pub effect: Option<StatusEffectInstance>,

    #[serde(default)]
    pub item: Option<ItemRef>,

    #[serde(skip)]
    pub base: BaseStats,

    pub current_hp: Health,
}

pub type BorrowedPokemon = BorrowableMut<'static, PokemonInstance>;

impl PokemonInstance {
    pub fn generate(id: PokemonId, min: Level, max: Level, ivs: Option<Stats>) -> Self {
        let pokemon = Pokemon::get(&id);

        let level = if min == max {
            max
        } else {
            crate::RANDOM.gen_range(min, max + 1)
        };

        let ivs = ivs.unwrap_or_else(Stats::random);
        let evs = Stats::default();

        let base = BaseStats::new(&pokemon, &ivs, &evs, level);

        Self {
            nickname: None,
            level,
            gender: pokemon.generate_gender(),

            ivs,
            evs,

            experience: 0,
            friendship: 70,

            moves: pokemon.generate_moves(level),

            item: None,

            effect: None,

            current_hp: base.hp(),

            base,

            pokemon,
        }
    }

    pub fn generate_with_level(id: PokemonId, level: Level, ivs: Option<Stats>) -> Self {
        Self::generate(id, level, level, ivs)
    }

    pub fn name(&self) -> &str {
        self.nickname.as_ref().unwrap_or(&self.pokemon.name)
    }

    pub fn fainted(&self) -> bool {
        self.current_hp == 0
    }

    pub fn hp(&self) -> Health {
        self.current_hp
    }

    pub fn percent_hp(&self) -> f32 {
        self.current_hp as f32 / self.max_hp() as f32
    }

    pub fn can_afflict_status(&self) -> bool {
        self.effect.is_none()
    }

    pub fn max_hp(&self) -> Health {
        self.base.hp()
    }

    pub fn heal(&mut self) {
        self.heal_hp();
        self.heal_pp();
    }

    pub fn heal_hp(&mut self) {
        self.current_hp = self.max_hp();
    }

    pub fn heal_pp(&mut self) {
        self.moves.iter_mut().for_each(MoveInstance::restore)
    }

    pub fn moves_at_level(&self) -> Vec<MoveRef> {
        self.pokemon.moves_at_level(self.level)
    }

    pub fn effective(&self, pokemon_type: PokemonType, category: MoveCategory) -> Effective {
        let pokemon = self.pokemon;
        let primary = pokemon_type.effective(pokemon.primary_type, category);
        if let Some(secondary) = pokemon.secondary_type {
            primary * pokemon_type.effective(secondary, category)
        } else {
            primary
        }
    }
}

impl core::fmt::Debug for PokemonInstance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        core::fmt::Display::fmt(&self, f)
    }
}

impl core::fmt::Display for PokemonInstance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Lv. {} {}", self.level, self.name())
    }
}
