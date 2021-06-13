use std::borrow::Cow;

use pokedex::pokemon::instance::PokemonInstance;

use crate::{
    pokemon::{
        view::{BattlePartyKnown, BattlePartyUnknown, UnknownPokemon},
        ActivePokemonIndex, BattleClientActionInstance, BattleMove,
    },
    BattleData,
};

pub type Active = usize;
pub type PartyIndex = usize;

pub enum ClientMessage {
    Move(Active, BattleMove),
    FaintReplace(Active, PartyIndex),
    RequestPokemon(PartyIndex),
    FinishedTurnQueue,
    Forfeit,
}

pub enum ServerMessage<'a> {
    User(BattleData, BattlePartyKnown),
    Opponents(BattlePartyUnknown),
    // UpdatePokemon(PlayerId, usize, UnknownPokemon),
    PokemonRequest(PartyIndex, PokemonInstance),
    StartSelecting,
    TurnQueue(Cow<'a, Vec<BattleClientActionInstance>>),
    AskFinishedTurnQueue,
    // SelectMoveError(usize),
    // Catch(ActivePokemonIndex),
    // RequestFaintReplace(Active),
    // FaintReplaceError(Active),
    FaintReplace(ActivePokemonIndex, Option<PartyIndex>),
    AddUnknown(PartyIndex, UnknownPokemon),
    // Winner(PlayerId),
}
