use std::ops::{Deref, Range};

use pokengine::pokedex::{
    item::{bag::InitBag, Item},
    moves::Move,
    pokemon::{owned::OwnedPokemon, Pokemon},
};

use battle::{pokemon::remote::InitUnknownPokemon, prelude::BattleData};

use crate::BattleTrainer;

type PlayerParty<ID, P> = battle::party::PlayerParty<ID, usize, P, BattleTrainer>;

pub type GuiRemotePlayer<ID> = PlayerParty<ID, Option<InitUnknownPokemon>>;

pub struct GuiLocalPlayer<ID> {
    pub player: PlayerParty<ID, OwnedPokemon>,
    pub selecting: Option<Range<usize>>,
    pub bag: InitBag,
    pub data: BattleData,
}

pub struct GuiRemotePlayers<ID> {
    pub current: usize,
    pub players: indexmap::IndexMap<ID, GuiRemotePlayer<ID>>,
}

impl<ID> Default for GuiRemotePlayers<ID> {
    fn default() -> Self {
        Self {
            current: Default::default(),
            players: Default::default(),
        }
    }
}
