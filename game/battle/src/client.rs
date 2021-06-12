// use std::{cell::RefCell, sync::atomic::{AtomicBool, Ordering}};

use crate::message::{ServerMessage, ClientMessage};

pub struct LocalBattleClient {
    client: Box<dyn BattleClient>,
    state: BattleClientState,
    pub forfeit: bool,
}

impl LocalBattleClient {
    pub fn new(client: Box<dyn BattleClient>) -> Self {
        Self {
            client,
            state: BattleClientState::SelectMoves,
            forfeit: false,
        }
    }
    pub fn send(&mut self, message: ServerMessage) {
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
            match message {
                ClientMessage::FinishedTurnQueue => if matches!(self.state, BattleClientState::ProcessTurnQueue) {
                    self.state = BattleClientState::FinishedTurnQueue;
                },
                ClientMessage::Forfeit => self.forfeit = true,
                _ => (),
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

pub trait BattleEndpoint {
    fn give_client(&mut self, message: ServerMessage);
}

pub trait BattleClient: BattleEndpoint {
    fn give_server(&mut self) -> Option<ClientMessage>;
}