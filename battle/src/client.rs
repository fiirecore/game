use core::fmt::{Debug, Display};

use crate::message::{ServerMessage, ClientMessage};

pub mod local;
pub mod action;

pub trait BattleEndpoint<ID: Sized + Copy + Debug + Display + PartialEq> {
    fn give_client(&mut self, message: ServerMessage<ID>);
}

pub trait BattleClient<ID: Sized + Copy + Debug + Display + PartialEq>: BattleEndpoint<ID> {
    fn give_server(&mut self) -> Option<ClientMessage>;
}