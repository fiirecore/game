use std::ops::Range;

use bevy::prelude::Entity;
use pokengine::pokedex::{item::bag::InitBag, pokemon::owned::OwnedPokemon};

use battle::{pokemon::remote::InitUnknownPokemon, prelude::BattleData};

use crate::BattleTrainer;

type PlayerParty<ID, P> = battle::party::PlayerParty<ID, usize, P, BattleTrainer>;

pub type GuiRemotePlayer<ID> = PlayerParty<ID, Option<InitUnknownPokemon>>;

