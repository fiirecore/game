use std::{ops::Range, sync::Arc};

use battle::{party::PlayerParty, pokedex::{item::bag::OwnedBag, pokemon::owned::OwnedPokemon}, data::BattleData, select::ClientAction, endpoint::{MpscConnection, BattleEndpoint}, message::{ClientMessage, ServerMessage}};
use bevy::prelude::*;
use rand::rngs::SmallRng;

use crate::components::BattleTrainer;

#[derive(Resource, Deref, DerefMut)]
pub struct BattleRandom(pub SmallRng);

#[derive(Default, Resource)]
pub struct BattleBackground {
    pub background: Handle<Image>,
    pub floor: Handle<Image>,
}

#[derive(Resource)]
pub struct GuiLocalPlayer<ID> {
    pub player: PlayerParty<ID, usize, OwnedPokemon, BattleTrainer>,
    pub bag: OwnedBag,
    pub data: BattleData,
}

#[derive(Resource)]
pub struct PlayerChannel<ID, T>(pub Arc<dyn BattleEndpoint<ClientMessage<ID>, ServerMessage<ID, T>> + Send + Sync + 'static>);

#[derive(Default, Resource)]
pub struct CurrentRemote {
    pub data: Option<Entity>,
    pub space: Option<Entity>,
}

#[derive(Resource, Deref, DerefMut)]
pub struct Results<ID>(pub Vec<ClientAction<ID>>);