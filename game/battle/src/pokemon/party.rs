use crate::{
    deps::vec::ArrayVec,
    pokedex::{
        pokemon::{
            instance::{PokemonInstance, BorrowedPokemon},
            party::{PokemonParty, BorrowedParty},
        },
        moves::target::PlayerId,
    },
};

use crate::{
    client::{LocalBattleClient, BattleClient},
    pokemon::{
        ActivePokemon,
        view::{BattlePartyKnown, BattlePartyUnknown, UnknownPokemon},
    },
};

pub struct BattlePlayer {

    pub id: PlayerId,
    pub name: String,

    pub client: LocalBattleClient,

    pub pokemon: BattleParty,
    pub active: PartyActive,

}

pub type BattleParty = ArrayVec<[BattlePartyPokemon; 6]>;

pub struct BattlePartyPokemon {
    pub pokemon: BorrowedPokemon,
    pub known: bool,
    pub requestable: bool,
}

impl From<BorrowedPokemon> for BattlePartyPokemon {
    fn from(pokemon: BorrowedPokemon) -> Self {
        Self {
            pokemon,
            known: false,
            requestable: false,
        }
    }
}

impl BattlePartyPokemon {
    pub fn know(&mut self) -> Option<UnknownPokemon> {
        (!self.known).then(|| {
            self.known = true;
            UnknownPokemon::new(self.pokemon.value())
        })
    }
}

pub enum PartyActive {
    Single([ActivePokemon; 1]),
    Double([ActivePokemon; 2]),
    Triple([ActivePokemon; 3]),
    Other(Vec<ActivePokemon>),
}

impl PartyActive {
    pub fn get(&self, index: usize) -> Option<&ActivePokemon> {
        match self {
            PartyActive::Single(a) => a as &[ActivePokemon],
            PartyActive::Double(a) => a,
            PartyActive::Triple(a) => a,
            PartyActive::Other(a) => a,
        }.get(index)
    }
    pub fn get_mut(&mut self, index: usize) -> Option<&mut ActivePokemon> {
        match self {
            PartyActive::Single(a) => a as &mut [ActivePokemon],
            PartyActive::Double(a) => a,
            PartyActive::Triple(a) => a,
            PartyActive::Other(a) => a,
        }.get_mut(index)
    }
    pub fn set(&mut self, index: usize, active: ActivePokemon) {
        let a = match self {
            PartyActive::Single(a) => a as &mut [ActivePokemon],
            PartyActive::Double(a) => a,
            PartyActive::Triple(a) => a,
            PartyActive::Other(a) => a,
        }.get_mut(index);
        if let Some(a) = a {
            *a = active;
        }
    }
    pub fn iter(&self) -> std::slice::Iter<'_, ActivePokemon> {
        match self {
            PartyActive::Double(a) => a as &[ActivePokemon],
            PartyActive::Triple(a) => a,
            PartyActive::Other(a) => a,
            PartyActive::Single(a) => a,
        }.iter()
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, ActivePokemon> {
        match self {
            PartyActive::Double(a) => a as &mut [ActivePokemon],
            PartyActive::Triple(a) => a,
            PartyActive::Other(a) => a,
            PartyActive::Single(a) => a,
        }.iter_mut()
    }
    pub fn len(&self) -> usize {
        match self {
            PartyActive::Single(..) => 1,
            PartyActive::Double(..) => 2,
            PartyActive::Triple(..) => 3,
            PartyActive::Other(a) => a.len(),
        }
    }
}

impl BattlePlayer {

    pub fn new(id: PlayerId, name: &str, party: BorrowedParty, client: Box<dyn BattleClient>, active: usize) -> Self {

        let mut active_pokemon = Vec::with_capacity(active);
        let mut count = 0;

        while active_pokemon.len() <= active {
            match party.get(count) {
                Some(p) => if !p.value().fainted() {
                    active_pokemon.push(ActivePokemon::Some(count, None));
                },
                None => active_pokemon.push(ActivePokemon::None),
            }
            count+=1;
        }

        let active = match active {
            0 => panic!("Cannot create a battle party with 0 active pokemon!"),
            1 => PartyActive::Single([active_pokemon.remove(0)]),
            2 => PartyActive::Double([active_pokemon.remove(0), active_pokemon.remove(0)]),
            3 => PartyActive::Triple([
                active_pokemon.remove(0),
                active_pokemon.remove(0),
                active_pokemon.remove(0)
            ]),
            len => PartyActive::Other(party.iter().enumerate().flat_map(|(i, p)| {
                (i < len).then(|| {
                    ActivePokemon::new(i, !p.value().fainted())
                })
            }).collect()),
        };       

        Self {
            id,
            name: name.to_string(),
            client: LocalBattleClient::new(client),
            active,
            pokemon: party.into_iter().map(|b| BattlePartyPokemon::from(b)).collect(),
        }
    }

    pub fn all_fainted(&self) -> bool {
        !self.pokemon.iter().any(|p| !p.pokemon.value().fainted()) || self.pokemon.len() == 0
    }

    pub fn any_inactive(&self) -> bool {
        self.pokemon.iter().enumerate().filter(|(i, _)| !self.active_contains(*i)).any(|(_, p)| !p.pokemon.value().fainted())
    }

    pub fn active(&self, active: usize) -> Option<&PokemonInstance> {
        self.active_index(active).map(|index| self.pokemon.get(index).map(|b| b.pokemon.value())).flatten()
    }

    pub fn active_mut(&mut self, active: usize) -> Option<&mut PokemonInstance> {
        self.active_index(active).map(move |index| self.pokemon.get_mut(index).map(|b| b.pokemon.value_mut())).flatten()
    }

    pub fn know(&mut self, index: usize) -> Option<UnknownPokemon> {
        self.pokemon.get_mut(index).map(|p| p.know()).flatten()
    }

    pub fn active_index(&self, index: usize) -> Option<usize> {
        self.active.get(index).map(|active| active.index()).flatten()
    }

    pub fn active_contains(&self, index: usize) -> bool {
        self.active.iter().any(|active| match active {
            ActivePokemon::Some(i, _) => i == &index,
            _ => false,
        })
    }

    pub fn needs_replace(&self) -> bool {
        self.active.iter().any(|a| matches!(a, ActivePokemon::ToReplace))
    }

    pub fn reveal_active(&mut self) {
        for active in self.active.iter() {
            if let Some(index) = active.index() {
                if let Some(pokemon) = self.pokemon.get_mut(index) {
                    pokemon.known = true;
                }
            }
        }
    }

    pub fn replace(&mut self, active: usize, new: Option<usize>) {
        self.active.set(active, match new {
            Some(new) => ActivePokemon::Some(new, None),
            None => ActivePokemon::None,
        });
    }

    pub fn ready_to_move(&self) -> bool {
        self.active.iter().filter(|a| a.is_active()).all(|a| match a {
            ActivePokemon::Some(_, m) => m.is_some(),
            _ => false
        })
    }

    pub fn collect_ref(&self) -> ArrayVec<[&PokemonInstance; 6]> {
        self.pokemon.iter().map(|b| b.pokemon.value()).collect()
    }

    pub fn collect_cloned(&self) -> PokemonParty {
        self.pokemon.iter().map(|b| b.pokemon.cloned()).collect()
    }

    pub fn collect_owned(self) -> BorrowedParty {
        self.pokemon.into_iter().map(|b|b.pokemon).collect()
    }    

    pub fn as_known(&self) -> BattlePartyKnown {
        BattlePartyKnown {
            id: self.id,
            name: self.name.clone(),
            pokemon: self.pokemon.iter().map(|b| b.pokemon.cloned()).collect(),
            active: self.active.iter().map(|active| active.index()).collect(),
        }
    }

    pub fn as_unknown(&self) -> BattlePartyUnknown {
        let active = self.active.iter().map(|active| active.index()).collect::<ArrayVec<[Option<usize>; 3]>>();
        let mut pokemon = ArrayVec::new();
        for p in self.pokemon.iter() {
            pokemon.push(p.known.then(|| UnknownPokemon::new(p.pokemon.value())));
        }
        BattlePartyUnknown {
            id: self.id,
            name: self.name.clone(),
            pokemon,
            active,
        }
    }

}