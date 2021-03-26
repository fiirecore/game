use firecore_pokedex::moves::PokemonMove;
use firecore_pokedex::pokemon::battle::BattlePokemon;
use firecore_pokedex::pokemon::party::PokemonParty;
use firecore_pokedex::pokemon::texture::PokemonTexture;
use macroquad::prelude::Vec2;
use macroquad::prelude::warn;
use smallvec::SmallVec;

use crate::util::graphics::Texture;

use super::gui::pokemon_texture::ActivePokemonRenderer;

#[derive(Default)]
pub struct BattleParty {

    pub pokemon: SmallVec<[GameBattlePokemon; 6]>,
    pub active: usize,

    pub renderer: ActivePokemonRenderer,
    pub next_move: BattleMoveStatus,

}

impl BattleParty {

    pub fn new(party: &PokemonParty, side: PokemonTexture, active_pos: Vec2) -> Self {

        let mut pokemon = SmallVec::new();

        for pokemon_instance in &party.pokemon {
			if let Some(battle_pokemon) = BattlePokemon::new(pokemon_instance) {
				pokemon.push(
                    GameBattlePokemon {
                        texture: crate::pokemon::pokemon_texture(&battle_pokemon.pokemon.data.number, side),
                        pokemon: battle_pokemon,
                    }
                );
			} else {
				warn!("Could not add pokemon with id {} to pokemon party", pokemon_instance.id);
			}
		}


        Self {
            pokemon,
            renderer: ActivePokemonRenderer::new(active_pos),
            ..Default::default()
        }

    }

    pub fn select_pokemon(&mut self, selected: usize) {
		self.active = selected;
		
	}

    pub fn all_fainted(&self) -> bool {
        for pokemon in &self.pokemon {
            if pokemon.pokemon.current_hp != 0 {
                return false;
            }
        }
        true
    }

    pub fn active(&self) -> &BattlePokemon {
        &self.pokemon.get(self.active).expect("Could not get pokemon from battle party!").pokemon
    }

    pub fn active_mut(&mut self) -> &mut BattlePokemon {
        &mut self.pokemon.get_mut(self.active).expect("Could not get pokemon from battle party!").pokemon
    }

    pub fn active_texture(&self) -> Texture {
        self.pokemon[self.active].texture
    }

}


pub struct GameBattlePokemon {

    pub pokemon: BattlePokemon,
    pub texture: Texture,

}

#[derive(Default)]
pub struct BattleMoveStatus {

    pub pokemon_move: PokemonMove,
    pub queued: bool,

}

impl BattleMoveStatus {

    pub fn new(pokemon_move: &PokemonMove) -> Self {
        Self {
            pokemon_move: pokemon_move.clone(),
            queued: true,
        }
    }

}