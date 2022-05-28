use std::ops::{Deref, Range};

use pokengine::pokedex::{
    item::{bag::Bag, Item},
    moves::Move,
    pokemon::{owned::OwnedPokemon, Pokemon},
};

use battle::{pokemon::remote::UnknownPokemon, prelude::BattleData};

use crate::BattleTrainer;

type PlayerParty<ID, P> = battle::party::PlayerParty<ID, usize, P, BattleTrainer>;

pub type InitLocalPlayer<ID, P, M, I> = PlayerParty<ID, OwnedPokemon<P, M, I>>;

pub type GuiRemotePlayer<ID, P> = PlayerParty<ID, Option<UnknownPokemon<P>>>;

pub struct GuiLocalPlayer<
    ID,
    P: Deref<Target = Pokemon> + Clone,
    M: Deref<Target = Move> + Clone,
    I: Deref<Target = Item> + Clone,
> {
    pub player: PlayerParty<ID, OwnedPokemon<P, M, I>>,
    pub selecting: Option<Range<usize>>,
    pub bag: Bag<I>,
    pub data: BattleData,
}

pub struct GuiRemotePlayers<ID, P> {
    pub current: usize,
    pub players: indexmap::IndexMap<ID, GuiRemotePlayer<ID, P>>,
}

impl<ID, P> Default for GuiRemotePlayers<ID, P> {
    fn default() -> Self {
        Self {
            current: Default::default(),
            players: Default::default(),
        }
    }
}
