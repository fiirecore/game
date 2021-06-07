use crate::{
    deps::{str::TinyStr16, vec::ArrayVec},
    pokedex::{
        pokemon::{
            instance::PokemonInstance,
            party::{
                PokemonParty,
                BorrowedParty,
            },
        },
    },
};

use crate::battle::pokemon::ActivePokemon;

// #[deprecated(note = "use enum instead")]
pub type ActivePokemonArray = ArrayVec<[ActivePokemon; 3]>;

pub type BattlePlayer = TinyStr16;


pub struct BattleParty {

    pub name: String,

    // pub client: Box<dyn BattleClient>,

    pub pokemon: BorrowedParty,
    pub active: ActivePokemonArray,

}

impl BattleParty {

    pub fn new(name: &str, party: BorrowedParty,/*player: Box<dyn BattlePlayer>,*/ size: usize) -> Self {

        let mut active = vec![None; size];
        let mut current = 0;

        for (index, pokemon) in party.iter().enumerate() {
			if !pokemon.value().fainted() {
				active[current] = Some(index);
                current += 1;
                if current == size {
                    break;
                }
			}
		}

        Self {
            name: name.to_string(),
            // client,
            active: active.into_iter().map(|active| match active {
                Some(index) => ActivePokemon::Some(index, None),
                None => ActivePokemon::default()
            }).collect(),
            pokemon: party,
        }
    }

    pub fn all_fainted(&self) -> bool {
        !self.pokemon.iter().any(|p| !p.value().fainted())
    }

    pub fn any_inactive(&self) -> bool {
        self.pokemon.iter().enumerate().filter(|(i, _)| !self.active_contains(*i)).any(|(_, p)| !p.value().fainted())
    }

    pub fn active(&self, active: usize) -> Option<&PokemonInstance> {
        self.active_index(active).map(|index| self.pokemon.get(index).map(|b| b.value())).flatten()
    }

    pub fn active_mut(&mut self, active: usize) -> Option<&mut PokemonInstance> {
        self.active_index(active).map(move |index| self.pokemon.get_mut(index).map(|b| b.value_mut())).flatten()
    }

    fn active_index(&self, index: usize) -> Option<usize> {
        self.active.get(index).map(|active| active.index()).flatten()
    }

    pub fn active_contains(&self, index: usize) -> bool {
        self.active.iter().any(|active| match active {
            ActivePokemon::Some(i, _) => i == &index,
            _ => false,
        })
    }

    pub fn any_replace(&self) -> Option<usize> {
        self.active.iter().enumerate().find(|(_, a)| matches!(a, ActivePokemon::ToReplace)).map(|(i, _)| i)
    }

    pub fn replace(&mut self, active: usize, new: Option<usize>) {
        self.active[active] = match new {
            Some(new) => ActivePokemon::Some(new, None),
            None => ActivePokemon::None,
        };
    }

    pub fn collect_ref(&self) -> ArrayVec<[&PokemonInstance; 6]> {
        self.pokemon.iter().map(|b| b.value()).collect()
    }

    pub fn collect_cloned(&self) -> PokemonParty {
        self.pokemon.iter().map(|b| b.cloned()).collect()
    }

    pub fn collect_owned(self) -> BorrowedParty {
        self.pokemon
    }

    pub fn as_known(&self) -> super::view::BattlePartyKnown {
        super::view::BattlePartyKnown {
            pokemon: self.collect_cloned(),
            active: self.active.iter().map(|active| active.index()).collect(),
        }
    }

    pub fn as_unknown(&self) -> super::view::BattlePartyUnknown {
        let active = self.active.iter().map(|active| active.index()).collect::<ArrayVec<[Option<usize>; 3]>>();
        let mut pokemon = ArrayVec::new();
        for (i, p) in self.collect_ref().iter().enumerate() {
            if active.contains(&Some(i)) {
                pokemon.push(Some(super::view::PokemonUnknown::new(p)));
            } else {
                pokemon.push(None);
            }
        }
        super::view::BattlePartyUnknown {
            pokemon,
            active,
        }
    }

}