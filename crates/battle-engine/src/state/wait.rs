use std::{marker::PhantomData, ops::Deref};

use bevy_ecs::prelude::*;
use pokedex::{item::Item, moves::Move, pokemon::Pokemon};

use super::select::SelectRange;

#[derive(Default)]
pub struct Wait<ID, P, M, I>(
    PhantomData<ID>,
    PhantomData<P>,
    PhantomData<M>,
    PhantomData<I>,
);

impl<
        ID: Component + Eq + Clone + std::fmt::Debug + std::hash::Hash,
        P: Deref<Target = Pokemon> + Component,
        M: Deref<Target = Move> + Component + Clone,
        I: Deref<Target = Item> + Component,
    > Wait<ID, P, M, I>
{
    pub fn add(schedule: &mut Schedule) {
        let start_begin = SystemSet::on_enter(super::BattlePlayerState::<ID>::WaitToStart)
            .with_system(Self::start_begin);

        schedule.add_system_set_to_stage(crate::UPDATE, start_begin);

        let idle_update = SystemSet::on_update(super::BattlePlayerState::<ID>::Idle).with_system(Self::idle_update);

        schedule.add_system_set_to_stage(crate::UPDATE, idle_update);

    }

    pub fn start_begin(mut state: ResMut<State<super::BattlePlayerState<ID>>>) {
        state.push(super::BattlePlayerState::<ID>::Opening);
        state.push(super::BattlePlayerState::<ID>::Introduction);
    }

    // pub fn idle_begin(
    //     mut state: ResMut<State<super::BattlePlayerState<ID>>>,
    // ) {
        
    // }

    pub fn idle_update(
        mut commands: Commands,
        mut state: ResMut<State<super::BattlePlayerState<ID>>>,
        mut reader: EventReader<super::StateEvent>,
        local: NonSend<crate::ui::view::GuiLocalPlayer<ID, P, M, I>>,
    ) {
        if reader.iter().any(|e| e == &super::StateEvent::ShouldSelect) {
            match state.push(super::BattlePlayerState::<ID>::Select) {
                Ok(()) => {
                    commands.insert_resource(SelectRange(0..local.player.active.len()));
                    state.pop();
                }
                Err(err) => todo!(),
            }
        }
    }
}
