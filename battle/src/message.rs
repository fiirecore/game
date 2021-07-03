use serde::{Deserialize, Serialize};

use pokedex::{
    battle::{
        party::knowable::{BattlePartyKnown, BattlePartyUnknown},
        view::UnknownPokemon,
        ActionInstance, Active, BattleMove, PartyIndex, PokemonIndex,
    },
    moves::MoveRef,
    pokemon::{instance::PokemonInstance, party::PokemonParty},
};

use crate::{client::action::BattleClientAction, BattleData};

#[derive(Debug, Deserialize, Serialize)]
pub enum ClientMessage {
    // Connect(BattleParty),
    Move(Active, BattleMove),
    FaintReplace(Active, PartyIndex),
    RequestPokemon(PartyIndex),
    FinishedTurnQueue,
    AddLearnedMove(PartyIndex, usize, MoveRef),
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
    // AskFinishedTurnQueue,
    // SelectMoveError(usize),
    // Catch(PokemonIndex),
    // RequestFaintReplace(Active),
    // FaintReplaceError(Active),
    FaintReplace(PokemonIndex<ID>, Option<PartyIndex>),
    AddUnknown(PartyIndex, UnknownPokemon),
    Winner(ID, Option<Box<PokemonParty>>), // party is for when user requests party back. used in remote clients
}
