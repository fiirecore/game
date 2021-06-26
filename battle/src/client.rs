// use std::{cell::RefCell, sync::atomic::{AtomicBool, Ordering}};

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