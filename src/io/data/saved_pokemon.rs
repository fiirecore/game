use oorandom::Rand32;
use serde_derive::{Deserialize, Serialize};

use crate::game::pokedex::pokedex::Pokedex;
use crate::game::pokedex::pokemon::pokemon_instance::PokemonInstance;
use crate::game::pokedex::pokemon::pokemon_instance::calculate_hp;
use crate::game::pokedex::pokemon::pokemon_instance::get_stats;
use crate::game::pokedex::pokemon::pokemon_owned::OwnedPokemon;
use crate::game::pokedex::pokemon::stat_set::StatSet;

use super::saved_pokemon_move::SavedPokemonMoveSet;

#[derive(Debug, Hash, Clone, Serialize, Deserialize)]
pub struct SavedPokemon {

	pub pokemon_id: usize,
    pub level: u8,
    
	pub ivs: Option<StatSet>,
    pub evs: Option<StatSet>,
    
    pub moves: Option<SavedPokemonMoveSet>,

    pub current_hp: Option<u16>,
    
	pub exp: Option<usize>,
	pub friendship: Option<u8>,

}

impl SavedPokemon {

	pub fn generate(random: &mut Rand32, pokemon_id: usize, min_level: u8, mut max_level: u8, ivs: Option<StatSet>, evs: Option<StatSet>) -> Self {
        if min_level == max_level {
            max_level += 1;
        }

        Self {

            pokemon_id: pokemon_id,
            level: random.rand_range(min_level as u32..max_level as u32) as u8,
            ivs: ivs,
            evs: evs,
            current_hp: None,
            moves: None,
            exp: None,
            friendship: None,

        }

	}
    
    pub fn from_pokemon(instance: PokemonInstance) -> Self { // maybe reduce code size by deleting stuff later

        let mut ivs = Some(instance.ivs);
        if instance.ivs.eq(&StatSet::uniform(15)) {
            ivs = None;
        }

        let mut evs = Some(instance.evs);
        if instance.evs.eq(&StatSet::uniform(0)) {
            evs = None;
        }

        Self {

            pokemon_id: instance.pokemon.number,
            level: instance.level,
            current_hp: Some(instance.current_hp),
			ivs: ivs,
			evs: evs,
            moves: Some(SavedPokemonMoveSet::from_instance(instance.moves)),
            exp: None,
            friendship: None,

        }

    }

	pub fn from_owned_pokemon(pokemon: OwnedPokemon) -> Self {

		Self {
			
			exp: Some(pokemon.exp),
            friendship: Some(pokemon.friendship),
            ..SavedPokemon::from_pokemon(pokemon.instance)

		}

    }
    
	pub fn to_pokemon(&self, pokedex: &Pokedex) -> PokemonInstance {

        let moves = if let Some(moves) = &self.moves {
            moves.to_instance(pokedex)
        } else {
            PokemonInstance::moves_to_instance(pokedex.moves_from_level(self.pokemon_id, self.level))
        };

        let pokemon = pokedex.pokemon_from_id(self.pokemon_id).clone();
        let ivs = self.ivs.unwrap_or(StatSet::uniform(15));
        let evs = self.evs.unwrap_or(StatSet::uniform(0));

        let base = get_stats(&pokemon, ivs, evs, self.level);

        let current_hp = self.current_hp.unwrap_or(calculate_hp(pokemon.base_hp, ivs.hp, evs.hp, self.level));

		PokemonInstance {

			pokemon: pokemon,

			moves: moves,

			level: self.level,

			ivs: ivs,
            evs: evs,

		    base: base,
		    current_hp: current_hp,

		}

    }

    pub fn to_owned_pokemon(&self, pokedex: &Pokedex) -> OwnedPokemon {

		OwnedPokemon {

			instance: self.to_pokemon(pokedex),
			exp: self.exp.unwrap_or(0),
			friendship: self.friendship.unwrap_or(/*instance.pokemon.friendship*/70),

		}

    }

}