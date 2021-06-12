use std::borrow::Cow;

use pokedex::moves::target::PlayerId;

use crate::{BattleType, pokemon::{BattleClientActionInstance, BattleMove, view::{BattlePartyKnown, BattlePartyUnknown, UnknownPokemon}}};

pub type Active = usize; pub type PartyIndex = usize;

pub enum ClientMessage {
    Move(Active, BattleMove),
    FaintReplace(Active, PartyIndex),
    // RequestPokemon(PlayerId, usize)
    FinishedTurnQueue,
    Forfeit,
}

pub enum ServerMessage<'a> {
    User(BattleType, BattlePartyKnown),
    Opponents(BattlePartyUnknown),
    // UpdatePokemon(PlayerId, usize, UnknownPokemon),
    // PokemonRequest(PlayerId, usize, PokemonInstance),
    StartSelecting,
    TurnQueue(Cow<'a, Vec<BattleClientActionInstance>>),
    AskFinishedTurnQueue,
    // SelectMoveError(usize),

    // RequestFaintReplace(Active),
    // FaintReplaceError(Active),

    FaintReplace(PlayerId, Active, Option<PartyIndex>),
    AddUnknown(PlayerId, PartyIndex, UnknownPokemon),
    // Winner(PlayerId),
}