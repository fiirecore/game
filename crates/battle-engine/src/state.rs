use std::ops::Deref;

use bevy_ecs::{prelude::*, event::Events};

use pokedex::{item::Item, moves::Move, pokemon::Pokemon};

mod introduction;
mod opening;
mod wait;
mod select;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BattlePlayerState<ID> {
    WaitToStart,
    Opening,
    Introduction,
    /// Waiting to select, waiting for server response, etc.
    Idle,
    Select,
    Moving, //(MoveQueue<ID, M>),
    PlayerEnd,
    GameEnd(Option<ID>),
    Closing,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug,)]
pub enum StateEvent {
    ShouldSelect,
}

pub struct BattleStatePlugin;

impl BattleStatePlugin {
    pub fn setup<
        ID: Component + Eq + Clone + std::fmt::Debug + std::hash::Hash,
        P: Deref<Target = Pokemon> + Component,
        M: Deref<Target = Move> + Component + Clone,
        I: Deref<Target = Item> + Component,
    >(
        schedule: &mut Schedule,
        world: &mut World,
    ) {
        world.insert_resource(State::new(BattlePlayerState::<ID>::WaitToStart));

        schedule.add_stage(crate::PROCESS, SystemStage::parallel());
        schedule.add_stage_after(crate::PROCESS, crate::UPDATE, SystemStage::parallel());
        schedule.add_stage_after(crate::UPDATE, crate::DRAW, SystemStage::single_threaded());

        schedule
            .add_system_set_to_stage(crate::UPDATE, State::<BattlePlayerState<ID>>::get_driver());

        let mut events = SystemStage::parallel();

        events.add_system(Events::<StateEvent>::update_system);

        wait::Wait::<ID, P, M, I>::add(schedule);

        schedule.add_system_set_to_stage(
            crate::UPDATE,
            SystemSet::on_enter(BattlePlayerState::<ID>::WaitToStart)
                .with_system(wait::Wait::<ID, P, M, I>::start_begin),
        );

        opening::OpeningState::<ID, P, M, I>::add(schedule, world);

        introduction::IntroductionState::<ID, P, M, I>::add(schedule, world);
    }
}
