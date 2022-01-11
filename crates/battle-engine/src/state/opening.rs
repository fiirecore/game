use std::{marker::PhantomData, ops::Deref, rc::Rc};

use bevy_ecs::prelude::*;

use battle::prelude::BattleType;
use pokedex::{
    engine::utils::HashMap, item::Item, moves::Move, pokemon::Pokemon, PokedexClientData,
};

use crate::{
    transition::opener::BattleOpenerManager,
    ui::view::{GuiLocalPlayer, GuiRemotePlayer},
};

#[derive(Default)]
pub struct OpeningState<ID, P, M, I>(
    PhantomData<ID>,
    PhantomData<P>,
    PhantomData<M>,
    PhantomData<I>,
);

impl<
        ID: Component + Clone + Eq + std::hash::Hash + std::fmt::Debug,
        P: Deref<Target = Pokemon> + Component,
        M: Deref<Target = Move> + Component + Clone,
        I: Deref<Target = Item> + Component,
    > OpeningState<ID, P, M, I>
{
    const STATE: super::BattlePlayerState<ID> = super::BattlePlayerState::Opening;

    pub fn add(schedule: &mut Schedule, world: &mut World) {
        let begin = SystemSet::on_enter(Self::STATE).with_system(Self::begin);
        let update = SystemSet::on_update(Self::STATE).with_system(Self::update);
        schedule.add_system_set_to_stage(crate::UPDATE, begin);
        schedule.add_system_set_to_stage(crate::UPDATE, update);
    }

    pub fn begin(
        mut manager: NonSendMut<BattleOpenerManager>,
        dex: NonSend<Rc<PokedexClientData>>,
        mut trainer: NonSendMut<crate::transition::trainer::PokemonCount>,
        local: NonSend<GuiLocalPlayer<ID, P, M, I>>,
        remotes: NonSend<HashMap<ID, GuiRemotePlayer<ID, P>>>,
    ) {
        manager.begin(&dex, &local, &remotes);
        if !matches!(local.data.type_, BattleType::Wild) {
            trainer.spawn(
                local.player.pokemon.len(),
                remotes.values().next().unwrap().player.pokemon.len(),
            );
        }
    }

    pub fn update(
        mut manager: NonSendMut<BattleOpenerManager>,
        mut state: ResMut<State<super::BattlePlayerState<ID>>>,
        mut delta: Res<crate::Delta>,
    ) {
        manager.update::<ID, P, M, I>(&mut *state, delta.0)
    }
}
