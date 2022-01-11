use std::ops::Deref;

use bevy_ecs::prelude::*;

use battle::{moves::{ClientMove, ClientMoveAction, damage::ClientDamage}, pokemon::Indexed, prelude::{BattleType, ClientMessage}, endpoint::MpscClient};
use log::warn;
use pokedex::{moves::Move, engine::{gui::MessageBox, utils::{Reset, HashMap, Completable}}, pokemon::Pokemon, item::{Item, ItemCategory, usage::ItemExecution}, types::Effective, PokedexClientData, gui::party::PartyGui};

use crate::{action::{BattleClientGuiAction, BattleClientGuiCurrent, MoveQueue}, view::{GuiPokemonView, PlayerView}, ui::{view::{GuiLocalPlayer, GuiRemotePlayer}, panels::level::LevelUpMovePanel}, Delta};

pub fn queue<
    ID: Clone + Eq + std::fmt::Debug + std::hash::Hash + Component,
    P: Deref<Target = Pokemon> + Clone,
    M: Deref<Target = Move> + Clone + Component,
    I: Deref<Target = Item> + Clone,
>(
    queue: ResMut<MoveQueue<ID, M>>,
    mut state: ResMut<State<crate::state::BattlePlayerState<ID>>>,
    mut text: ResMut<MessageBox>,
    mut local: NonSendMut<GuiLocalPlayer<ID, P, M, I>>,
    mut remotes: NonSendMut<HashMap<ID, GuiRemotePlayer<ID, P>>>,
    pokedex: Res<crate::Pokedex<P>>,
    movedex: Res<crate::Movedex<M>>,
    itemdex: Res<crate::Itemdex<I>>,
    dex: NonSend<PokedexClientData>,
    delta: Res<Delta>,
    mut ctx: NonSendMut<crate::Ctx>,
    mut eng: NonSendMut<crate::Eng>,
    mut level_up: NonSendMut<LevelUpMovePanel<M>>,
    party: NonSend<PartyGui>,
    client: Res<MpscClient<ID>>,
) {
    let pokedex = pokedex.get();
    let movedex = movedex.get();
    let itemdex = itemdex.get();
    let ctx = &mut **ctx;
    let eng = &mut **eng;
    let delta = delta.0;
    let dex = &*dex;
    match &mut queue.current {
        None => {
            match queue.actions.pop_front() {
                None => {
                    // self.messages.send(ClientMessage::FinishedTurnQueue);
                    state.push(crate::state::BattlePlayerState::Idle);
                    state.pop();
                }
                Some(Indexed(user_id, action)) => {
                    // log::trace!("set current");

                    text.pages.clear();
                    text.reset();

                    let user = match user_id.team() == local.player.id() {
                        true => Some((
                            &mut local.player as &mut dyn PlayerView<ID, P, M, I>,
                            &mut local.renderer,
                        )),
                        false => remotes
                            .get_mut(user_id.team())
                            .map(|p| (&mut p.player as _, &mut p.renderer)),
                    };

                    match user {
                        Some((user, user_ui)) => {
                            if user.active(user_id.index()).is_some() || !action.requires_user() {
                                if let Some(action) = match action {
                                    BattleClientGuiAction::Action(action) => {
                                        match action {
                                            ClientMove::<ID>::Move(pokemon_move, pp, targets) => {
                                                // log::trace!("set current: client move");

                                                match movedex.try_get(&pokemon_move) {
                                                    Some(pokemon_move) => {
                                                        {
                                                            if let Some(user_active) =
                                                                user.active_mut(user_id.index())
                                                            {
                                                                user_active.decrement_pp(
                                                                    &pokemon_move,
                                                                    pp,
                                                                );

                                                                crate::ui::text::on_move(
                                                                    &mut text,
                                                                    &pokemon_move,
                                                                    user_active.name(),
                                                                );
                                                            }

                                                            // user_active.decrement_pp(pp);
                                                        }

                                                        drop(user);
                                                        drop(user_ui);

                                                        let mut faint = Vec::new();

                                                        for Indexed(target_id, action) in &targets {
                                                            let userui = &mut local.renderer
                                                                [target_id.index()];

                                                            let target = match target_id.team() == local.player.id() {
                                                                    true => local.player.active_mut(target_id.index()).map(|p| (p as &mut dyn GuiPokemonView<P, M, I>, userui)),
                                                                    false => remotes.get_mut(target_id.team()).map(|remote| {
                                                                        let ui = &mut remote.renderer[target_id.index()];
                                                                        remote.player.active_mut(target_id.index()).map(|p| (p as _, ui))
                                                                    }).flatten(),
                                                                };

                                                            if let Some((target, target_ui)) =
                                                                target
                                                            {
                                                                match *action {
                                                                            ClientMoveAction::SetHP(result) => {
                                                                                target.set_hp(result.damage());
                                                                                if let ClientDamage::Result(result) = result {
                                                                                    match result.damage > 0.0 {
                                                                                        true => target_ui.pokemon.flicker(),
                                                                                        false => faint.push(target_id),
                                                                                    }
                                                                                    if result.effective != Effective::Effective {
                                                                                        crate::ui::text::on_effective(&mut text, &result.effective)
                                                                                    }
                                                                                    if result.crit {
                                                                                        crate::ui::text::on_crit(&mut text);
                                                                                    }
                                                                                }
                                                                            },
                                                                            ClientMoveAction::Error => crate::ui::text::on_fail(&mut text, vec![format!("{} cannot use move", target.name()), format!("{}, as there was an error.", pokemon_move.name)]),
                                                                            ClientMoveAction::Miss => crate::ui::text::on_miss(&mut text, target.name()),
                                                                            ClientMoveAction::SetExp(experience, level) => {
                                                                                let previous = target.level();
                                                                                target.set_level(level);
                                                                                target.set_exp(experience);
                                                                                if let Some(user_pokemon) = target.instance() {
                                                                                    let moves = user_pokemon.on_level_up(movedex, previous).flat_map(|id| movedex.try_get(id)).collect();
                                                                                    queue.actions.push_front(Indexed(target_id.clone(), BattleClientGuiAction::SetExp(previous, experience, moves)));
                                                                                }
                                                                            }
                                                                            ClientMoveAction::Flinch => crate::ui::text::on_flinch(&mut text, target.name()),
                                                                            ClientMoveAction::AddStat(stat, stage) => crate::ui::text::on_stat_stage(&mut text, target.name(), stat, stage),
                                                                            ClientMoveAction::Ailment(ailment) => {
                                                                                target.set_ailment(ailment);
                                                                                crate::ui::text::on_status(&mut text, target.name(), ailment.ailment);
                                                                            }
                                                                        }

                                                                match target.instance() {
                                                                    Some(i) => {
                                                                        target_ui.status.update_gui(
                                                                            Some(i),
                                                                            None,
                                                                            false,
                                                                        )
                                                                    }
                                                                    None => target_ui
                                                                        .status
                                                                        .update_gui_view(
                                                                            Some(target as _),
                                                                            None,
                                                                            false,
                                                                        ),
                                                                }
                                                            } else {
                                                                // target_ui.status.update_gui(None, None, false);
                                                            }
                                                        }

                                                        for target_id in faint {
                                                            queue.actions.push_front(Indexed(
                                                                target_id.clone(),
                                                                BattleClientGuiAction::Faint,
                                                            ))
                                                        }

                                                        Some(BattleClientGuiCurrent::Move(targets))
                                                    }
                                                    None => None,
                                                }
                                            }
                                            ClientMove::UseItem(Indexed(target, item)) => {
                                                if let Some(item) = itemdex.try_get(&item) {
                                                    if let Some(pokemon) = match item.category {
                                                        ItemCategory::Pokeballs => remotes
                                                            .get(target.team())
                                                            .map(|p| {
                                                                p.player.active(target.index())
                                                            })
                                                            .flatten()
                                                            .map(|p| p as _),
                                                        _ => match &item.usage.execute {
                                                            ItemExecution::Actions(..) => {
                                                                user.active(target.index())
                                                            }
                                                            ItemExecution::None => None,
                                                        },
                                                    } {
                                                        if let ItemCategory::Pokeballs =
                                                            &item.category
                                                        {
                                                            // self.messages.push(ClientMessage::RequestPokemon(index));
                                                            queue.actions.push_front(Indexed(
                                                                target.clone(),
                                                                BattleClientGuiAction::Catch,
                                                            ));
                                                        }
                                                        crate::ui::text::on_item(
                                                            &mut text,
                                                            pokemon.name(),
                                                            &item,
                                                        );
                                                    }
                                                    Some(BattleClientGuiCurrent::UseItem(target))
                                                } else {
                                                    None
                                                }
                                            }
                                            ClientMove::Switch(index) => {
                                                let coming = user
                                                    .pokemon(index)
                                                    .map(|v| v.name())
                                                    .unwrap_or("Unknown");
                                                crate::ui::text::on_switch(
                                                    &mut text,
                                                    user.active(user_id.index())
                                                        .map(|v| v.name())
                                                        .unwrap_or("Unknown"),
                                                    coming,
                                                );
                                                Some(BattleClientGuiCurrent::Switch(index))
                                            }
                                        }
                                    }
                                    BattleClientGuiAction::Faint => {
                                        let is_player = user_id.team() == user.id();
                                        let target = user.active_mut(user_id.index()).unwrap();
                                        target.set_hp(0.0);
                                        crate::ui::text::on_faint(
                                            &mut text,
                                            matches!(local.data.type_, BattleType::Wild),
                                            is_player,
                                            target.name(),
                                        );
                                        user_ui[user_id.index()].pokemon.faint();
                                        Some(BattleClientGuiCurrent::Faint)
                                    }
                                    BattleClientGuiAction::Catch => {
                                        match remotes.get_mut(user_id.team()) {
                                            Some(remote) => {
                                                if let Some(pokemon) =
                                                    remote.player.active(user_id.index())
                                                {
                                                    crate::ui::text::on_catch(
                                                        &mut text,
                                                        crate::view::BasePokemonView::name(pokemon),
                                                    );
                                                }
                                                // if let Some(pokemon) = pokemon {
                                                remote.player.replace(user_id.index(), None);
                                                let renderer =
                                                    &mut remote.renderer[user_id.index()];
                                                renderer
                                                    .status
                                                    .update_gui_view::<P, M, I>(None, None, false);
                                                renderer.pokemon.new_pokemon(&dex, None);
                                                // }
                                                Some(BattleClientGuiCurrent::Catch)
                                            }
                                            None => None,
                                        }
                                    }
                                    BattleClientGuiAction::Replace(new) => {
                                        crate::ui::text::on_replace(
                                            &mut text,
                                            user.name(),
                                            new.map(|index| user.pokemon(index).map(|v| v.name()))
                                                .flatten(),
                                        );
                                        user.replace(user_id.index(), new);
                                        Some(BattleClientGuiCurrent::Replace(
                                            user_id.index(),
                                            false,
                                        ))
                                    }
                                    // To - do: experience spreading
                                    BattleClientGuiAction::SetExp(previous, experience, moves) => {
                                        match user.active_mut(user_id.index()) {
                                            Some(pokemon) => {
                                                crate::ui::text::on_gain_exp(
                                                    &mut text,
                                                    pokemon.name(),
                                                    experience,
                                                    pokemon.level(),
                                                );
                                                let status = &mut user_ui[user_id.index()].status;
                                                match pokemon.instance() {
                                                    Some(p) => status.update_gui(
                                                        Some(p),
                                                        Some(previous),
                                                        false,
                                                    ),
                                                    None => status.update_gui_view(
                                                        Some(pokemon),
                                                        Some(previous),
                                                        false,
                                                    ),
                                                }
                                                queue.actions.push_front(Indexed(
                                                    user_id.clone(),
                                                    BattleClientGuiAction::LevelUp(moves),
                                                ));
                                                Some(BattleClientGuiCurrent::SetExp)
                                            }
                                            None => None,
                                        }
                                    }
                                    BattleClientGuiAction::LevelUp(moves) => {
                                        match user
                                            .active_mut(user_id.index())
                                            .map(|v| v.instance())
                                            .flatten()
                                        {
                                            Some(instance) => match moves.is_empty() {
                                                false => {
                                                    level_up.spawn(
                                                        instance,
                                                        &mut text,
                                                        moves,
                                                    );
                                                    Some(BattleClientGuiCurrent::LevelUp)
                                                }
                                                true => None,
                                            },
                                            None => None,
                                        }
                                    } // ClientMove::Catch(index) => {
                                      //     if let Some(target) = match index.team {
                                      //         Team::Player => &user.active[index.active],
                                      //         Team::Opponent => &other.active[index.active],
                                      //     }.pokemon.as_ref() {
                                      //         crate::ui::text::on_catch(text, target);
                                      //     }
                                      // }
                                } {
                                    queue.current = Some(Indexed(user_id, action));
                                } else {
                                    // self.update(ctx, eng, dex, pokedex, movedex, itemdex, delta);
                                }
                            }
                        }
                        None => log::warn!("cannot get user {:?}", user_id),
                    }
                }
            }
        }
        Some(Indexed(user_id, action)) => {
            // log::trace!("update current");

            let user = if user_id.team() == local.player.id() {
                Some((
                    &mut local.player as &mut dyn PlayerView<ID, P, M, I>,
                    &mut local.renderer,
                ))
            } else {
                remotes
                    .get_mut(user_id.team())
                    .map(|p| (&mut p.player as _, &mut p.renderer))
            };

            match user {
                Some((user, user_ui)) => match action {
                    BattleClientGuiCurrent::Move(targets) => {
                        // log::trace!("update current: client move");

                        match text.finished() {
                            false => text.update(ctx, eng, delta),
                            true =>
                            /*if text.page() > 0 || text.waiting() */
                            {
                                //&& user_ui[instance.pokemon.index].renderer.moves.finished() {

                                // run through hp update and flicker

                                let mut not_done = false;

                                for Indexed(location, ..) in targets {
                                    if let Some(target_ui) = if location.team() == local.player.id()
                                    {
                                        Some(&mut local.renderer)
                                    } else {
                                        remotes
                                            .get_mut(location.team())
                                            .map(|p| &mut p.renderer)
                                    } {
                                        let ui = &mut target_ui[location.index()];

                                        // while ui.pokemon.flicker.flickering() || ui.status.health_moving() {
                                        ui.pokemon.flicker.update(delta);
                                        ui.status.update_hp(delta);
                                        // }

                                        if ui.pokemon.flicker.flickering()
                                            || ui.status.health_moving()
                                        {
                                            not_done = true;
                                        }
                                    }
                                }

                                if !not_done {
                                    queue.current = None;
                                }
                            }
                        }
                    }
                    BattleClientGuiCurrent::Switch(new) => match text.finished() {
                        false => {
                            text.update(ctx, eng, delta);

                            if text.page() == 1
                                && !user.active_eq(user_id.index(), Some(*new))
                            {
                                user.replace(user_id.index(), Some(*new));
                                let renderer = &mut user_ui[user_id.index()];
                                let id = match user.active_mut(user_id.index()) {
                                    Some(user) => Some(match user.instance() {
                                        Some(i) => {
                                            renderer.status.update_gui(Some(i), None, true);
                                            i.pokemon.id
                                        }
                                        None => {
                                            renderer.status.update_gui_view(
                                                Some(user as _),
                                                None,
                                                true,
                                            );
                                            user.pokemon().id
                                        }
                                    }),
                                    None => None,
                                };
                                renderer.pokemon.new_pokemon(dex, id);
                            }
                        }
                        true => queue.current = None,
                    },
                    BattleClientGuiCurrent::UseItem(target) => {
                        if !text.finished() {
                            text.update(ctx, eng, delta)
                        } else if let Some(p_ui) = match target.team() == local.player.id() {
                            true => Some(&mut local.renderer),
                            false => remotes.get_mut(target.team()).map(|p| &mut p.renderer),
                        } {
                            let target = &mut p_ui[target.index()].status;
                            if target.health_moving() {
                                target.update_hp(delta);
                            } else {
                                queue.current = None;
                            }
                        } else {
                            queue.current = None;
                        }
                    }
                    BattleClientGuiCurrent::Faint => {
                        let ui = &mut user_ui[user_id.index()];
                        if ui.pokemon.faint.fainting() {
                            ui.pokemon.faint.update(delta);
                        } else if !text.finished() {
                            text.update(ctx, eng, delta);
                        } else {
                            drop(user);
                            match user_id.team() == local.player.id() && local.player.any_inactive()
                            {
                                true => match party.alive() {
                                    true => {
                                        party.input(
                                            ctx,
                                            eng,
                                            dex,
                                            pokedex,
                                            local.player.pokemon.as_mut_slice(),
                                        );
                                        party.update(delta);
                                        if let Some(selected) = party.take_selected() {
                                            if !local.player.pokemon[selected].fainted() {
                                                // user.queue_replace(index, selected);
                                                party.despawn();
                                                client.send(ClientMessage::ReplaceFaint(
                                                    user_id.index(),
                                                    selected,
                                                ));
                                                local
                                                    .player
                                                    .replace(user_id.index(), Some(selected));
                                                let pokemon = local.player.active(user_id.index());
                                                ui.status.update_gui(pokemon, None, true);
                                                ui.pokemon.new_pokemon(
                                                    dex,
                                                    pokemon.map(|p| p.pokemon.id),
                                                );
                                                queue.current = None;
                                            }
                                        }
                                    }
                                    false => {
                                        if let Err(err) = party.spawn(
                                            dex,
                                            pokedex,
                                            &local.player.pokemon,
                                            Some(false),
                                            false,
                                        ) {
                                            warn!("Error opening party gui: {}", err);
                                        }
                                    }
                                },
                                false => {
                                    if let Some(remote) = remotes.get_mut(user_id.team()) {
                                        remote.player.replace(user_id.index(), None);
                                        let ui = &mut remote.renderer[user_id.index()];
                                        ui.status.update_gui::<P, M, I>(None, None, true);
                                        ui.pokemon.new_pokemon(dex, None);
                                    }
                                    queue.current = None;
                                }
                            }
                        }
                    }
                    BattleClientGuiCurrent::Replace(index, replaced) => {
                        if text.waiting() || text.finished() && !*replaced {
                            if let Some(ui) = user_ui.get_mut(*index) {
                                let id = match user.active_mut(user_id.index()) {
                                    Some(v) => Some(match v.instance() {
                                        Some(i) => {
                                            ui.status.update_gui(Some(i), None, true);
                                            i.pokemon.id
                                        }
                                        None => {
                                            ui.status.update_gui_view(Some(v as _), None, true);
                                            v.pokemon().id
                                        }
                                    }),
                                    None => None,
                                };
                                ui.pokemon.new_pokemon(dex, id);
                                *replaced = true;
                            }
                        }
                        match text.finished() {
                            false => text.update(ctx, eng, delta),
                            true => queue.current = None,
                        }
                    }
                    BattleClientGuiCurrent::Catch => match text.finished() {
                        false => text.update(ctx, eng, delta),
                        true => queue.current = None,
                    },
                    BattleClientGuiCurrent::SetExp => {
                        match !text.finished()
                            || local.renderer[user_id.index()].status.exp_moving()
                        {
                            true => {
                                text.update(ctx, eng, delta);
                                match local.player.active(user_id.index()) {
                                    Some(pokemon) => local.renderer[user_id.index()]
                                        .status
                                        .update_exp(delta, pokemon),
                                    None => {
                                        warn!("Could not get pokemon gaining exp at {:?}", user_id);
                                        queue.current = None;
                                    }
                                }
                            }
                            false => queue.current = None,
                        }
                    }
                    BattleClientGuiCurrent::LevelUp => match level_up.alive() {
                        true => match local.player.pokemon.get_mut(user_id.index()) {
                            Some(pokemon) => {
                                if let Some((index, move_ref)) = level_up.update(
                                    ctx,
                                    eng,
                                    &mut text,
                                    delta,
                                    pokemon,
                                ) {
                                    client.send(ClientMessage::LearnMove(
                                        user_id.index(),
                                        move_ref.id,
                                        Some(index),
                                    ));
                                }
                            }
                            None => {
                                warn!("Could not get user's active pokemon at {:?}", user_id);
                                queue.current = None;
                            }
                        },
                        false => queue.current = None,
                    },
                },
                None => queue.current = None,
            }
        }
    }
}
