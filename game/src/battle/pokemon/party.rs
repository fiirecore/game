use crate::{
    deps::vec::ArrayVec,
    pokedex::{
        pokemon::{
            instance::PokemonInstance,
            party::{
                MoveableParty,
                PersistentParty,
            },
        },
        texture::{
            PokemonTexture,
        }
    },
};

use crate::battle::{
    pokemon::{
        ActivePokemon,
        PokemonOption,
    },
    ui::{
        BattleGuiPosition,
        BattleGuiPositionIndex,
        pokemon::{
            PokemonRenderer,
            status::PokemonStatusGui,            
        },
    },
};

// #[deprecated(note = "use enum instead")]
pub type ActivePokemonArray = ArrayVec<[ActivePokemon; 3]>;

pub struct BattleParty {

    // pub name:
    pub pokemon: MoveableParty, // To - do: option<pokemonInstance> enum + unknown enum value
    pub active: ActivePokemonArray,

}

impl BattleParty {

    pub fn new(mut party: MoveableParty, size: usize, side: PokemonTexture, position: BattleGuiPosition) -> Self {

        let mut active = vec![None; size];
        let mut current = 0;

        for (index, pokemon) in party.iter().flatten().enumerate() {
			if pokemon.value().current_hp != 0 {
				active[current] = Some(index);
                current += 1;
                if current == size {
                    break;
                }
			}
		}

        // let mut pokemon: ArrayVec<[Option<PokemonInstance>; 6]> = party.into_iter().map(|pokemon| Some(pokemon)).collect();

        let size = active.len() as u8;

        Self {
            active: active.into_iter().enumerate().map(|(index, active)| match active.map(|index| party[index].take().map(|pokemon| (index, pokemon))).flatten() {
                Some((index2, pokemon)) => {
                    let instance = pokemon.value();
                    let index = BattleGuiPositionIndex::new(position, index as u8, size);
                    ActivePokemon {
                        status: PokemonStatusGui::with(index, instance),
                        renderer: PokemonRenderer::with(index, instance, side),
                        pokemon: PokemonOption::Some(index2, pokemon),
                        queued_move: None,
                        last_move: None,
                    }
                }
                None => {
                    let index = BattleGuiPositionIndex::new(position, index as u8, size);
                    ActivePokemon {
                        pokemon: PokemonOption::None,
                        queued_move: None,
                        status: PokemonStatusGui::new(index),
                        renderer: PokemonRenderer::new(index, side),
                        last_move: None,
                    }
                }
            }).collect(),
            pokemon: party,
        }
    }

    pub fn all_fainted(&self) -> bool {
        // self.pokemon.iter().flatten().find(|pokemon| pokemon.current_hp != 0).is_none() ||
        // self.active.iter().flat_map(|active| active.pokemon.as_ref()).find(|pokemon| pokemon.current_hp != 0).is_none()
        for pokemon in self.pokemon.iter().flatten() {
            if pokemon.value().current_hp != 0 {
                return false;
            }
        }
        for active in self.active.iter() {
            if let Some(pokemon) = active.pokemon.as_ref() {
                if pokemon.current_hp != 0 {
                    return false;
                }
            }
        }
        true
    }

    pub fn any_inactive(&self) -> bool {
        // self.pokemon.iter().flatten().find(|instance| instance.current_hp != 0).is_some()
        for pokemon in self.pokemon.iter().flatten() {
            if pokemon.value().current_hp != 0 {
                return true;
            }
        }
        false
    }

    pub fn pokemon(&self, active_index: usize) -> Option<&PokemonInstance> {
        self.active[active_index].pokemon.as_ref()
    }

    pub fn pokemon_mut(&mut self, active_index: usize) -> Option<&mut PokemonInstance> {
        self.active[active_index].pokemon.as_mut()
    }

    pub fn queue_replace(&mut self, active_index: usize, new: usize) {
        if let Some((index, instance)) = self.active[active_index].pokemon.replace(new) {
            if self.pokemon[index].is_some() {
                panic!("Party spot at {} is already occupied!", index);
            }
            let level = instance.value().level;
            self.pokemon[index] = Some(instance);
            self.active[active_index].update_status(level, true);
        }
    }

    pub fn run_replace(&mut self) {
        for active in self.active.iter_mut() {
            if let PokemonOption::ToReplace(new) = &active.pokemon {
                let new = *new;
                active.pokemon = PokemonOption::Some(new, self.pokemon[new].take().expect("Could not get inactive pokemon from party!"));
                active.reset();
            }
        }
    }

    pub fn replace(&mut self, active_index: usize, new: usize) {
        if let PokemonOption::Some(index, instance) = self.active[active_index].pokemon.take() {
            if self.pokemon[index].is_some() {
                panic!("Party spot at {} is already occupied!", active_index);
            }
            self.pokemon[index] = Some(instance);
            self.active[active_index].pokemon = PokemonOption::Some(new, self.pokemon[new].take().unwrap());
            self.active[active_index].reset();
        }
    }

    pub fn remove_pokemon(&mut self, active_index: usize) {
        if let PokemonOption::Some(index, instance) = self.active[active_index].pokemon.take() {
            if self.pokemon[index].is_some() {
                panic!("Party spot at {} is already occupied!, \n {:?} \n {:?}", index, self.active, self.pokemon);
            }
            self.pokemon[index] = Some(instance);
            self.active[active_index].reset();
        }
    }

    pub fn collect_cloned(&self) -> PersistentParty {
        let mut party = self.pokemon.clone();
        for pokemon in self.active.iter() {
            if let PokemonOption::Some(index, instance) = pokemon.pokemon.clone() {
                party[index] = Some(instance);
            }
        }
        party.into_iter().flatten().map(|cow| cow.owned()).collect()
    }

    pub fn collect_owned(self) -> MoveableParty {
        let mut party = self.pokemon;
        for pokemon in self.active.into_iter() {
            if let PokemonOption::Some(index, instance) = pokemon.pokemon {
                party[index] = Some(instance);
            }
        }
        party
    }

}