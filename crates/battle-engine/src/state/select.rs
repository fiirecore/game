use std::{
    marker::PhantomData,
    ops::{Deref, Range},
    rc::Rc,
};

use battle::{
    endpoint::MpscClient,
    moves::BattleMove,
    pokemon::{Indexed, PokemonIdentifier},
    prelude::{BattleType, ClientMessage},
};
use bevy_ecs::prelude::*;
use log::{debug, warn};
use pokedex::{
    engine::{
        graphics::Color,
        math::{vec2, Vec2},
        utils::{Entity, HashMap},
    },
    gui::{bag::BagGui, party::PartyGui},
    item::{usage::ItemExecution, Item},
    moves::{Move, MoveTarget},
    pokemon::Pokemon,
    PokedexClientData,
};

use crate::{
    ui::{
        panels::{BattlePanel, BattlePanels},
        pokemon::bounce::PlayerBounce,
        view::{GuiLocalPlayer, GuiRemotePlayer}, background::BattleBackground,
    },
    view::PlayerView,
    Delta,
};

pub struct SelectState<ID, P, M, I>(
    PhantomData<ID>,
    PhantomData<P>,
    PhantomData<M>,
    PhantomData<I>,
);

pub struct SelectRange(pub Range<usize>);

impl<
        ID: Component + Clone + Eq + std::hash::Hash + std::fmt::Debug,
        P: Deref<Target = Pokemon> + Component,
        M: Deref<Target = Move> + Component + Clone,
        I: Deref<Target = Item> + Component + Clone,
    > SelectState<ID, P, M, I>
{
    const STATE: super::BattlePlayerState<ID> = super::BattlePlayerState::Select;

    pub fn add(schedule: &mut Schedule) {
        let update = SystemSet::on_update(Self::STATE).with_system(Self::update);
        let end = SystemSet::on_exit(Self::STATE).with_system(Self::end);

        schedule.add_system_set_to_stage(crate::UPDATE, update);
        schedule.add_system_set_to_stage(crate::UPDATE, end);
    }

    pub fn end(mut commands: Commands) {
        commands.remove_resource::<SelectRange>();
    }

    pub fn update(
        delta: Res<Delta>,
        mut panel: NonSendMut<BattlePanel<M>>,
        local: NonSend<GuiLocalPlayer<ID, P, M, I>>,
        remotes: NonSend<HashMap<ID, GuiRemotePlayer<ID, P>>>,
        range: ResMut<SelectRange>,
        mut bounce: ResMut<PlayerBounce>,
        mut ctx: NonSendMut<crate::Ctx>,
        mut eng: NonSendMut<crate::Eng>,
        dex: NonSend<Rc<PokedexClientData>>,
        bag: NonSendMut<BagGui>,
        party: NonSendMut<PartyGui>,
        pokedex: Res<crate::Pokedex<P>>,
        client: Res<MpscClient<ID>>,
    ) {
        let delta = delta.0;
        let ctx = &mut **ctx;
        let eng = &mut **eng;
        let dex = &**dex;
        let pokedex = pokedex.get();
        bounce.update(delta);
        let SelectRange(range) = &mut *range;
        let current = range.start;
        match current < range.end {
            true => {
                match local.player.active.get(current) {
                    Some(index) => match index {
                        Some(index) => {
                            let pokemon = &local.player.pokemon[*index];
                            match panel.alive() {
                                true => {
                                    // Checks if a move is queued from an action done in the GUI

                                    if bag.alive() {
                                        bag.input(ctx, eng, &mut local.bag);
                                        if let Some(item) =
                                            bag.take_selected_despawn(&mut local.bag)
                                        {
                                            match item.category {
                                                pokedex::item::ItemCategory::Pokeballs => {
                                                    panel.active = BattlePanels::Target(
                                                        MoveTarget::Opponent,
                                                        Some(item.id),
                                                    );
                                                }
                                                _ => match &item.usage.execute {
                                                ItemExecution::Actions(..) => todo!(),
                                                // ItemExecution::Script => {
                                                //     todo!("user targeting")
                                                // }
                                                ItemExecution::None => {
                                                    debug!("make items with usage kind None unusable")
                                                }
                                                // ItemUsageKind::Pokeball => ,
                                                // ItemUsageKind::Script(..) => ,
                                                // ItemUsageKind::None => ,
                                            },
                                            }
                                        }
                                    } else if party.alive() {
                                        party.input(
                                            ctx,
                                            eng,
                                            dex,
                                            pokedex,
                                            local.player.pokemon.as_mut_slice(),
                                        );
                                        party.update(delta);
                                        if let Some(selected) = party.take_selected() {
                                            party.despawn();
                                            client.send(ClientMessage::Move(
                                                current,
                                                BattleMove::Switch(selected),
                                            ));
                                            range.next();
                                            panel.despawn();
                                        }
                                    } else if let Some(panels) = panel.input(ctx, eng, pokemon) {
                                        match panels {
                                        BattlePanels::Main => {
                                            match panel.battle.cursor {
                                                0 => panel.active = BattlePanels::Fight,
                                                1 => bag.spawn(),
                                                2 => if let Err(err) = party.spawn(dex, pokedex, &local.player.pokemon, Some(false), true) {
                                                    warn!("Error opening party gui: {}", err);
                                                },
                                                3 => if matches!(local.data.type_, BattleType::Wild) {
                                                    client.send(ClientMessage::Forfeit);
                                                },
                                                _ => unreachable!(),
                                            }
                                        }
                                        BattlePanels::Fight => match pokemon.moves.get(panel.fight.moves.cursor) {
                                            Some(instance) => match (!instance.is_empty()).then(|| instance.0.clone()) {
                                                Some(move_ref) => {
                                                    match move_ref.target {
                                                        MoveTarget::Opponent | MoveTarget::Any => {
                                                            let p = &remotes.values().next().unwrap().player;
                                                            panel.target(p as &dyn PlayerView<ID, P, M, I>);
                                                            panel.active = BattlePanels::Target(move_ref.target, None);
                                                        },
                                                        MoveTarget::Ally | MoveTarget::UserOrAlly => {
                                                            panel.target(&local.player);
                                                            panel.active = BattlePanels::Target(move_ref.target, None);
                                                        }
                                                        _ => {
                                                            client.send(
                                                                ClientMessage::Move(
                                                                    current,
                                                                    BattleMove::Move(
                                                                        panel.fight.moves.cursor,
                                                                        None,
                                                                    )
                                                                )
                                                            );
                                                            range.next();
                                                            panel.despawn();
                                                        }
                                                    }
                                                }
                                                None => warn!("Pokemon is out of Power Points for this move!"),
                                            }
                                            None => warn!("Could not get move at cursor!"),
                                        }
                                        BattlePanels::Target(target, item) => {
                                            client.send(
                                                ClientMessage::Move(
                                                    current,
                                                    match item {
                                                        Some(item) => BattleMove::UseItem(Indexed(
                                                            match target {
                                                                MoveTarget::Opponent => PokemonIdentifier(remotes.keys().next().unwrap().clone(), panel.targets.cursor),
                                                                _ => unreachable!(),
                                                            },
                                                            item,
                                                        )
                                                        ),
                                                        None => BattleMove::Move(panel.fight.moves.cursor, Some(PokemonIdentifier(remotes.keys().next().unwrap().clone(), panel.targets.cursor))),
                                                    }
                                                )
                                            );
                                            range.next();
                                            panel.despawn();
                                        }
                                    }
                                    }
                                }
                                false => {
                                    panel.user(pokemon);
                                    panel.spawn();
                                }
                            }
                        }
                        None => {
                            range.next();
                        }
                    },
                    None => {
                        panel.despawn();
                    }
                }
            }
            false => panel.despawn(),
        }
    }

    pub fn draw(
        mut party: NonSendMut<PartyGui>,
        mut bag: NonSendMut<BagGui>,
        local: NonSend<GuiLocalPlayer<ID, P, M, I>>,
        mut bounce: ResMut<PlayerBounce>,
        range: Res<SelectRange>,
        mut ctx: NonSendMut<crate::Ctx>,
        mut eng: NonSendMut<crate::Eng>,
        background: NonSend<BattleBackground>,
        mut panel: NonSendMut<BattlePanel<M>>,
    ) {
        let ctx = &mut **ctx;
        let eng = &mut **eng;

        if bag.alive() {
            bag.draw(ctx, eng);
        } else if party.alive() {
            party.draw(ctx, eng);
        } else {
            for (current, active) in local.renderer.iter().enumerate() {
                if current == range.0.start {
                    active
                        .pokemon
                        .draw(ctx, Vec2::new(0.0, bounce.offset), Color::WHITE);
                    active.status.draw(ctx, eng, 0.0, -bounce.offset);
                } else {
                    active.pokemon.draw(ctx, vec2(0.0, 0.0), Color::WHITE);
                    active.status.draw(ctx, eng, 0.0, 0.0);
                }
            }
            background.panel.draw(ctx, 0.0, crate::ui::PANEL_Y, Default::default());
            panel.draw(ctx, eng);
        }
    }
}
