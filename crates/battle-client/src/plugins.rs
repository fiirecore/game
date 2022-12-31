use std::marker::PhantomData;

use bevy::prelude::*;
use iyes_loopless::prelude::*;

use crate::{BattleClientState, systems::*, resources::{CurrentRemote, GuiLocalPlayer}};

mod transition;

#[derive(Default)]
pub struct BattleClientPlugin<ID: Clone + PartialEq + Send + Sync + 'static> {
    _p: PhantomData<ID>,
}

impl<ID: std::fmt::Debug + Clone + PartialEq + Send + Sync + 'static> Plugin for BattleClientPlugin<ID> {
    fn build(&self, app: &mut App) {
        app
            // ;
            .init_resource::<CurrentRemote>()
            // ;
            .add_loopless_state(Option::<BattleClientState>::None)
            // ;
            .add_system(process_events::<ID>.run_if(in_battle))
            .add_system(select::<ID>.run_if_resource_exists::<GuiLocalPlayer<ID>>())
            // .add_system
            // ;
            .add_plugin(transition::TransitionPlugin)

            // .add_system()
            // ;
            ;
    }
}