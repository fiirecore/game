use crate::{
    deps::vec::ArrayVec,
    pokedex::{
        pokemon::{
            instance::PokemonInstance,
            party::{
                MoveableParty,
                PokemonParty,
            },
        },
    },
};

use crate::battle::{
    pokemon::{
        ActivePokemon,
        PokemonOption,
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

    pub fn new(mut party: MoveableParty,/*player: Box<dyn BattlePlayer>,*/ size: usize) -> Self {

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
                Some((index, pokemon)) => ActivePokemon::new(index, pokemon),
                None => ActivePokemon::default()
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

    pub fn replace(&mut self, active_index: usize, new: Option<usize>) {
        if let PokemonOption::Some(index, instance) = self.active[active_index].pokemon.take() {
            if self.pokemon[index].is_some() {
                panic!("Party spot at {} is already occupied!", active_index);
            }
            self.pokemon[index] = Some(instance);
            self.active[active_index].pokemon = match new {
                Some(new) => PokemonOption::Some(new, self.pokemon[new].take().unwrap()),
                None => PokemonOption::None,
            };
            self.active[active_index].dequeue();
        }
    }

    pub fn collect_ref(&self) -> ArrayVec<[&PokemonInstance; 6]> {
        let mut party = Vec::new();
        for pokemon in self.pokemon.iter() {
            party.push(pokemon.as_ref().map(|p| p.value()));
        }
        for active in &self.active {
            if let PokemonOption::Some(index, pokemon) = &active.pokemon {
                party[*index] = Some(pokemon.value());
            }
        }
        party.into_iter().flatten().collect()
    }

    pub fn collect_cloned(&self) -> PokemonParty {
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

    pub fn as_known(&self) -> super::BattlePartyKnown {
        super::BattlePartyKnown {
            pokemon: self.collect_cloned(),
            active: self.active.iter().map(|active| active.pokemon.index()).collect(),
        }
    }

    pub fn as_unknown(&self) -> super::BattlePartyUnknown {
        let active = self.active.iter().map(|active| active.pokemon.index()).collect::<ArrayVec<[Option<usize>; 3]>>();
        let mut pokemon = ArrayVec::new();
        for (i, p) in self.collect_ref().iter().enumerate() {
            if active.contains(&Some(i)) {
                pokemon.push(Some(super::PokemonUnknown::new(p)));
            } else {
                pokemon.push(None);
            }
        }
        super::BattlePartyUnknown {
            pokemon,
            active,
        }
    }

}