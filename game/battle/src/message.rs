use std::borrow::Cow;

use pokedex::{
    pokemon::instance::PokemonInstance,
    moves::{MoveRef, target::PlayerId},
};

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
    // Connect(BattleParty),
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
    #[deprecated(note = "should not be sent to opponent")]
    AddMove(ActivePokemonIndex, usize, MoveRef), // pokemon, move index, move
    // GainExp(Active, Experience),
    // LevelUp(Level, Option<Vec<MoveRef>>),
    // SelectMoveError(usize),
    // Catch(ActivePokemonIndex),
    // RequestFaintReplace(Active),
    // FaintReplaceError(Active),
    FaintReplace(ActivePokemonIndex, Option<PartyIndex>),
    AddUnknown(PartyIndex, UnknownPokemon),
    Winner(PlayerId),
}
