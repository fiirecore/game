use crate::{
    deps::vec::ArrayVec,
    pokedex::{
        pokemon::{
            instance::PokemonInstance,
            party::{
                PokemonParty,
                BorrowedParty,
            },
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

pub struct BattleParty {

    pub id: PlayerId,
    pub name: String,

    pub client: LocalBattleClient,

    pub pokemon: BorrowedParty,
    pub active: PartyActive,

}

// pub enum BattlePartyType {
//     Single,
//     Double,
//     Triple,
//     Other(usize),
// }

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

impl BattleParty {

    pub fn new(id: PlayerId, name: &str, party: BorrowedParty, client: Box<dyn BattleClient>, active: usize) -> Self {

        let active = match active {
            0 => panic!("Cannot create a battle party with 0 active pokemon!"),
            1 => PartyActive::Single([ActivePokemon::new(0, party.get(0).map(|p| !p.value().fainted()).unwrap_or_default())]),
            2 => PartyActive::Double([ActivePokemon::new(0, party.get(0).map(|p| !p.value().fainted()).unwrap_or_default()), ActivePokemon::new(1, party.get(1).map(|p| !p.value().fainted()).unwrap_or_default())]),
            3 => PartyActive::Triple([
                ActivePokemon::new(0, party.get(0).map(|p| !p.value().fainted()).unwrap_or_default()),
                ActivePokemon::new(1, party.get(1).map(|p| !p.value().fainted()).unwrap_or_default()),
                ActivePokemon::new(2, party.get(2).map(|p| !p.value().fainted()).unwrap_or_default()),
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

    pub fn needs_replace(&self) -> bool {
        self.active.iter().any(|a| matches!(a, ActivePokemon::ToReplace))
    }

    // pub fn request_replace(&mut self) {
    //     for (i, _) in self.active.iter().enumerate().filter(|(_, a)| matches!(a, ActivePokemon::ToReplace)) {
    //         self.client.send(crate::message::ServerMessage::RequestFaintReplace(i));
    //     }
    // }

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
        self.pokemon.iter().map(|b| b.value()).collect()
    }

    pub fn collect_cloned(&self) -> PokemonParty {
        self.pokemon.iter().map(|b| b.cloned()).collect()
    }

    pub fn collect_owned(self) -> BorrowedParty {
        self.pokemon
    }    

    pub fn as_known(&self) -> BattlePartyKnown {
        BattlePartyKnown {
            id: self.id,
            pokemon: self.collect_cloned(),
            active: self.active.iter().map(|active| active.index()).collect(),
        }
    }

    pub fn as_unknown(&self) -> BattlePartyUnknown {
        let active = self.active.iter().map(|active| active.index()).collect::<ArrayVec<[Option<usize>; 3]>>();
        let mut pokemon = ArrayVec::new();
        for (i, p) in self.collect_ref().iter().enumerate() {
            if active.contains(&Some(i)) {
                pokemon.push(Some(UnknownPokemon::new(p)));
            } else {
                pokemon.push(None);
            }
        }
        BattlePartyUnknown {
            id: self.id,
            pokemon,
            active,
        }
    }

}