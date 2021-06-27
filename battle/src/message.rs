use serde::{Deserialize, Serialize};

use pokedex::{
    battle::{
        party::knowable::{BattlePartyKnown, BattlePartyUnknown},
        view::UnknownPokemon,
        ActionInstance, Active, BattleMove, PartyIndex, PokemonIndex,
    },
    moves::MoveRef,
    pokemon::instance::PokemonInstance,
};

use crate::{pokemon::BattleClientAction, BattleData};

#[derive(Debug, Deserialize, Serialize)]
pub enum ClientMessage {
    // Connect(BattleParty),
    Move(Active, BattleMove),
    FaintReplace(Active, PartyIndex),
    RequestPokemon(PartyIndex),
    FinishedTurnQueue,
    Forfeit,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum ServerMessage<ID: Sized + Copy + core::fmt::Debug + core::fmt::Display + PartialEq> {
    User(BattleData, BattlePartyKnown<ID>),
    Opponents(BattlePartyUnknown<ID>),
    // UpdatePokemon(TrainerId, usize, UnknownPokemon),
    PokemonRequest(PartyIndex, PokemonInstance),
    StartSelecting,
    TurnQueue(Vec<ActionInstance<ID, BattleClientAction<ID>>>),
    AskFinishedTurnQueue,
    /*#[deprecated(note = "should not be sent to opponent")]*/
    AddMove(PokemonIndex<ID>, usize, MoveRef), // pokemon, move index, move
    // GainExp(Active, Experience),
    // LevelUp(Level, Option<Vec<MoveRef>>),
    // SelectMoveError(usize),
    // Catch(PokemonIndex),
    // RequestFaintReplace(Active),
    // FaintReplaceError(Active),
    FaintReplace(PokemonIndex<ID>, Option<PartyIndex>),
    AddUnknown(PartyIndex, UnknownPokemon),
    Winner(ID),
}
