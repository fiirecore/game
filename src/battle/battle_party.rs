use firecore_pokedex::moves::PokemonMove;
use firecore_pokedex::pokemon::battle::BattlePokemon;
use firecore_pokedex::pokemon::party::PokemonParty;
use firecore_pokedex::pokemon::texture::PokemonTexture;
use macroquad::prelude::Vec2;
use macroquad::prelude::warn;
use smallvec::SmallVec;

use crate::util::battle_data::BattlePokemonParty;
use crate::util::graphics::Texture;

use super::gui::pokemon_texture::ActivePokemonRenderer;

pub struct BattleParty {

    pub pokemon: SmallVec<[GameBattlePokemon; 6]>,
    pub active: usize,

    pub renderer: ActivePokemonRenderer,

    pub next_move: Option<BattleMoveStatus>,

}

impl BattleParty {

    pub fn from_saved(party: &PokemonParty, side: PokemonTexture, active_pos: Vec2) -> Self {

        let mut battle_party = BattlePokemonParty::new();

        for pokemon in party {
			if let Some(pokemon) = BattlePokemon::new(pokemon) {
				battle_party.push(pokemon);
			} else {
				warn!("Could not add pokemon with id {} to pokemon party", pokemon.id);
			}
		}

        Self::new(
            battle_party,
            side, 
            active_pos,
        )       

    }

    pub fn new(party: BattlePokemonParty, side: PokemonTexture, active_pos: Vec2) -> Self {

        let mut active = 0;

        for (index, pokemon) in party.iter().enumerate() {
			if pokemon.current_hp != 0 {
				active = index;
				break;
			}
		}

        Self {
            pokemon: party.into_iter().map(|pokemon| 
                GameBattlePokemon {
                    texture: crate::pokemon::pokemon_texture(&pokemon.pokemon.data.id, side),
                    pokemon: pokemon,
                }
            ).collect(),
            renderer: ActivePokemonRenderer::new(active_pos),
            active,
            next_move: None,
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

    pub fn next_move_queued(&self) -> bool {
        self.next_move.as_ref().map(|next_move| next_move.queued).unwrap_or_default()
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