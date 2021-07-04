use pokedex::{
    battle::{
        party::{battle::BattlePartyPokemon, BattleParty},
        ActivePokemon,
    },
    pokemon::{instance::BorrowedPokemon, party::Party},
    trainer::TrainerData,
};

use crate::client::{BattleClient, local::LocalBattleClient};

mod settings;

pub use settings::*;

#[cfg(feature = "ai")]
pub mod ai;
pub struct BattlePlayer<ID: Sized + Copy + core::fmt::Debug + core::fmt::Display + PartialEq> {
    pub client: LocalBattleClient<ID>,
    pub party: BattleParty<ID, ActivePokemon, BattlePartyPokemon>,
    pub settings: PlayerSettings,
}

impl<ID: Sized + Copy + core::fmt::Debug + core::fmt::Display + PartialEq> BattlePlayer<ID> {
    pub fn new(
        id: ID,
        party: Party<BorrowedPokemon>,
        trainer: Option<TrainerData>,
        settings: PlayerSettings,
        client: Box<dyn BattleClient<ID>>,
        active_size: usize,
    ) -> Self {
        let mut active = Vec::with_capacity(active_size);
        let mut count = 0;

        while active.len() < active_size {
            match party.get(count) {
                Some(p) => {
                    if !p.fainted() {
                        active.push(ActivePokemon::Some(count, None));
                    }
                }
                None => active.push(ActivePokemon::None),
            }
            count += 1;
        }

        Self {
            client: LocalBattleClient::new(client),
            party: BattleParty {
                id,
                trainer,
                active,
                pokemon: party.into_iter().map(BattlePartyPokemon::from).collect(),
            },
            settings,
        }
    }
}
