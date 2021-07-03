use crate::message::{ServerMessage, ClientMessage};

use super::BattleClient;

pub struct LocalBattleClient<ID: Sized + Copy + core::fmt::Debug + core::fmt::Display> {
    client: Box<dyn BattleClient<ID>>,
    finished: bool,
}

impl<ID: Sized + Copy + core::fmt::Debug + core::fmt::Display + PartialEq> LocalBattleClient<ID> {
    pub fn new(client: Box<dyn BattleClient<ID>>) -> Self {
        Self {
            client,
            finished: false,
        }
    }
    pub fn send(&mut self, message: ServerMessage<ID>) {
        self.client.give_client(message)
    }
    pub fn receive(&mut self) -> Option<ClientMessage> {
        let message = self.client.give_server();
        if let Some(message) = &message {
            if let ClientMessage::FinishedTurnQueue = message {
                self.finished = true;
            }
        }
        message
    }
    pub fn finished_turn(&self) -> bool {
        self.finished
    }
    pub fn new_turn(&mut self) {
        self.finished = false;
    }
}