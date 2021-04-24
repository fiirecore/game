use game::{
    pokedex::pokemon::{
        instance::PokemonInstance,
        party::PokemonParty,
        texture::PokemonTexture,
    },
    macroquad::prelude::{Vec2, Texture2D, warn},
    util::smallvec::SmallVec,
    battle::BattlePokemonParty,
    textures::pokemon_texture,
};

use crate::{
    pokemon::{
        BattlePokemon,
        ActivePokemonRenderer,
        BattleMoveStatus,
    },
};

pub struct BattleParty {

    pub pokemon: SmallVec<[BattlePokemon; 6]>,
    pub active: usize,

    pub renderer: ActivePokemonRenderer,

    pub next_move: Option<BattleMoveStatus>,

}

impl BattleParty {

    pub fn from_saved(party: &PokemonParty, side: PokemonTexture, active_pos: Vec2) -> Self {

        let mut battle_party = BattlePokemonParty::new();

        for pokemon in party {
			if let Some(pokemon) = PokemonInstance::new(pokemon) {
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
                BattlePokemon {
                    texture: pokemon_texture(&pokemon.pokemon.data.id, side),
                    pokemon: pokemon,
                }
            ).collect(),
            renderer: ActivePokemonRenderer::new(active_pos, side == PokemonTexture::Front),
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

    pub fn active(&self) -> &PokemonInstance {
        &self.pokemon.get(self.active).expect("Could not get pokemon from battle party!").pokemon
    }

    pub fn active_mut(&mut self) -> &mut PokemonInstance {
        &mut self.pokemon.get_mut(self.active).expect("Could not get pokemon from battle party!").pokemon
    }

    pub fn active_texture(&self) -> Texture2D {
        self.pokemon[self.active].texture
    }

}