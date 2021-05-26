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
    tetra::Context,
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

    pub fn new(ctx: &mut Context, mut party: MoveableParty,/*player: Box<dyn BattlePlayer>,*/ size: usize, side: PokemonTexture, position: BattleGuiPosition) -> Self {

        let mut active = vec![None; size];
        let mut current = 0;

        for (index, pokemon) in party.iter().flatten().enumerate() {
			if !pokemon.value().fainted() {
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
                        status: PokemonStatusGui::with(ctx, index, instance),
                        renderer: PokemonRenderer::with(ctx, index, instance, side),
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
                        status: PokemonStatusGui::new(ctx, index),
                        renderer: PokemonRenderer::new(ctx, index, side),
                        last_move: None,
                    }
                }
            }).collect(),
            pokemon: party,
            // player,
        }
    }

    pub fn all_fainted(&self) -> bool {
        for pokemon in self.pokemon.iter().flatten() {
            if !pokemon.value().fainted() {
                return false;
            }
        }
        for active in self.active.iter() {
            if let Some(pokemon) = active.pokemon.as_ref() {
                if !pokemon.fainted() {
                    return false;
                }
            }
        }
        true
    }

    pub fn any_inactive(&self) -> bool {
        for pokemon in self.pokemon.iter().flatten() {
            if !pokemon.value().fainted() {
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