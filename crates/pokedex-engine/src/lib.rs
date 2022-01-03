pub extern crate firecore_engine as engine;
pub use firecore_pokedex_engine_builder::pokedex;
// pub use battle::pokedex;
pub use pokedex::*;

// #[deprecated(note = "add battle moves to battle-gui crate")]
// pub mod battle_move;

pub(crate) mod data;
pub mod gui;
pub mod texture;

/// Holds the string "cry"
pub const CRY_ID: tinystr::TinyStr8 = unsafe { tinystr::TinyStr8::new_unchecked(7959107) };

pub use data::PokedexClientData;
pub use firecore_pokedex_engine_builder::{npc_group::NpcGroupId, SerializedPokedexEngine};

mod get {
    use std::ops::Deref;

    use crate::pokedex::pokemon::{
        owned::OwnablePokemon,
        stat::{Stat, StatSet},
        Health, Level, Nature, Pokemon, PokemonId,
    };

    pub trait GetPokemonData {
        fn pokemon_id(&self) -> &PokemonId;

        fn name(&self) -> Option<&str>;

        fn level(&self) -> Level;

        fn ivs(&self) -> &StatSet<Stat>;

        fn evs(&self) -> &StatSet<Stat>;

        fn hp(&self) -> Option<Health>;

        fn nature(&self) -> Option<Nature>;
    }

    impl<M, I, G> GetPokemonData
        for OwnablePokemon<PokemonId, M, I, G, Option<Nature>, Option<Health>>
    {
        fn pokemon_id(&self) -> &PokemonId {
            &self.pokemon
        }

        fn name(&self) -> Option<&str> {
            self.nickname.as_deref()
        }

        fn level(&self) -> Level {
            self.level
        }

        fn ivs(&self) -> &StatSet<Stat> {
            &self.ivs
        }

        fn evs(&self) -> &StatSet<Stat> {
            &self.evs
        }

        fn hp(&self) -> Option<Health> {
            self.hp
        }

        fn nature(&self) -> Option<Nature> {
            self.nature
        }
    }

    impl<P: Deref<Target = Pokemon>, M, I, G> GetPokemonData
        for OwnablePokemon<P, M, I, G, Nature, Health>
    {
        fn pokemon_id(&self) -> &PokemonId {
            &self.pokemon.id
        }

        fn name(&self) -> Option<&str> {
            Some(OwnablePokemon::<P, M, I, G, Nature, Health>::name(self))
        }

        fn level(&self) -> Level {
            self.level
        }

        fn ivs(&self) -> &StatSet<Stat> {
            &self.ivs
        }

        fn evs(&self) -> &StatSet<Stat> {
            &self.evs
        }

        fn hp(&self) -> Option<Health> {
            Some(self.hp)
        }

        fn nature(&self) -> Option<Nature> {
            Some(self.nature)
        }
    }
}
