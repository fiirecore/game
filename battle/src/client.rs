use serde::{Deserialize, Serialize};
use pokedex::{
    pokemon::{Experience, stat::{StatType, Stage}},
    item::ItemRef,
    moves::{target::MoveTargetInstance, MoveRef},
    battle::view::UnknownPokemon,
    types::Effective,
    battle::PokemonIndex,
};

use crate::message::{ServerMessage, ClientMessage};

pub struct LocalBattleClient<ID: Sized + Copy + core::fmt::Debug + core::fmt::Display> {
    client: Box<dyn BattleClient<ID>>,
    state: BattleClientState,
    // pub forfeit: bool,
}

impl<ID: Sized + Copy + core::fmt::Debug + core::fmt::Display + PartialEq> LocalBattleClient<ID> {
    pub fn new(client: Box<dyn BattleClient<ID>>) -> Self {
        Self {
            client,
            state: BattleClientState::SelectMoves,
            // forfeit: false,
        }
    }
    pub fn send(&mut self, message: ServerMessage<ID>) {
        match &message {
            ServerMessage::StartSelecting => if matches!(self.state, BattleClientState::FinishedTurnQueue) {
                self.state = BattleClientState::SelectMoves;
            },
            ServerMessage::TurnQueue(..) => if matches!(self.state, BattleClientState::SelectMoves) {
                self.state = BattleClientState::ProcessTurnQueue;
            },
            _ => (),
        }
        self.client.give_client(message)
    }
    pub fn receive(&mut self) -> Option<ClientMessage> {
        let message = self.client.give_server();
        if let Some(message) = &message {
            if let ClientMessage::FinishedTurnQueue = message {
                if matches!(self.state, BattleClientState::ProcessTurnQueue) {
                    self.state = BattleClientState::FinishedTurnQueue;
                }
            }
        }
        message
    }
    pub fn finished_turn(&self) -> bool {
        matches!(self.state, BattleClientState::FinishedTurnQueue)
    }
}

#[derive(Clone, Copy)]
pub enum BattleClientState {
    SelectMoves,
    ProcessTurnQueue,
    FinishedTurnQueue,
}

pub trait BattleEndpoint<ID: Sized + Copy + core::fmt::Debug + core::fmt::Display + PartialEq> {
    fn give_client(&mut self, message: ServerMessage<ID>);
}

pub trait BattleClient<ID: Sized + Copy + core::fmt::Debug + core::fmt::Display + PartialEq>: BattleEndpoint<ID> {
    fn give_server(&mut self) -> Option<ClientMessage>;
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
pub enum BattleClientMove<ID: Sized + Copy + core::fmt::Debug + core::fmt::Display + PartialEq> {
    Miss,
    TargetHP(f32),
    UserHP(f32), // dont heal the target
    Effective(Effective),
    Critical,
    StatStage(StatType, Stage),
    Faint(PokemonIndex<ID>), // target that is fainting
    /* #[deprecated(note = "only needs to be sent to one client")] */ GainExp(Experience),
    Fail,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum BattleClientAction<ID: Sized + Copy + core::fmt::Debug + core::fmt::Display + PartialEq> {
    Move(MoveRef, Vec<(MoveTargetInstance, Vec<BattleClientMove<ID>>)>),
    Switch(usize, Option<UnknownPokemon>),
    UseItem(ItemRef, MoveTargetInstance),
}