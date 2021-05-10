extern crate firecore_game as game;

use std::collections::VecDeque;

use game::{
	deps::Random,
	util::{
		Entity,
		Completable,
		Reset,
		battle::BattleType,
	},
	gui::{
		party::PartyGui,
		bag::BagGui,
	},
	battle::{
		BattleData,
		TrainerData,
		BattleTeam,
	},
	pokedex::{
		moves::{
			MoveCategory,
			MoveRef,
			target::MoveTargetInstance,
			script::{
				MoveAction,
				MoveActionType,
				DamageKind
			},
		},
		pokemon::{
			Health,
			saved::SavedPokemonParty,
			instance::PokemonInstance,
		},
		texture::PokemonTexture,
	},
	storage::player::PlayerSave,
};
use pokemon::PokemonOption;

use crate::{
	state::{
		BattleState,
		MoveState,
	},
	pokemon::{
		BattleParty,
		ActivePokemonIndex,
		BattleAction,
		BattleMove,
	},
	gui::{
		BattleGui,
		BattleGuiPosition,
		battle_party_gui,
	},
	manager::BattleCloserManager,
};

pub mod state;
pub mod manager;

pub mod pokemon;
pub mod gui;
pub mod transitions;

pub static BATTLE_RANDOM: Random = Random::new();

pub struct Battle {

	battle_type: BattleType,
	trainer: Option<TrainerData>,
	
	pub player: BattleParty,
	pub opponent: BattleParty,

	// #[deprecated]
	pub winner: Option<BattleTeam>,

	state: BattleState,
	
}

impl Battle {

	#[deprecated]
	pub const TEMP_ACTIVE: usize = 0;

	pub const DEFAULT_ACTIVE: usize = 0;
	
	pub fn new(player: &SavedPokemonParty, data: BattleData) -> Option<Self> {		
		if !(
			player.is_empty() || 
			data.party.is_empty() ||
			// Checks if player has any pokemon in party that aren't fainted (temporary)
			player.iter().filter(|pokemon| pokemon.current_hp.map(|hp| hp != 0).unwrap_or(true)).next().is_none()
		) {
			Some(
				Self {
					battle_type: data.get_type(),
					player: BattleParty::from_saved(player, data.size, PokemonTexture::Back, BattleGuiPosition::Bottom),
					opponent: BattleParty::new(data.party, data.size, PokemonTexture::Front, BattleGuiPosition::Top),
					trainer: data.trainer,
					state: BattleState::default(),
					winner: None,
				}
			)
		} else {
			None
		}
	}

	pub fn start_moves(&mut self) {	
		self.generate_opponent_moves();
		self.state = BattleState::Moving(MoveState::Start);
	}

	pub fn input(&mut self, gui: &mut BattleGui) {
		gui.text.input();
	}

	// some input happens here too!
	pub fn update(&mut self, delta: f32, gui: &mut BattleGui, closer: &mut BattleCloserManager, party_gui: &mut PartyGui, bag_gui: &mut BagGui) {

		for active in self.player.active.iter_mut() {
			active.status.update(delta);
			active.renderer.update_flicker(delta);
		}

		for active in self.opponent.active.iter_mut() {
			active.status.update(delta);
			active.renderer.update_flicker(delta);
		}

		match &mut self.state {

			// Select pokemon moves / items / party switches

		    BattleState::Selecting { index, started } => {

				// Check if there is an active slot at the current index

				if let Some(active) = self.player.active.get(*index) {

					// Checks if the active slot has a pokemon

					if active.pokemon.is_some() {

						if !*started {

							// Spawn the battle panel and update its text

							gui.panel.spawn();
							gui.panel.update_text(self.player.pokemon(*index).unwrap(), &self.opponent.active);
							*started = true;

						} else if active.queued_move.is_none() {
	
							// Checks if a move is queued from an action done in the GUI
	
							if bag_gui.is_alive() {
								if let Some(selected) = bag_gui.take_selected_despawn() {
									self.player.active[*index].queued_move = Some(BattleMove::UseItem(selected));
									// battle_gui.update_gui(self, false, false);
								}
							} else if party_gui.is_alive() {
								if let Some(selected) = party_gui.selected.take() {
									party_gui.despawn();
									self.player.active[*index].queued_move = Some(BattleMove::Switch(selected));
								}
							} else {
								let index = *index;
								if gui.panel.input(self, closer, party_gui, bag_gui) {
									if gui.panel.fight.input(self.player.pokemon(index).unwrap()) {
										// Despawn the panel, set the text for the battle text, and spawn the battle text.
	
										if let Some(pokemon_move) = 
											self.player.pokemon_mut(index).unwrap().moves.get_mut(gui.panel.fight.moves.cursor).map(|instance| instance.use_move()).flatten() {
											gui.panel.despawn();
							
											self.player.active[index].queued_move = Some(BattleMove::Move(pokemon_move, 
												match pokemon_move.target {
													game::pokedex::moves::target::MoveTarget::Player => todo!(),//game::pokedex::moves::target::MoveTargetInstance::Player,
													game::pokedex::moves::target::MoveTarget::Opponent => game::pokedex::moves::target::MoveTargetInstance::Opponent(gui.panel.fight.targets.cursor),
												}
											));
							
										}
									}
								}
							}
						} else {
							*index += 1;
							*started = false;
						}
	
					} else {
						*index += 1;
					}
				} else {
					self.start_moves();
				}
			},
		    BattleState::Moving(move_state) => {

				match move_state {
					MoveState::Start => {
	
						// Despawn the player button panel
						if gui.panel.is_alive() {
							gui.panel.despawn();
						}
						
						gui.text.reset();

						// Queue pokemon moves					
						self.state = BattleState::Moving(MoveState::Pokemon { queue: self.move_queue(), current: None });
					},
					MoveState::Pokemon { queue, current } => {

						// Check if there is a current action active

						if let Some((battle_move, pokemon, started)) = current.as_mut() {

							match battle_move {
							    BattleAction::Pokemon(battle_move) => match battle_move {
									BattleMove::Move(.., target) => {
										let (user, other) = match pokemon.team {
											BattleTeam::Player => (&mut self.player, &mut self.opponent),
											BattleTeam::Opponent => (&mut self.opponent, &mut self.player),
										};
										if !gui.text.is_finished() || gui.text.len() == 0 {
											if gui.text.can_continue && gui.text.current() == 0 && !*started {

												let (target, index) = match target {
													MoveTargetInstance::Opponent(index) => (match pokemon.team {
														BattleTeam::Player => &mut self.opponent,
														BattleTeam::Opponent => &mut self.player,
													}, *index)
												};
	
												if target.update_status(index, false) {
													target.active[index].renderer.flicker();
												}
	
												*started = true;
											}
											if gui.text.update(delta, #[cfg(debug_assertions)] "battle move update") { // if err
												*current = None;
											}
										} else {
											
											let (other, index) = match target {
												MoveTargetInstance::Opponent(index) => other.pokemon_mut_or_other(*index),
											};
	
											if other.is_faint() {
												// get team and index of fainted pokemon

												let team = match target {
													MoveTargetInstance::Opponent(_) => pokemon.team.other(),
													// _ => pokemon.team,
												};

												let target = ActivePokemonIndex { team, active: index };

												if team != BattleTeam::Opponent {
													let experience = exp_gain(self.battle_type, other);
													let level = if let Some((level, moves)) = user.pokemon_mut(pokemon.active).unwrap().add_exp(experience) {
														let instance = user.pokemon(pokemon.active).unwrap();
														if let Some(moves) = moves {
															gui.level_up.setup(pokemon.active, instance, moves);
															// battle_gui.level_up.wants_to_spawn = true;
															gui.level_up.spawn();
														}
														user.update_status(pokemon.active, false);
														Some(level)
													} else {
														None
													};
													let instance = user.pokemon(pokemon.active).unwrap();
													gui::text::player_gain_exp(&mut gui.text, instance.name(), instance.data.experience, level);
												}

												*current = Some((BattleAction::Faint(false), target, false));
												return;
											}

											*current = None;
										}
									}
									BattleMove::UseItem(..) => todo!("handle items"),
									BattleMove::Switch(new) => {
										if gui.text.is_finished() {
											*current = None;
										} else {
											gui.text.update(delta, #[cfg(debug_assertions)] "switch update");
											if gui.text.current() == 1 && !*started {
												let team = match pokemon.team {
													BattleTeam::Player => &mut self.player,
													BattleTeam::Opponent => &mut self.opponent,
												};
												team.replace_pokemon(pokemon.active, *new);
		
												*started = true;
											}
										}
									}
								}
							    BattleAction::Faint(waiting) => {
									if !*waiting {
										let party = match pokemon.team {
											BattleTeam::Player => &mut self.player,
											BattleTeam::Opponent => &mut self.opponent,
										};
						
										if party.active[pokemon.active].pokemon.is_some() {
											if !*started {
						
												// Add text
							
												gui.text.clear();
												gui::text::on_faint(&mut gui.text, party.pokemon(pokemon.active).unwrap().name());
												gui.text.spawn();
							
							
												*started = true;
											} else {
												if !gui.text.is_finished() || !party.active[pokemon.active].renderer.faint_finished() {
													gui.text.update(delta, #[cfg(debug_assertions)] "faint update");
													if !party.active[pokemon.active].renderer.faint_finished() {
														party.active[pokemon.active].renderer.update_faint(delta);
													};
												} else {
													match pokemon.team {
														BattleTeam::Player => {
															party.remove_pokemon(pokemon.active);
															if party.all_fainted() {
																gui.panel.despawn();
																self.winner = Some(BattleTeam::Opponent);
																closer.spawn_closer(self);
																return;
															} else if party.any_inactive() {										
																battle_party_gui(party_gui, party, false);
															} else {
																*current = None;
																return;							
															}
														}
														BattleTeam::Opponent => {
							
															if self.opponent.all_fainted() {
																gui.panel.despawn();
																self.winner = Some(BattleTeam::Player);
																closer.spawn_closer(self);
																// self.move_queue = MoveQueueOption:;
															} else {
																let available: Vec<usize> = self.opponent.pokemon.iter()
																	// .map(|(index, pokemon)| pokemon.as_ref().map(|pokemon| (index, pokemon)))
																	.flatten()
																	.enumerate()
																	.filter(|(_, pokemon)| pokemon.current_hp != 0)
																	.map(|(index, _)| index)
																	.collect();
							
																self.opponent.remove_pokemon(pokemon.active);
																if !available.is_empty() {
																	self.opponent.queue_replace(pokemon.active, available[BATTLE_RANDOM.gen_range(0, available.len())]);
																}
																	
																// Reset the pokemon renderer so it renders pokemon
										
																// battle_gui.panel.start();
									
																let index = pokemon.active;
						
																*current = None;
																// self.move_queue = MoveQueueOption::Some(next.clone());
									
																// Update the opponent's pokemon GUI
										
																self.opponent.update_status(index, true);
						
																// gui.update_gui(self, false, true);
															}
															return;
														}
													}
													gui.text.despawn();
													*waiting = true;
													// self.move_queue = MoveQueueOption::FaintWait(*pokemon, next.clone(), None);
												}
											}
										}
									} else {
										match pokemon.team {
											BattleTeam::Player => {
												if !gui.text.is_alive() {
													if party_gui.is_alive() {
														if let Some(selected) = party_gui.selected.take() {
															party_gui.despawn();
															self.player.queue_replace(pokemon.active, selected);
															gui::text::on_go(&mut gui.text, self.player.pokemon[selected].as_ref().unwrap());
															gui.text.spawn();				
														}
													} else {
														battle_party_gui(party_gui, &self.player, false);
													}
												} else {
													if !gui.text.is_finished() {
														gui.text.update(delta, #[cfg(debug_assertions)] "faint wait update");
													} else {
														// self.move_queue = MoveQueueOption::Some(next.clone());
														*current = None;
													}
												}
											}
											BattleTeam::Opponent => unreachable!("Battle does not wait on opponent faint!"),
										}
									}
								}
							}
						} else if let Some(pokemon) = queue.pop_front() {
							let (user, other) = match pokemon.team {
								BattleTeam::Player => (&mut self.player, &mut self.opponent),
								BattleTeam::Opponent => (&mut self.opponent, &mut self.player),
							};
							if user.pokemon(pokemon.active).is_some() { // maybe need to check if fainted too
								if let Some(queued_move) = user.active[pokemon.active].queued_move.take() {
									*current = Some((BattleAction::Pokemon(queued_move), pokemon, false));
									gui.text.clear();
									if user.active[pokemon.active].pokemon.is_some() {
										match queued_move {
											BattleMove::Move(pokemon_move, target) => {
												let (user, target) = match target {
													// MoveTargetInstance::Player => (None, user.active_mut(pokemon.index)),
													// MoveTargetInstance::Team(index) => (user.active(pokemon.index), user.active_mut(index)),
													MoveTargetInstance::Opponent(index) => (user.pokemon_mut(pokemon.active).unwrap(), &mut other.active[index]),
												};
												if let Some(pokemon) = target.pokemon.as_mut() {
													do_move(pokemon_move, user, pokemon);
												}
												if let Some(pokemon) = target.pokemon.as_ref() {
													gui::text::on_move(&mut gui.text, pokemon_move, user, pokemon);
												}
											}
											BattleMove::UseItem(item) => {
												todo!()
												// if let Some(pokemon) = user.active_mut(pokemon.index) {
												// 	pokemon.pokemon_mut().execute_item(item);
												// 	gui.update_gui(self, true, false);
												// }
											}
											BattleMove::Switch(new) => {
												// if let Some(leaving) = user.pokemon(pokemon.index) {
													gui::text::on_switch(&mut gui.text, user.pokemon(pokemon.active).unwrap(), user.pokemon[new].as_ref().unwrap());
												// }
											}
										}
									}
									gui.text.spawn();
								}
							}
						} else {
							*move_state = MoveState::Post;
						}
					},
					MoveState::Post => {
						self.player.replace();
						self.opponent.replace();
						// if started { stuff } else start and do calculations and add text
						self.state = BattleState::default();
						// Once the text is finished, despawn it
						gui.text.despawn();
			 			// Spawn the player panel
						gui.panel.spawn();
					},
				}
			}
		}
	}
	
	pub fn render(&self, gui: &BattleGui) {
		gui.background.render(0.0);
		for active in self.opponent.active.iter() {
			active.renderer.render(game::macroquad::prelude::Vec2::default());
			active.status.render();
		}
		match &self.state {
		    BattleState::Selecting { index, .. } => {
				for (current, active) in self.player.active.iter().enumerate() {
					if current.eq(index) {
						active.renderer.render(game::macroquad::prelude::Vec2::new(0.0, gui.bounce.offset));
						active.status.render_offset(0.0, -gui.bounce.offset);
					} else {
						active.renderer.render(game::macroquad::prelude::Vec2::default());
						active.status.render();
					}
				}
				gui.render_panel();
			},
			BattleState::Moving( .. ) => {
				for active in self.player.active.iter() {
					active.renderer.render(game::macroquad::prelude::Vec2::default());
					active.status.render();
				}
				gui.render_panel();
				gui.text.render(#[cfg(debug_assertions)] "moving render");
			}
		}
	}

	pub fn move_queue(&self) -> VecDeque<ActivePokemonIndex> {
		let mut queue = VecDeque::with_capacity(self.player.active.len() + self.opponent.active.len());

		for (index, active) in self.player.active.iter().enumerate() {
			if let Some(battle_move) = active.queued_move {
				match battle_move {
					BattleMove::Move(..) => (),
					_ => {
						queue.push_back(ActivePokemonIndex { team: BattleTeam::Player, active: index });
					}
				}
			}
		}

		for (index, active) in self.opponent.active.iter().enumerate() {
			if let Some(battle_move) = &active.queued_move {
				match battle_move {
					BattleMove::Move(..) => (),
					_ => {
						queue.push_back(ActivePokemonIndex { team: BattleTeam::Opponent, active: index });
					}
				}
			}
		}

		let mut pokemon_map = std::collections::BTreeMap::new();

		for (index, active) in self.player.active.iter().enumerate() {
			if let Some(battle_move) = &active.queued_move {
				if let PokemonOption::Some(_, pokemon) = &active.pokemon {
					if let BattleMove::Move(..) = battle_move {
						pokemon_map.insert(pokemon.base.speed, ActivePokemonIndex { team: BattleTeam::Player, active: index });
					}
				}
			}
		}

		for (index, active) in self.opponent.active.iter().enumerate() {
			if let Some(battle_move) = &active.queued_move {
				if let PokemonOption::Some(_, pokemon) = &active.pokemon {
					if let BattleMove::Move(..) = battle_move {
						pokemon_map.insert(pokemon.base.speed, ActivePokemonIndex { team: BattleTeam::Opponent, active: index });
					}
				}
			}
		}

		queue.extend(pokemon_map.values().rev());

		queue

	}

	// warning: ignores PP
	pub fn generate_opponent_moves(&mut self) {
		for active in self.opponent.active.iter_mut() {
			if let PokemonOption::Some(_, pokemon) = &active.pokemon {
				let index = crate::BATTLE_RANDOM.gen_range(0, pokemon.moves.len());
				active.queued_move = Some(
					BattleMove::Move(
						pokemon.moves[index].pokemon_move, 
						MoveTargetInstance::Opponent(
							crate::BATTLE_RANDOM.gen_range(0, self.player.active.len())
						)
					)
				);
			}
		}
	}

	pub fn update_data(self, player: &mut PlayerSave) -> Option<(BattleTeam, bool)> {

		let trainer = self.trainer.is_some();

		if let Some(winner) = self.winner {
			match winner {
			    BattleTeam::Player => {
					if let Some(trainer) = self.trainer {
						player.worth += trainer.worth as u32;
					}		
				}
			    BattleTeam::Opponent => {
				}
			}
		}

		player.party = self.player.collect_owned().into_iter().map(|instance| instance.to_saved()).collect();

		self.winner.map(|winner| (winner, trainer))
		
	}
	
}

pub fn exp_gain(battle_type: BattleType, target: &PokemonInstance) -> game::pokedex::pokemon::Experience {
	((target.pokemon.training.base_exp * target.data.level as u16) as f32 * match battle_type {
		BattleType::Wild => 1.0,
		_ => 1.5,
	} / 7.0) as game::pokedex::pokemon::Experience
}

fn do_move(pokemon_move: MoveRef, user: &mut PokemonInstance, target: &mut PokemonInstance) -> bool {
	if let Some(script) = &pokemon_move.script {
		// for action in &script.actions {
			let actions = match &script.action {
				MoveAction::Action(action) => Some(*action),
				MoveAction::Persistent(persistent) => {
					target.persistent = Some(game::pokedex::moves::persistent::PersistentMoveInstance {
						pokemon_move,
						actions: &persistent.action,
						remaining: persistent.length.map(|(min, max)| BATTLE_RANDOM.gen_range(min, max)),
						should_do: persistent.on_move,
					});
					None
				}
			};
			if let Some(action) = actions {
				move_action(action, user, target)
			} else {
				false
			}
		// }
	} else {
		if pokemon_move.accuracy.map(|accuracy| {
			let hit: u8 = BATTLE_RANDOM.gen_range(0, 100);
			hit < accuracy
		}).unwrap_or(true) {
			if let Some(power) = pokemon_move.power {
				return damage_kind(user, target, DamageKind::Move(power, pokemon_move.category, pokemon_move.pokemon_type)) > 0;
			}
		}
		false
	}
}

pub fn get_move_damage(power: game::pokedex::moves::Power, category: MoveCategory, pokemon_type: game::pokedex::pokemon::types::PokemonType, user: &PokemonInstance, target: &PokemonInstance) -> Health {
	let effective = target.effective(pokemon_type);
	if effective == game::pokedex::pokemon::types::effective::Effective::Ineffective {
		return 0;
	}
	let effective = effective.multiplier() as f64;
	let (atk, def) = match category {
		MoveCategory::Physical => (user.base.atk as f64, target.base.def as f64),
		MoveCategory::Special => (user.base.sp_atk as f64, target.base.sp_def as f64),
		MoveCategory::Status => (0.0, 0.0),
	};
	(
		(((((2.0 * user.data.level as f64 / 5.0 + 2.0).floor() * atk * power as f64 / def).floor() / 50.0).floor() * effective) + 2.0)
		* (BATTLE_RANDOM.gen_range(85, 101u8) as f64 / 100.0)
		* (if pokemon_type == user.pokemon.data.primary_type { 1.5 } else { 1.0 })
	) as Health
}

fn move_action(action: MoveActionType, user: &mut PokemonInstance, target: &mut PokemonInstance) -> bool {
	match action {
	    MoveActionType::Damage(damage) => {
			damage_kind(user, target, damage);
			true
		},
	    MoveActionType::Status(chance, effect) => {
			chance_status(target, chance, effect)
		},
	    MoveActionType::Drain(damage, percent) => {
			let damage = damage_kind(user, target, damage) as f32;
			user.current_hp += (damage * percent) as Health;
			if user.current_hp > user.base.hp {
				user.current_hp = user.base.hp;
			}
			true
		}
	}
}

fn damage_kind(user: &PokemonInstance, target: &mut PokemonInstance, damage: DamageKind) -> Health {
	let damage = match damage {
		DamageKind::Move(power, category, pokemon_type) => {
			get_move_damage(power, category, pokemon_type, user, target)
		}
		DamageKind::PercentCurrent(percent) => {
			(target.current_hp as f32 * percent) as Health
		}
		DamageKind::PercentMax(percent) => {
			(target.base.hp as f32 * percent) as Health
		}
		DamageKind::Constant(damage) => damage,
	};
	target.current_hp = target.current_hp.saturating_sub(damage);
	damage
}

fn chance_status(target: &mut PokemonInstance, chance: u8, effect: game::pokedex::pokemon::status::StatusEffect) -> bool {
	if target.data.status.is_none() {
		if chance >= BATTLE_RANDOM.gen_range(1, 11) {
			target.data.status = Some(effect);
			true
		} else {
			false
		}
	} else {
		false
	}
}