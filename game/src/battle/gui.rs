use std::rc::Rc;

use crate::{
    util::{Entity, Completable, Reset},
    pokedex::{
        moves::{
            instance::MoveInstance,
            target::{
                MoveTarget, 
                MoveTargetInstance,
                PlayerId,
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
        ActivePokemonIndex,
        ActionInstance,
        BattleMove,
        BattleClientMove,
        BattleClientAction,
        view::{
            BattlePartyView,
            BattlePartyKnown, 
            BattlePartyUnknown,
        },
    },
    client::{BattleEndpoint, BattleClient},
    message::{ClientMessage, ServerMessage, Active},
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

pub mod action;
pub mod guiref;

use action::*;

pub struct BattlePlayerGui {

    party: Rc<PartyGui>,
    bag: Rc<BagGui>,
	pub gui: BattleGui,

    state: BattlePlayerState,

    is_wild: bool,

    pub player: ActivePokemonParty<BattlePartyKnown>,
    pub opponent: ActivePokemonParty<BattlePartyUnknown>,

    messages: Vec<ClientMessage>,

}

#[derive(Debug)]
struct MoveQueue {
    actions: std::collections::VecDeque<BattleClientGuiActionInstance>,
    current: Option<BattleClientGuiCurrentInstance>,
}


#[derive(Debug)]
enum BattlePlayerState {
    WaitToSelect,
    Select(Active),
    Moving(MoveQueue),
    Winner(PlayerId),
}

impl Default for BattlePlayerState {
    fn default() -> Self {
        Self::WaitToSelect
    }
}

impl BattleEndpoint for BattlePlayerGui {
    fn give_client(&mut self, message: ServerMessage) {
        match message {
            ServerMessage::User(data, user) => {
                self.player.party = user;
                self.is_wild = matches!(data.type_, BattleType::Wild);
            },
            ServerMessage::Opponents(opponent) => {
                self.gui.panel.target(&opponent);
                self.opponent.party = opponent;
            },
            ServerMessage::StartSelecting => {
                self.state = BattlePlayerState::Select(0);
                self.gui.panel.despawn();
            },
            ServerMessage::TurnQueue(queue) => {
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
            ServerMessage::AskFinishedTurnQueue => if !matches!(self.state, BattlePlayerState::Moving(..)) {
                self.messages.push(ClientMessage::FinishedTurnQueue);
            },
            ServerMessage::PokemonRequest(index, instance) => self.opponent.party.add_instance(index, instance),
            ServerMessage::FaintReplace(pokemon, new) => match &mut self.state {
                BattlePlayerState::Moving(queue) => {
                    queue.actions.push_back(ActionInstance {
                        pokemon,
                        action: BattleClientGuiAction::Replace(new),
                    });
                },
                _ => {
                    let (player, player_ui) = match pokemon.team == self.player.party.id {
                        true => (&mut self.player.party as &mut dyn BattlePartyView, &mut self.player.renderer),
                        false => (&mut self.opponent.party as _, &mut self.opponent.renderer),
                    };
                    player.replace(pokemon.index, new);
                    player_ui[pokemon.index].update(self.opponent.party.active(pokemon.index))
                }
            },
            ServerMessage::AddUnknown(index, unknown) => self.opponent.party.add(index, unknown),
            ServerMessage::Winner(player) => self.state = BattlePlayerState::Winner(player),
            ServerMessage::AddMove(pokemon, index, move_ref) => if pokemon.team == self.player.party.id {
                if let Some(pokemon) = self.player.party.pokemon.get_mut(pokemon.index) {
                    debug!("to - do: set move to its index.");
                    if let Err(err) = pokemon.moves.try_push(MoveInstance::new(move_ref)) {
                        warn!("Cannot add moves to {} because it has maximum number of moves. error: {}", pokemon.name(), err)
                    }
                }
            }
        }
    }
}

impl BattleClient for BattlePlayerGui {

    fn give_server(&mut self) -> Option<ClientMessage> {
        self.messages.pop()
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
            messages: Vec::new(),
        }
    }

    pub fn winner(&self) -> Option<PlayerId> {
        if let BattlePlayerState::Winner(w) = self.state {
            Some(w)
        } else {
            None
        }
    }

    pub fn update(&mut self, ctx: &Context, delta: f32) {

        match &mut self.state {
            BattlePlayerState::WaitToSelect | BattlePlayerState::Winner(..) => (),
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
                                                match &item.value().usage {
                                                    ItemUseType::Pokeball => self.gui.panel.active = BattlePanels::Target(MoveTarget::Opponent, Some(item)),
                                                    ItemUseType::Script(..) => todo!("user targeting"),
                                                    ItemUseType::None => todo!("make item unusable"),
                                                }
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
                                                *active_index += 1;
                                                self.gui.panel.despawn();
                                            }
                                        } else if let Some(panels) = self.gui.panel.input(ctx, pokemon) {
                                            match panels {
                                                BattlePanels::Main => {
                                                    match self.gui.panel.battle.cursor {
                                                        0 => self.gui.panel.active = BattlePanels::Fight,
                                                        1 => self.bag.spawn(),
                                                        2 => crate::battle::ui::battle_party_known_gui(&self.party, &self.player.party, true),
                                                        3 => if self.is_wild {
                                                            self.messages.push(ClientMessage::Forfeit);
                                                        },
                                                        _ => unreachable!(),
                                                    }
                                                }
                                                BattlePanels::Fight => match pokemon.moves.get(self.gui.panel.fight.moves.cursor) {
                                                    Some(instance) => match instance.get() {
                                                        Some(move_ref) => {
                                                            let target = move_ref.value().target;
                                                            match target {
                                                                MoveTarget::Opponent => self.gui.panel.active = BattlePanels::Target(target, None),
                                                                _ => {
                                                                    self.messages.push(
                                                                        ClientMessage::Move(
                                                                            *active_index,
                                                                            BattleMove::Move(
                                                                                self.gui.panel.fight.moves.cursor,
                                                                                match target {
                                                                                    MoveTarget::User => MoveTargetInstance::user(),
                                                                                    MoveTarget::AllButUser => MoveTargetInstance::all_but_user(*active_index, self.player.party.active.len()),
                                                                                    MoveTarget::Opponents => MoveTargetInstance::opponents(self.opponent.party.active.len()),
                                                                                    MoveTarget::Opponent => unreachable!(),
                                                                                }
                                                                            )
                                                                        )
                                                                    );
                                                                    *active_index += 1;
                                                                    self.gui.panel.despawn();
                                                                }
                                                            }
                                                        }
                                                        None => warn!("Pokemon is out of Power Points for this move!")
                                                    }
                                                    None => warn!("Could not get move at cursor!"),
                                                }
                                                BattlePanels::Target(target, item) => {
                                                    self.messages.push(
                                                        ClientMessage::Move(
                                                            *active_index,
                                                            match item {
                                                                Some(item) => BattleMove::UseItem(
                                                                    item,
                                                                    match target {
                                                                        MoveTarget::Opponent => MoveTargetInstance::Opponent(self.gui.panel.targets.cursor),
                                                                        _ => unreachable!(),
                                                                    }
                                                                ),
                                                                None => BattleMove::Move(
                                                                    self.gui.panel.fight.moves.cursor, 
                                                                    match target {
                                                                        MoveTarget::Opponent => MoveTargetInstance::opponent(self.gui.panel.targets.cursor),
                                                                        _ => unreachable!(),
                                                                    }
                                                                ),
                                                            }
                                                        )
                                                    );
                                                    *active_index += 1;
                                                    self.gui.panel.despawn();
                                                }
                                            }
                                        }
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
            BattlePlayerState::Moving(queue) => {

                match &mut queue.current {
                    None => {
                        match queue.actions.pop_front() {
                            None => {
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

                                    if let Some(action) = match instance.action {
                                        BattleClientGuiAction::Action(action) => match action {
                                            BattleClientAction::Move(pokemon_move, targets) => {
                                                match user.active(instance.pokemon.index) {
                                                    Some(user_active) => {
                                                        ui::text::on_move(&mut self.gui.text, pokemon_move.value(), user_active);
            
                                                        for (target, moves) in &targets {
            
                                                            {
            
                                                                let user_pokemon = user.active_mut(instance.pokemon.index).unwrap();
            
                                                                let user_pokemon_ui = &mut user_ui[instance.pokemon.index];

                                                                for moves in moves {
                                                                    match moves {
                                                                        BattleClientMove::UserHP(damage) => user_pokemon.set_hp(*damage),
                                                                        BattleClientMove::Fail => ui::text::on_fail(&mut self.gui.text, vec![format!("{} cannot use move", user_pokemon.name()), format!("{} (Unimplemented)", pokemon_move.value().name)]),
                                                                        BattleClientMove::Miss => ui::text::on_miss(&mut self.gui.text, user_pokemon),
                                                                        BattleClientMove::GainExp(experience) => if let Some(pokemon) = user_pokemon.instance_mut() {
                                                                            queue.actions.push_front(ActionInstance { pokemon: instance.pokemon, action: BattleClientGuiAction::GainExp(pokemon.level, *experience) });
                                                                            let prev = pokemon.level;
                                                                            pokemon.add_exp(*experience);
                                                                            if prev != pokemon.level {
                                                                                debug!("To - do: pokemon level up");
                                                                            }
                                                                            // {
                                                                            //     queue.actions.push_front(ActionInstance { pokemon: instance.pokemon, action: BattleClientGuiAction::LevelUp(level, moves) });
                                                                            // }
                                                                            user_pokemon_ui.update_status_with_level(Some(pokemon), pokemon.level, false);
                                                                        },
                                                                        _ => (),
                                                                    }
                                                                }
                
                                                                user_pokemon_ui.update_status(Some(user_pokemon), false);
            
                                                            }
            
                                                            let (target, target_ui) = match target {
                                                                MoveTargetInstance::Opponent(index) => (other.active_mut(*index), &mut other_ui[*index]),
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
                                                                        BattleClientMove::Effective(effective) => ui::text::on_effective(&mut self.gui.text, &effective),
                                                                        BattleClientMove::StatStage(stat, stage) => ui::text::on_stat_stage(&mut self.gui.text, target, *stat, *stage),
                                                                        BattleClientMove::Faint(target_instance) => queue.actions.push_front(
                                                                            ActionInstance {
                                                                                pokemon: *target_instance,
                                                                                action: BattleClientGuiAction::Faint,
                                                                            }
                                                                        ), // exp gain stuff here,
                                                                        _ => (),
                                                                    }
                                                                }
                                                                target_ui.update_status(Some(target), false);
                                                            } else {
                                                                target_ui.update_status(None, false);
                                                            }
                                                        }
                                                    }
                                                    None => {
                                                        debug!("client was sent extra information!");
                                                        queue.current = None;
                                                    }
                                                }
                                                Some(BattleClientGuiCurrent::Move(targets))
                                            }
                                            BattleClientAction::UseItem(item, target) => {
                                                let (index, player) = match target {
                                                    MoveTargetInstance::Opponent(i) => (other.index(i), other),
                                                    MoveTargetInstance::Team(i) => (user.index(i), user),
                                                    MoveTargetInstance::User => (user.index(instance.pokemon.index), user),
                                                };
                                                if let Some(index) = index {
                                                    let item = item.value();
                                                    if let ItemUseType::Pokeball = item.usage {
                                                        // self.messages.push(ClientMessage::RequestPokemon(index));
                                                        queue.actions.push_front(ActionInstance {
                                                            pokemon: ActivePokemonIndex { team: *player.id(), index },
                                                            action: BattleClientGuiAction::Catch,
                                                        });
                                                    }
                                                    ui::text::on_item(&mut self.gui.text, player.pokemon(index), item);
                                                }
                                                Some(BattleClientGuiCurrent::UseItem(target))
                                            }
                                            BattleClientAction::Switch(index, unknown_pokemon) => {
                                                if let Some(unknown) = unknown_pokemon {
                                                    user.add(index, unknown);
                                                }
                                                let coming = user.pokemon(index).unwrap();
                                                ui::text::on_switch(&mut self.gui.text, user.active(instance.pokemon.index).unwrap(), coming);
                                                Some(BattleClientGuiCurrent::Switch(index))
                                            }
                                        }
                                        BattleClientGuiAction::Faint => {
                                            let is_user = &instance.pokemon.team == user.id();
                                            let target = user.active_mut(instance.pokemon.index).unwrap();
                                            target.set_hp(0.0);
                                            ui::text::on_faint(&mut self.gui.text, self.is_wild, is_user, target);
                                            user_ui[instance.pokemon.index].renderer.faint();
                                            Some(BattleClientGuiCurrent::Faint)
                                        },
                                        BattleClientGuiAction::Catch => {
                                            let pokemon = self.opponent.party.pokemon(instance.pokemon.index).map(|v| v.instance()).flatten();
                                            ui::text::on_catch(&mut self.gui.text, pokemon);
                                            if let Some(pokemon) = pokemon {
                                                if let Err(_) = pokemon_firered_clone_storage::data_mut().party.try_push(pokemon.clone()) {
                                                    debug!("Cannot add {} to party because party is full!", pokemon);
                                                }
                                                self.opponent.party.replace(instance.pokemon.index, None);
                                                self.opponent.renderer[instance.pokemon.index].update(None);
                                            }
                                            Some(BattleClientGuiCurrent::Catch)
                                        }
                                        BattleClientGuiAction::Replace(new) => {
                                            ui::text::on_replace(&mut self.gui.text, user.name(), new.map(|index| user.pokemon(index)).flatten());
                                            user.replace(instance.pokemon.index, new);
                                            Some(BattleClientGuiCurrent::Replace(false))
                                        }
                                        BattleClientGuiAction::GainExp(level, experience) => { // To - do: experience spreading
                                            if let Some(pokemon) = user.active(instance.pokemon.index).map(|v| v.instance()).flatten() {
                                                ui::text::on_gain_exp(&mut self.gui.text, pokemon, experience);
                                                user_ui[instance.pokemon.index].update_status_with_level(Some(pokemon), level, false);
                                                Some(BattleClientGuiCurrent::GainExp)
                                            } else {
                                                None
                                            }
                                        }
                                        BattleClientGuiAction::LevelUp(level, moves) => {
                                            // if let Some() = user.active(instance.pokemon.index).map(|v| v.instance()).flatten() {
                                            //     ui::text::on_level_up(text, pokemon, *level);
                                            //     if let Some(_) = moves {
                                            //         ui::text::on_fail(&mut self.gui.text, vec![format!("To - do: handle moves on level up")]);
                                            //     }
                                            // }
                                            None
                                        }
                                        // BattleClientAction::Catch(index) => {
                                        //     if let Some(target) = match index.team {
                                        //         Team::Player => &user.active[index.active],
                                        //         Team::Opponent => &other.active[index.active],
                                        //     }.pokemon.as_ref() {
                                        //         ui::text::on_catch(text, target);
                                        //     }
                                        // }
                                    } {
                                        queue.current = Some(ActionInstance {
                                            pokemon: instance.pokemon,
                                            action
                                        });
                                    }
                                }                                
                            },
                        }
                    },
                    Some(instance) => {

                        let (user, user_ui, other_ui) = if instance.pokemon.team == self.player.party.id {
                            (&mut self.player.party as &mut dyn BattlePartyView, &mut self.player.renderer, &mut self.opponent.renderer)
                        } else {
                            (&mut self.opponent.party as _, &mut self.opponent.renderer, &mut self.player.renderer)
                        };
                        

                        match &mut instance.action {
                            BattleClientGuiCurrent::Move(targets) => {
                                if !self.gui.text.finished() {
                                    self.gui.text.update(ctx, delta);
                                } else if self.gui.text.current > 0 || self.gui.text.can_continue {
                                    let index = instance.pokemon.index;
                                    targets.retain(|(t, _)| {
                                        let ui = match *t {
                                            MoveTargetInstance::Opponent(i) => &other_ui[i],
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
                                                MoveTargetInstance::Opponent(i) => &mut other_ui[i],
                                                MoveTargetInstance::Team(i) => &mut user_ui[i],
                                                MoveTargetInstance::User => &mut user_ui[instance.pokemon.index],
                                            };
                                            ui.renderer.flicker.update(delta);
                                            ui.status.update_hp(delta);
                                        }
                                    }                                    
                                }
                            },
                            BattleClientGuiCurrent::Switch(new) => {
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
                            BattleClientGuiCurrent::UseItem(target) => {
                                let target = match target {
                                    MoveTargetInstance::Opponent(i) => &mut other_ui[*i],
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
                            BattleClientGuiCurrent::Faint => {
                                let ui = &mut user_ui[instance.pokemon.index];
                                if ui.renderer.faint.fainting() {
                                	ui.renderer.faint.update(delta);
                                } else if !self.gui.text.finished() {
                                	self.gui.text.update(ctx, delta);
                                } else {
                                    match instance.pokemon.team == self.player.party.id && self.player.party.any_inactive() {
                                        true => match self.party.alive() {
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
                                        },
                                        false => {
                                            let user = match instance.pokemon.team == self.player.party.id {
                                                true => &mut self.player.party as &mut dyn BattlePartyView,
                                                false => &mut self.opponent.party as _,
                                            };
                                            user.replace(instance.pokemon.index, None);
                                            user_ui[instance.pokemon.index].update(None);
                                            queue.current = None;
                                        }
                                    }
                                }
                            }
                            BattleClientGuiCurrent::Replace(replaced) => {
                                if self.gui.text.can_continue() || self.gui.text.finished() && !*replaced {
                                    user_ui[instance.pokemon.index].update(user.active(instance.pokemon.index));
                                    *replaced = true;
                                }
                                if !self.gui.text.finished() {
                                    self.gui.text.update(ctx, delta);
                                } else {
                                    queue.current = None;
                                }
                            }
                            BattleClientGuiCurrent::Catch => {
                                if !self.gui.text.finished() {
                                    self.gui.text.update(ctx, delta);
                                } else {
                                    queue.current = None;
                                }
                            }
                            BattleClientGuiCurrent::GainExp => {
                                if !self.gui.text.finished() || self.player.renderer[instance.pokemon.index].status.exp_moving() {
                                    self.gui.text.update(ctx, delta);
                                    if let Some(pokemon) = self.player.party.active(instance.pokemon.index).map(|v| v.instance()).flatten() {
                                        self.player.renderer[instance.pokemon.index].status.update_exp(delta, pokemon);
                                    } else {
                                        warn!("Could not get pokemon gaining exp at {}", instance.pokemon);
                                        queue.current = None;
                                    }
                                } else {
                                    queue.current = None;
                                }
                            }
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
            BattlePlayerState::WaitToSelect | BattlePlayerState::Moving(..) => {
                for active in self.player.renderer.iter() {
                    active.renderer.draw(ctx, ZERO, Color::WHITE);
                    active.status.draw(ctx, 0.0, 0.0);
                }
                self.gui.draw_panel(ctx);
                self.gui.text.draw(ctx);
                if self.party.alive() {
                    self.party.draw(ctx)
                }
            },
            BattlePlayerState::Winner(..) => {
                for active in self.player.renderer.iter() {
                    active.renderer.draw(ctx, ZERO, Color::WHITE);
                    active.status.draw(ctx, 0.0, 0.0);
                }
                self.gui.draw_panel(ctx);
                self.gui.text.draw(ctx);
            }
        }
        
    }
}