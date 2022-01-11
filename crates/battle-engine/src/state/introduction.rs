use std::{marker::PhantomData, ops::Deref, rc::Rc};

use bevy_ecs::prelude::*;

use battle::prelude::BattleType;
use pokedex::{
    engine::{gui::MessageBox, utils::HashMap},
    item::Item,
    moves::Move,
    pokemon::Pokemon,
    PokedexClientData,
};

use crate::{
    transition::introduction::BattleIntroductionManager,
    ui::view::{GuiLocalPlayer, GuiRemotePlayer},
};

#[derive(Default)]
pub struct IntroductionState<ID, P, M, I>(
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
    > IntroductionState<ID, P, M, I>
{
    const STATE: super::BattlePlayerState<ID> = super::BattlePlayerState::Introduction;

    pub fn add(schedule: &mut Schedule, world: &mut World) {
        let begin = SystemSet::on_enter(Self::STATE).with_system(Self::begin);
        let update = SystemSet::on_update(Self::STATE).with_system(Self::update);
        let update = SystemSet::on_exit(Self::STATE).with_system(Self::end);
        schedule.add_system_set_to_stage(crate::UPDATE, begin);
        schedule.add_system_set_to_stage(crate::UPDATE, update);
    }

    pub fn begin(
        mut manager: NonSendMut<BattleIntroductionManager>,
        mut text: ResMut<MessageBox>,
        dex: NonSend<Rc<PokedexClientData>>,
        local: NonSend<GuiLocalPlayer<ID, P, M, I>>,
        remotes: NonSend<HashMap<ID, GuiRemotePlayer<ID, P>>>,
    ) {
        manager.begin(&dex, &local, &remotes, &mut text);
    }

    pub fn update(
        mut manager: NonSendMut<BattleIntroductionManager>,
        mut state: ResMut<State<super::BattlePlayerState<ID>>>,
        mut text: ResMut<MessageBox>,
        mut trainer: NonSendMut<crate::transition::trainer::PokemonCount>,
        local: NonSend<GuiLocalPlayer<ID, P, M, I>>,
        remotes: NonSend<HashMap<ID, GuiRemotePlayer<ID, P>>>,
        delta: Res<crate::Delta>,
        mut ctx: NonSendMut<crate::Ctx>,
        mut eng: NonSendMut<crate::Eng>,
    ) {
        manager.update(
            &mut state,
            &mut ctx,
            &mut eng,
            delta.0,
            &mut local,
            remotes.values_mut().next().unwrap(),
            &mut text,
        );
        trainer.update(delta.0);
        if text.page() > 0 && !trainer.ending() && !matches!(local.data.type_, BattleType::Wild) {
            trainer.end();
        }
    }

    pub fn end(
        mut manager: NonSendMut<BattleIntroductionManager>,
        mut text: ResMut<MessageBox>,
        mut trainer: NonSendMut<crate::transition::trainer::PokemonCount>,
    ) {
        manager.end(&mut text);
        trainer.despawn();
    }
}
