use std::rc::Rc;

use crate::{
    util::{Entity, Completable, Reset},
    pokedex::{
        pokemon::{Level, Experience},
        moves::{
            MoveRef,
            target::{
                MoveTarget, 
                MoveTargetInstance,
            }
        },
        item::ItemUseType, 
    },
    gui::{bag::BagGui, party::PartyGui}, 
    tetra::Context,
    log::{warn, debug},
};

use crate::battle::{
    BattleType, 
    pokemon::{
        BattleMove,
        BattleClientMove,
        BattleClientAction,
        ActionInstance,
        BattleClientActionInstance,
        view::{
            BattlePartyView,
            BattlePartyKnown, 
            BattlePartyUnknown,
        },
    },
    client::{BattleEndpoint, BattleClient},
    message::{ClientMessage, ServerMessage},
    ui::{
        self,
        BattleGui,
        panels::BattlePanels,
        view::{
            ActivePokemonParty,
            ActivePokemonRenderer,
        }
    },
};

pub struct BattlePlayerGui {

    party: Rc<PartyGui>,
    bag: Rc<BagGui>,
	pub gui: BattleGui,

    state: BattlePlayerState,

    is_wild: bool,

    pub player: ActivePokemonParty<BattlePartyKnown>,

    pub opponent: ActivePokemonParty<BattlePartyUnknown>,

    // faint: deps::hash::HashMap<usize, usize>,

    messages: Vec<ClientMessage>,

}

#[derive(Debug, Clone)]
pub enum BattleClientGuiAction {
    Action(BattleClientAction),
    Faint,
    Catch(usize),
    GainExp(Level, Experience),
    LevelUp(Level, Option<Vec<MoveRef>>),
    Replace(Option<usize>),
}

impl BattleClientGuiAction {
    pub fn requires_user(&self) -> bool {
        matches!(self, Self::Faint)
    }
}

#[derive(Clone)]
pub struct BattlePlayerGuiRef(pub Rc<std::cell::UnsafeCell<BattlePlayerGui>>);

impl BattlePlayerGuiRef {
    pub fn new(ctx: &mut Context, party: Rc<PartyGui>, bag: Rc<BagGui>) -> Self {
        Self(Rc::new(std::cell::UnsafeCell::new(BattlePlayerGui::new(ctx, party, bag))))
    }
    pub fn get(&self) -> &mut BattlePlayerGui {
        unsafe { self.0.get().as_mut().unwrap() }
    }
}

impl BattleEndpoint for BattlePlayerGuiRef {
    fn give_client(&mut self, message: ServerMessage) {
        self.get().give_client(message)
    }
}

impl BattleClient for BattlePlayerGuiRef {
    fn give_server(&mut self) -> Option<ClientMessage> {
        self.get().give_server()
    }
}


#[derive(Debug)]
struct MoveQueue {
    actions: std::collections::VecDeque<BattleClientGuiActionInstance>,
    current: Option<BattleClientGuiActionInstance>,
}

pub type BattleClientGuiActionInstance = firecore_battle::pokemon::ActionInstance<BattleClientGuiAction>;

#[derive(Debug)]
enum BattlePlayerState {
    WaitToSelect,
    Select(usize), // usize = active pokemon num
    // Faint(usize),
    WaitToMove,
    Moving(MoveQueue),
}

// enum MoveState {
//     Start,
// 	SetupPokemon,
// 	Pokemon(Vec<BattleMoveInstance>), // queue of pokemon
// 	SetupPost,
// 	Post,
// 	End,
// }

impl Default for BattlePlayerState {
    fn default() -> Self {
        Self::WaitToSelect
    }
}

impl BattlePlayerGui {

    pub fn new(ctx: &mut Context, party: Rc<PartyGui>, bag: Rc<BagGui>) -> Self {
        Self {
            party,
            bag,
			gui: BattleGui::new(ctx),
            state: Default::default(),
            is_wild: false,
            player: Default::default(),
            opponent: Default::default(),
            // faint: Default::default(),
            messages: Vec::new(),
        }
    }

    pub fn update(&mut self, ctx: &Context, delta: f32) {

        match &mut self.state {
            BattlePlayerState::WaitToSelect | BattlePlayerState::WaitToMove => (),//debug!("{:?}", self.state),
            BattlePlayerState::Select(active_index) => {
                match self.player.party.active.get(*active_index) {
                    Some(index) => match index {
                        Some(index) => {
                            let pokemon = &self.player.party.pokemon[*index];
                            match self.gui.panel.alive {
                                // true => match self.player.party.active.len() <= *active_index {
                                    true => {
            
                                        // Checks if a move is queued from an action done in the GUI
            
                                        if self.bag.alive() {
                                            self.bag.input(ctx);
                                            if let Some(item) = self.bag.take_selected_despawn() {
                                                let target = match &item.value().usage {
                                                    ItemUseType::Pokeball => MoveTargetInstance::Opponent(self.opponent.party.id, crate::battle::BATTLE_RANDOM.gen_range(0, self.gui.panel.targets.names.len())),
                                                    ItemUseType::Script(..) => todo!("user targeting"),
                                                    ItemUseType::None => todo!("make item unusable"),
                                                    // MoveTarget::Opponents => todo!("make none"),
                                                };
                                                self.messages.push(
                                                    ClientMessage::Move(
                                                        *active_index,
                                                        BattleMove::UseItem(item, target)
                                                    )
                                                );
                                            }
                                        } else if self.party.alive() {
                                            self.party.input(ctx);
                                            self.party.update(delta);
                                            if let Some(selected) = self.party.take_selected() {
                                                self.party.despawn();
                                                self.messages.push(
                                                    ClientMessage::Move(
                                                        *active_index,
                                                        BattleMove::Switch(selected)
                                                    )
                                                );
                                            }
                                        } else if let Some(panels) = self.gui.panel.input(ctx, pokemon) {
                                            match panels {
                                                BattlePanels::Main => {
                                                    match self.gui.panel.battle.cursor {
                                                        0 => self.gui.panel.active = BattlePanels::Fight,
                                                        1 => self.bag.spawn(),
                                                        2 => crate::battle::ui::battle_party_known_gui(&self.party, &self.player.party, true),
                                                        3 => if self.is_wild {
                                                            self.messages.push(ClientMessage::Forfeit); // To - do: "Got away safely!" - run text and conditions
                                                        },
                                                        _ => unreachable!(),
                                                    }
                                                }
                                                BattlePanels::Fight => match pokemon.moves.get(self.gui.panel.fight.moves.cursor) {
                                                    Some(instance) => match instance.get() {
                                                        Some(move_ref) => {
                                                            let target = move_ref.value().target;
                                                            match target {
                                                                MoveTarget::Opponent => self.gui.panel.active = BattlePanels::Target(target),
                                                                _ => self.messages.push(
                                                                    ClientMessage::Move(
                                                                        *active_index,
                                                                        BattleMove::Move(
                                                                            self.gui.panel.fight.moves.cursor,
                                                                            match target {
                                                                                MoveTarget::User => MoveTargetInstance::user(),
                                                                                MoveTarget::AllButUser => MoveTargetInstance::all_but_user(*active_index, self.opponent.party.id, self.player.party.active.len()),
                                                                                MoveTarget::Opponents => MoveTargetInstance::opponents(self.opponent.party.id, self.opponent.party.active.len()),
                                                                                MoveTarget::Opponent => unreachable!(),
                                                                            }
                                                                        )
                                                                    )
                                                                ),
                                                            }
                                                        }
                                                        None => warn!("Pokemon is out of Power Points for this move!")
                                                    }
                                                    None => warn!("Could not get move at cursor!"),
                                                }
                                                BattlePanels::Target(target) => self.messages.push(
                                                    ClientMessage::Move(
                                                        *active_index,
                                                        BattleMove::Move(self.gui.panel.fight.moves.cursor, match target {
                                                        MoveTarget::Opponent => MoveTargetInstance::opponent(self.opponent.party.id, self.gui.panel.targets.cursor),
                                                        _ => unreachable!(),
                                                }))),
                                            }
                                        }
                                    // }
                                    // false => {
                                    //     *active_index += 1;
                                    //     self.gui.panel.despawn();
                                    // }
                                }
                                false => {
                                    
                                    self.gui.panel.user(pokemon);
                                }
                            }
                        },
                        None => *active_index += 1,
                    },
                    None => {
                        self.gui.panel.despawn();
                    },
                }
            },
            // BattlePlayerState::Faint(active) => ,
            BattlePlayerState::Moving(queue) => {

                match &mut queue.current {
                    None => {
                        match queue.actions.pop_front() {
                            None => {
                                debug!("finished turn queue");
                                self.messages.push(ClientMessage::FinishedTurnQueue);
                                self.state = BattlePlayerState::WaitToSelect;
                            }
                            Some(instance) => {

                                // to - do: better client checking

                                let (user, user_ui, other, other_ui) = if instance.pokemon.team == self.player.party.id {
                                    (&mut self.player.party as &mut dyn BattlePartyView, &mut self.player.renderer, &mut self.opponent.party as &mut dyn BattlePartyView, &mut self.opponent.renderer)
                                } else {
                                    (&mut self.opponent.party as _, &mut self.opponent.renderer, &mut self.player.party as _, &mut self.player.renderer)
                                };

                                self.gui.text.clear();
                                self.gui.text.reset();

                                if user.active(instance.pokemon.index).is_some() || !instance.action.requires_user() {

                                    match &instance.action {
                                        BattleClientGuiAction::Action(action) => match action {
                                            BattleClientAction::Move(pokemon_move, targets) => {

                                                ui::text::on_move(&mut self.gui.text, pokemon_move.value(), user.active(instance.pokemon.index).unwrap());
        
                                                for (target, moves) in targets {
    
                                                    {
    
                                                        let user = user.active_mut(instance.pokemon.index).unwrap();
    
                                                        let user_pokemon_ui = &mut user_ui[instance.pokemon.index];
    
                                                        for moves in moves {
                                                            match moves {
                                                                BattleClientMove::UserHP(damage) => {
                                                                    user.set_hp(*damage);
                                                                    
                                                                }
                                                                BattleClientMove::Fail => {
                                                                    ui::text::on_fail(&mut self.gui.text, vec![format!("{} cannot use move", user.name()), format!("{} (Unimplemented)", pokemon_move.value().name)])
                                                                },
                                                                BattleClientMove::Miss => ui::text::on_miss(&mut self.gui.text, user),
                                                                _ => (),
                                                            }
                                                        }
        
                                                        user_pokemon_ui.update_status(Some(user), false);
    
                                                    }
    
                                                    let (target, target_ui) = match target {
                                                        MoveTargetInstance::Opponent(team, index) => (other.active_mut(*index), &mut other_ui[*index]),
                                                        MoveTargetInstance::Team(index) => (user.active_mut(*index), &mut user_ui[*index]),
                                                        MoveTargetInstance::User => (user.active_mut(instance.pokemon.index), &mut user_ui[instance.pokemon.index]),
                                                    };
    
                                                    if let Some(target) = target {
                                                        for moves in moves {
                                                            match moves {
                                                                BattleClientMove::TargetHP(damage) => {
                                                                    target.set_hp(*damage);
                                                                    if damage >= &0.0 {
                                                                        target_ui.renderer.flicker()
                                                                    }
                                                                },
                                                                BattleClientMove::Effective(effective) => {
                                                                    ui::text::on_effective(&mut self.gui.text, effective);
                                                                },
                                                                BattleClientMove::StatStage(stat, stage) => {
                                                                    ui::text::on_stat_stage(&mut self.gui.text, target, *stat, *stage);
                                                                }
                                                                BattleClientMove::Faint(target_instance) => {
                                                                    queue.actions.push_front(
                                                                        ActionInstance {
                                                                            pokemon: *target_instance,
                                                                            action: BattleClientGuiAction::Faint,
                                                                        }
                                                                    );
                                                                    // exp gain stuff here
        
                                                                },
                                                                _ => (),
                                                            }
                                                        }
    
                                                        target_ui.update_status(Some(target), false);
                                                    } else {
                                                        target_ui.update_status(None, false);
                                                    }
    
                                                }
        
                                            }
                                            BattleClientAction::UseItem(item, target) => {
                                                let target = match target {
                                                    MoveTargetInstance::Opponent(team, i) => other.active(*i),
                                                    MoveTargetInstance::Team(i) => user.active(*i),
                                                    MoveTargetInstance::User => user.active(instance.pokemon.index),
                                                }.unwrap();
                                                ui::text::on_item(&mut self.gui.text, target, item.value())
                                            }
                                            BattleClientAction::Switch(index, unknown_pokemon) => {
                                                if let Some(unknown) = unknown_pokemon {
                                                    user.add(*index, *unknown);
                                                }
                                                let coming = user.pokemon(*index).unwrap();
                                                ui::text::on_switch(&mut self.gui.text, user.active(instance.pokemon.index).unwrap(), coming);
                                            }
                                        }
                                        BattleClientGuiAction::Faint => {
                                            let is_user = &instance.pokemon.team == user.id();
                                            let target = user.active_mut(instance.pokemon.index).unwrap();
                                            target.set_hp(0.0);
                                            ui::text::on_faint(&mut self.gui.text, self.is_wild, is_user, target);
                                            user_ui[instance.pokemon.index].renderer.faint();
                                            // let target = match instance.pokemon.team {
                                            //     Team::Player => &mut self.user as &mut dyn BattlePartyView,
                                            //     Team::Opponent => &mut self.opponent as _,
                                            // };
                                            // ui::text::on_faint(&mut self.gui.text, self.is_wild, instance.pokemon.team, target);
                                            // user_pokemon_ui.renderer.faint();
    
                                            // if instance.pokemon.team == Team::Player {
                                            //     self.start_faint(instance.pokemon.index);
                                            // }
    
                                            // if let Some(assailant) = assailant {
                                            //     if assailant.team == Team::Player {
                                            //         let experience = {
                                            //             let instance = user.active(instance.pokemon.index).unwrap();
                                            //             instance.pokemon().value().exp_from(instance.level()) as f32 * 
                                            //             match self.is_wild {
                                            //                 true => 1.0,
                                            //                 false => 1.5,
                                            //             } *
                                            //             7.0
                                            //         } as crate::pokedex::pokemon::Experience;
                                            //         let (assailant_party, index) = (&mut match assailant.team {
                                            //             Team::Player => &mut self.player,
                                            //             Team::Opponent => &mut self.opponent,
                                            //         }, assailant.active);
                                            //         if let Some(assailant_pokemon) = assailant_party.active[index].pokemon.as_mut() {
                                            //             let level = assailant_pokemon.level;
                                            //             if let Some((level, moves)) = assailant_pokemon.add_exp(experience) {
                                            //                 queue.actions.push_front(BattleMoveInstance { pokemon: *assailant, action: BattleAction::LevelUp(level, moves) });
                                            //             }
                                            //             queue.actions.push_front(BattleMoveInstance { pokemon: *assailant, action: BattleAction::GainExp(level, experience) });
                                            //         }
                                            //     }
                                            // }
    
                                        },
                                        BattleClientGuiAction::Catch(i) => {
                                            let o = self.opponent.party.active(*i).unwrap();
                                            pokemon_firered_clone_storage::data_mut().party.try_push(pokedex::pokemon::instance::PokemonInstance::generate(*o.pokemon().id(), o.level(), o.level(), None));
                                            self.opponent.party.replace(*i, None);
                                            self.opponent.renderer[*i].update(None);
                                        }
                                        BattleClientGuiAction::Replace(new) => {
                                            debug!("{:?}", new);
                                            user.replace(instance.pokemon.index, *new);
                                            user_ui[instance.pokemon.index].update(user.active(instance.pokemon.index));
                                        }
                                        _ => todo!(),
                                        // BattleClientAction::GainExp(level, experience) => { // To - do: experience spreading
                                        //     ui::text::on_gain_exp(&mut self.gui.text, pokemon, *experience);
                                        //     user_pokemon_ui[instance.pokemon.index].update_status(user.active(instance.pokemon.index), *level, false);
                                        // }
                                        // BattleClientAction::LevelUp(level, moves) => {
                                        //     ui::text::on_level_up(text, pokemon, *level);
                                        //     if let Some(_) = moves {
                                        //         ui::text::on_fail(&mut self.gui.text, vec![format!("To - do: handle moves on level up")]);
                                        //     }
                                        // }
                                        // BattleClientAction::Catch(index) => {
                                        //     if let Some(target) = match index.team {
                                        //         Team::Player => &user.active[index.active],
                                        //         Team::Opponent => &other.active[index.active],
                                        //     }.pokemon.as_ref() {
                                        //         ui::text::on_catch(text, target);
                                        //     }
                                        // }
                                    }

                                    // end of let Some(pokemon)

                                    queue.current = Some(instance);

                                }                                
                            },
                        }
                    },
                    Some(instance) => {

                        let (user, user_ui, other, other_ui) = if instance.pokemon.team == self.player.party.id {
                            (&mut self.player.party as &mut dyn BattlePartyView, &mut self.player.renderer, &mut self.opponent.party as &mut dyn BattlePartyView, &mut self.opponent.renderer)
                        } else {
                            (&mut self.opponent.party as _, &mut self.opponent.renderer, &mut self.player.party as _, &mut self.player.renderer)
                        };
                        

                        match &mut instance.action {
                            BattleClientGuiAction::Action(action) => match action {
                                BattleClientAction::Move(_, targets) => {
                                    if !self.gui.text.finished() {
                                        self.gui.text.update(ctx, delta);
                                    } else if self.gui.text.current > 0 || self.gui.text.can_continue {
                                        let index = instance.pokemon.index;
                                        targets.retain(|(t, _)| {
                                            let ui = match *t {
                                                MoveTargetInstance::Opponent(team ,i) => &other_ui[i],
                                                MoveTargetInstance::Team(i) => &user_ui[i],
                                                MoveTargetInstance::User => &user_ui[index],
                                            };
                                            ui.renderer.flicker.flickering() || ui.status.health_moving()
                                        });
                                        if targets.is_empty() {
                                            queue.current = None;
                                        } else {
                                            for target in targets {
                                                let ui = match target.0 {
                                                    MoveTargetInstance::Opponent(team ,i) => &mut other_ui[i],
                                                    MoveTargetInstance::Team(i) => &mut user_ui[i],
                                                    MoveTargetInstance::User => &mut user_ui[instance.pokemon.index],
                                                };
                                                ui.renderer.flicker.update(delta);
                                                ui.status.update_hp(delta);
                                            }
                                        }                                    
                                    }
                                },
                                BattleClientAction::Switch(new, _) => {
                                    if self.gui.text.finished() {
                                        queue.current = None;
                                    } else {
    
                                        self.gui.text.update(ctx, delta);
    
                                        if self.gui.text.current() == 1 && !user.active_eq(instance.pokemon.index, Some(*new)) {
                                            user.replace(instance.pokemon.index, Some(*new));
                                            user_ui[instance.pokemon.index].update(user.active(instance.pokemon.index));
                                        }
    
                                    }
                                },
                                BattleClientAction::UseItem(_, target) => {
                                    let target = match target {
                                        MoveTargetInstance::Opponent(team, i) => &mut other_ui[*i],
                                        MoveTargetInstance::Team(i) => &mut user_ui[*i],
                                        MoveTargetInstance::User => &mut user_ui[instance.pokemon.index],
                                    };
                                    if !self.gui.text.finished() {
                                        self.gui.text.update(ctx, delta)
                                    } else if target.status.health_moving() {
                                        target.status.update_hp(delta);
                                    } else {
                                        queue.current = None;
                                    }
                                },
                            },
                            BattleClientGuiAction::Faint => {
                                let ui = &mut user_ui[instance.pokemon.index];
                                if ui.renderer.faint.fainting() {
                                	ui.renderer.faint.update(delta);
                                } else if !self.gui.text.finished() {
                                	self.gui.text.update(ctx, delta);
                                } else {
                                    match instance.pokemon.team == self.player.party.id {
                                        true => if self.player.party.any_inactive() {
                                            match self.party.alive() {
                                                true => {
                                                    self.party.input(ctx);
                                                    self.party.update(delta);
                                                    if let Some(selected) = self.party.take_selected() {
                                                        if !self.player.party.pokemon[selected].fainted() {
                                                            // user.queue_replace(index, selected);
                                                            self.party.despawn();
                                                            self.messages.push(ClientMessage::FaintReplace(instance.pokemon.index, selected));
                                                            self.player.party.replace(instance.pokemon.index, Some(selected));
                                                            ui.update(self.player.party.active(instance.pokemon.index));
                                                            queue.current = None;
                                                        }
                                                    }
                                                },
                                                false => crate::battle::ui::battle_party_known_gui(&self.party, &self.player.party, false)
                                            }
                                        } else {
                                            self.player.party.replace(instance.pokemon.index, None);
                                            user_ui[instance.pokemon.index].update(None);
                                            queue.current = None;
                                        },
                                        false => {
                                            self.opponent.party.replace(instance.pokemon.index, None);
                                            queue.current = None;
                                        }
                                    }
                                }
                            }
                            BattleClientGuiAction::Catch(..) | BattleClientGuiAction::Replace(..) => queue.current = None,
                            _ => todo!(),
                            // BattleClientAction::GainExp(_, _) => {
                            //     let user = self.user.active_mut(instance.pokemon.index);
                            //     let renderer = user_rend[instance.pokemon.active];
                            //     if !self.gui.text.finished() || cli.status.exp_moving() {
                            //         self.gui.text.update(ctx, delta);
                            //         if self.gui.text.current > 0 || text.can_continue {
                            //             renderer.status.update_exp(delta, user.pokemon.as_ref().unwrap());
                            //         }
                            //     } else {
                            //         queue.current = None;
                            //     }
                            // },
                            // BattleClientAction::LevelUp(_, _) => todo!(),
                        }
                    },
                }
            }
        }
    }

    pub fn on_begin(&mut self, ctx: &mut Context) {
        self.player.renderer = ActivePokemonRenderer::init_known(ctx, &self.player.party);
        self.opponent.renderer = ActivePokemonRenderer::init_unknown(ctx, &self.opponent.party);
    }

    pub fn draw(&self, ctx: &mut Context) {
        use crate::{graphics::ZERO, tetra::{math::Vec2, graphics::Color}};
        self.gui.background.draw(ctx, 0.0);
        for active in self.opponent.renderer.iter() {
            active.renderer.draw(ctx, ZERO, Color::WHITE);
            active.status.draw(ctx, 0.0, 0.0);
        }
        match &self.state {
            BattlePlayerState::Select(index) => {
                if self.party.alive() {
                    self.party.draw(ctx);
                } else if self.bag.alive() {
                    self.bag.draw(ctx);
                } else {
                    for (current, active) in self.player.renderer.iter().enumerate() {
                        if &current == index {
                            active.renderer.draw(ctx, Vec2::new(0.0, self.gui.bounce.offset), Color::WHITE);
                            active.status.draw(ctx, 0.0, -self.gui.bounce.offset);
                        } else {
                            active.renderer.draw(ctx, ZERO, Color::WHITE);
                            active.status.draw(ctx, 0.0, 0.0);
                        }
                    }
                    self.gui.draw_panel(ctx);
                    self.gui.panel.draw(ctx);
                }
            },
            // BattlePlayerState::Faint(..) => if self.party.alive() {
            //     self.party.draw(ctx)
            // },
            BattlePlayerState::WaitToSelect | BattlePlayerState::WaitToMove | BattlePlayerState::Moving(..) => {
                for active in self.opponent.renderer.iter().chain(self.player.renderer.iter()) {
                    active.renderer.draw(ctx, ZERO, Color::WHITE);
                    active.status.draw(ctx, 0.0, 0.0);
                }
                self.gui.draw_panel(ctx);
                self.gui.text.draw(ctx);
                if self.party.alive() {
                    self.party.draw(ctx)
                }
            },
        }
        
    }
}

impl BattleEndpoint for BattlePlayerGui {
    fn give_client(&mut self, message: ServerMessage) {
        match message {
            ServerMessage::User(battle_type, user) => {
                self.player.party = user;
                self.is_wild = battle_type == BattleType::Wild;
            },
            ServerMessage::Opponents(opponent) => {
                self.gui.panel.target(&opponent);
                self.opponent.party = opponent;
            },
            ServerMessage::StartSelecting => {
                debug!("starting selecting");
                self.state = BattlePlayerState::Select(0);
                self.gui.panel.despawn();
            },
            ServerMessage::TurnQueue(queue) => {
                debug!("starting turn queue");
                self.state = BattlePlayerState::Moving(MoveQueue {
                    actions: queue.into_owned().into_iter().map(|a| ActionInstance {
                        pokemon: a.pokemon,
                        action: BattleClientGuiAction::Action(a.action),
                    }).collect(),
                    current: None,
                });
                self.gui.text.clear();
                self.gui.text.spawn();
            },
            ServerMessage::AskFinishedTurnQueue => (),
            // ServerMessage::RequestFaintReplace(active) => {
            //     if let Some(replacement) = self.faint.remove(&active) {
            //         self.messages.push(ClientMessage::SelectFaintReplace(active, replacement));
            //     }
            // },
            ServerMessage::FaintReplace(team, active, new) => match &mut self.state {
                BattlePlayerState::Moving(queue) => {
                    debug!("faint replace: {}, {}, {:?}, len {}", team, active, new, queue.actions.len());
                    queue.actions.push_back(ActionInstance {
                        pokemon: firecore_battle::pokemon::ActivePokemonIndex { team, index: active },
                        action: BattleClientGuiAction::Replace(new),
                    });
                },
                _ => {
                    debug!("faint replace: {}, {}, {:?}", team, active, new);
                    self.opponent.party.replace(active, new);
                    self.opponent.renderer[active].update(self.opponent.party.active(active))
                }
            },
            ServerMessage::AddUnknown(_, index, unknown) => self.opponent.party.add(index, unknown),
            // ServerMessage::Winner(winner) => (),
        }
    }
}

impl BattleClient for BattlePlayerGui {

    fn give_server(&mut self) -> Option<ClientMessage> {
        self.messages.pop()
    }
}