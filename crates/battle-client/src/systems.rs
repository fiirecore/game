use battle::{
    message::{ClientMessage, ServerMessage},
    pokedex::{item::Item, moves::Move, pokemon::Pokemon},
    pokemon::Indexed, party::PlayerParty,
};
use bevy::prelude::*;
use iyes_loopless::state::CurrentState;
use pokengine::{
    dex::BevyDex,
    engine::bevy_egui::{egui, EguiContext},
};

use crate::{
    components::*,
    resources::*, BattleClientState,
};

pub fn in_battle(state: Res<CurrentState<Option<BattleClientState>>>) -> bool {
    matches!(&state.0, Some(..))
}

pub fn process_events<
    ID: core::fmt::Debug + Clone + PartialEq + Send + Sync + 'static,
>(
    mut commands: Commands,
    channel: Res<PlayerChannel<ID, BattleTrainer>>,
    mut remote: ResMut<CurrentRemote>,
    pokedex: Res<BevyDex<Pokemon>>,
    movedex: Res<BevyDex<Move>>,
    itemdex: Res<BevyDex<Item>>,
    mut local: Option<ResMut<GuiLocalPlayer<ID>>>,
    mut remotes: Query<(Entity, &BattleId<ID>, &mut Active<usize>, &mut RemoteParty)>,
) {
    loop {
       match channel.0.receive(){
        Ok(Some(message)) => {
            println!("{message:?}");
            match message {
            ServerMessage::Select(pos, _) => {
                commands.spawn(Select(pos));
            }
            ServerMessage::Results(mut results) => commands.insert_resource(Results({
                results.reverse();
                results
            }
            )),
            ServerMessage::Replace(Indexed(active_pos, with)) => if let Some(local) = local.as_mut() {
                match active_pos.team() == &local.player.id {
                    true => local.player.replace(active_pos.index(), Some(with)),
                    false => if let Some((_, _, mut active, ..)) = remotes.iter_mut().find(|(e, id, ..)| &***id == active_pos.team()) {
                        active[active_pos.index()] = Some(with);
                    }
                }
            },
            ServerMessage::Reveal(Indexed(party_pos, view)) => {
                for (_, id, _, mut party, ..) in remotes.iter_mut() {
                    if &**id == party_pos.team() {
                        if let Some(pokemon) = party.0.get_mut(party_pos.index()) {
                            match view {
                                battle::pokemon::PokemonView::Partial(revealed) => {
                                    *pokemon = Some(revealed.init(&pokedex).unwrap());
                                }
                                battle::pokemon::PokemonView::Full(_) => todo!(),
                            }
                        }
                        break;
                    }
                }
            }
            ServerMessage::PlayerData(data, player, bag) => {
                commands.insert_resource(GuiLocalPlayer {
                    player: PlayerParty {
                        id: player.id,
                        name: player.name,
                        active: player.active,
                        pokemon: player.pokemon.into_iter().map(|p| p.try_init(&pokedex, &movedex, &itemdex).unwrap()).collect(),
                        trainer: player.trainer,
                    },
                    bag: bag.init(&itemdex).unwrap(),
                    data: data,
                });
            },
            ServerMessage::AddOpponent(party) => {
                let mut e = commands.spawn((
                    BattleId(party.id),
                    Active(party.active),
                    RemoteParty(
                        party
                            .pokemon
                            .iter()
                            .cloned()
                            .map(|u| u.map(|u| u.init(&pokedex).unwrap()))
                            .collect(),
                    ),
                    BattleComponent,
                ));

                if let Some(n) = party.name {
                    e.insert(PlayerName(n));
                }

                if let Some(t) = party.trainer {
                    e.insert(Trainer(t));
                }

                if remote.data.is_none() {
                    remote.data = Some(e.id());
                }
            }
            ServerMessage::Remove(_, _, _) => {}
            ServerMessage::End(_) => todo!(),}
        },
        Ok(None) => break,
        Err(e) => todo!(),
    }
}
}

pub fn select<ID: Send + Sync + 'static>(
    mut commands: Commands,
    channel: Res<PlayerChannel<ID, BattleTrainer>>,
    mut ctx: ResMut<EguiContext>,
    local: Res<GuiLocalPlayer<ID>>,
    query: Query<(Entity, &Select)>,
) {
    println!("{}", query.iter().count());
    if let Some((e, Select(pos))) = query.iter().next() {
        if let Some(pokemon) = local.player.active(*pos) {
            egui::Window::new("Select")
                .collapsible(false)
                .show(ctx.ctx_mut(), |ui| {
                    for (i, m) in pokemon.moves.iter().enumerate() {
                        if ui.button(&m.m.name).clicked() {
                            channel.0.send(ClientMessage::Select(
                                *pos,
                                battle::select::BattleSelection::Move(m.id().clone(), None),
                            )).unwrap();
                            commands.entity(e).despawn_recursive();
                        }
                        if i != 0 && i % 2 == 0 {
                            ui.end_row();
                        }
                    }
                });
        }
    }
}

// pub fn start_battle<ID: Send + Sync + 'static>(
//     In(data): In<ClientPlayerData<ID, BattleTrainer>>,
//     mut commands: Commands,
//     mut random: ResMut<BattleRandom>,
//     pokedex: Res<BevyDex<Pokemon>>,
//     movedex: Res<BevyDex<Move>>,
//     itemdex: Res<BevyDex<Item>>,
// ) {
//     // commands.insert_resource();
//     commands.insert_resource(NextState(Some(BattleClientState::Opening)));

//     let mut e0 = None;

//     data.remotes.into_iter().for_each(|player| {});

//     if let Some(e) = e0 {
//         commands.insert_resource(CurrentRemote(e));
//     }

//     commands.insert_resource(GuiLocalPlayer {
//         player: battle::party::PlayerParty {
//             name: data.local.name,
//             id: data.local.id,
//             active: data.local.active,
//             pokemon: data
//                 .local
//                 .pokemon
//                 .into_iter()
//                 .flat_map(|p| p.init(&mut random.0, &pokedex, &movedex, &itemdex))
//                 .collect(),
//             trainer: data.local.trainer,
//         },
//         bag: data.bag.init(&itemdex).unwrap_or_default(),
//         data: data.data,
//     });
// }
