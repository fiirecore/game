// #![feature(map_into_keys_values)] // for move queue fn ~line 558

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
		BattleEntry,
		BattleTrainerEntry,
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
use pokemon::ActivePokemon;
use pokemon::PokemonOption;
use state::MoveQueue;

use crate::pokemon::BattleActionInstance;
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

	pub data: BattleData,
	
	pub player: BattleParty,
	pub opponent: BattleParty,

	// #[deprecated]
	

	state: BattleState,
	
}

pub struct BattleData {
	battle_type: BattleType,
	trainer: Option<BattleTrainerEntry>,
	pub winner: Option<BattleTeam>,
}

impl Battle {

	#[deprecated]
	pub const TEMP_ACTIVE: usize = 0;

	pub const DEFAULT_ACTIVE: usize = 0;
	
	pub fn new(player: &SavedPokemonParty, entry: BattleEntry) -> Option<Self> {		
		if !(
			player.is_empty() || 
			entry.party.is_empty() ||
			// Checks if player has any pokemon in party that aren't fainted (temporary)
			player.iter().filter(|pokemon| pokemon.current_hp.map(|hp| hp != 0).unwrap_or(true)).next().is_none()
		) {
			Some(
				Self {
					data: BattleData {
						battle_type: entry.get_type(),
						trainer: entry.trainer,
						winner: None,
					},
					player: BattleParty::from_saved(player, entry.size, PokemonTexture::Back, BattleGuiPosition::Bottom),
					opponent: BattleParty::new(entry.party, entry.size, PokemonTexture::Front, BattleGuiPosition::Top),
					state: BattleState::default(),
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
		match &mut self.state {

			// Select pokemon moves / items / party switches

		    BattleState::Selecting(index) => {

				// Check if there is an active slot at the current index

				match self.player.active.get_mut(*index) {
				    Some(active) => match active.pokemon.as_mut() {
						Some(pokemon) => match gui.panel.is_alive() {
							true => match active.queued_move.is_none() {
								true => {

									// Checks if a move is queued from an action done in the GUI
	
									if bag_gui.is_alive() {
										if let Some(selected) = bag_gui.take_selected_despawn() {
											active.queued_move = Some(BattleMove::UseItem(selected));
										}
									} else if party_gui.is_alive() {
										if let Some(selected) = party_gui.selected.take() {
											party_gui.despawn();
											active.queued_move = Some(BattleMove::Switch(selected));
										}
									} else {
										if let Some(panels) = gui.panel.input(pokemon) {
											match panels {
												gui::panels::BattlePanels::Main => {
													match gui.panel.battle.cursor {
														0 => gui.panel.active = gui::panels::BattlePanels::Fight,
														1 => bag_gui.spawn(false),
														2 => gui::battle_party_gui(party_gui, &self.player, true),
														3 => if self.data.battle_type == game::util::battle::BattleType::Wild {
															closer.spawn_closer(self);
														},
														_ => unreachable!(),
													}
												}
												gui::panels::BattlePanels::Fight => {
													if let Some(pokemon_move) = pokemon.moves.get_mut(gui.panel.fight.moves.cursor).map(|instance| instance.use_move()).flatten() {
														active.queued_move = Some(BattleMove::Move(pokemon_move, 
															match pokemon_move.target {
																game::pokedex::moves::target::MoveTarget::Player => todo!(),//game::pokedex::moves::target::MoveTargetInstance::Player,
																game::pokedex::moves::target::MoveTarget::Opponent => MoveTargetInstance::Opponent(gui.panel.fight.targets.cursor),
															}
														));
													}
												}
											}
										}
									}
								}
								false => {
									*index += 1;
									gui.panel.despawn();
								}
							}
							false => {
								gui.panel.spawn();
								gui.panel.setup(&mut active.last_move, pokemon, &self.opponent.active);
							}
						},
						None => *index += 1,
					}
				    None => self.start_moves(),
				}
			},
		    BattleState::Moving(move_state) => {

				match move_state {
					MoveState::Start => {
						// Despawn the player button panel
						gui.panel.despawn();
						gui.text.reset();
						*move_state = MoveState::SetupPokemon;
					}
					MoveState::SetupPokemon => {
						// Queue pokemon moves					
						*move_state = MoveState::Pokemon(MoveQueue::new(Self::move_queue(&mut self.player.active, &mut self.opponent.active)));
					},
					MoveState::Pokemon(queue) => {

						// Check if there is a current action active

						match queue.current.as_mut() {
						    None => {
								match queue.actions.pop_front() {
									Some(instance) => {
										let (user, other) = match instance.pokemon.team {
											BattleTeam::Player => (&mut self.player, &mut self.opponent),
											BattleTeam::Opponent => (&mut self.opponent, &mut self.player),
										};

										if let Some(pokemon) = user.active[instance.pokemon.active].pokemon.as_mut() {
											gui.text.clear();
											gui.text.spawn();
											match &instance.action {
												BattleAction::Pokemon(battle_move) => match battle_move {
													BattleMove::Move(pokemon_move, target) => {
														let (user, target, index) = match target {
															// MoveTargetInstance::Player => (None, user.active_mut(pokemon.index)),
															// MoveTargetInstance::Team(index) => (user.active(pokemon.index), user.active_mut(index)),
															MoveTargetInstance::Opponent(index) => (pokemon, &mut other.active[*index], ActivePokemonIndex { team: instance.pokemon.team.other(), active: *index }),
														};
														if let Some(pokemon) = target.pokemon.as_mut() {
															do_move(pokemon_move, user, pokemon);
															gui::text::on_move(&mut gui.text, pokemon_move, user, pokemon);

															let effective = pokemon.effective(pokemon_move.pokemon_type);
														
															if effective != game::pokedex::pokemon::types::effective::Effective::Effective && pokemon_move.category != MoveCategory::Status {
																queue.actions.push_front(BattleActionInstance { pokemon: index, action: BattleAction::Effective(effective) })
															}
															
															if target.status.update_gui(Some(pokemon), false) {
																target.renderer.flicker();														
															}
															if pokemon.is_faint() {
																queue.actions.push_front(BattleActionInstance { pokemon: index, action: BattleAction::Faint });
															}
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
															gui::text::on_switch(&mut gui.text, pokemon, user.pokemon[*new].as_ref().unwrap());
														// }
													}
												}
												BattleAction::Effective(effective) => {
													gui.text.clear();
													gui.text.push(
														game::text::MessagePage::new(
															vec![format!("It was {}{}", effective, if game::pokedex::pokemon::types::effective::Effective::SuperEffective.eq(effective) { "!" } else { "..." })], 
															Some(0.5)
														)
													);
													gui.text.spawn();
												}
											    BattleAction::Faint => {
													// Add text
								
													gui.text.clear();
													gui::text::on_faint(&mut gui.text, pokemon.name());
													gui.text.spawn();
													user.active[instance.pokemon.active].renderer.fainting.fainting = true;

												}
											    BattleAction::LevelUp => todo!(),
											    // BattleAction::Wait => todo!(),
											}
											queue.current = Some(BattleActionInstance { pokemon: instance.pokemon, action: instance.action });
										}
									},
									None => {
										*move_state = MoveState::SetupPost;
									}
								}
							},
						    Some(instance) => {

								let (user, other) = match instance.pokemon.team {
									BattleTeam::Player => (&mut self.player, &mut self.opponent),
									BattleTeam::Opponent => (&mut self.opponent, &mut self.player),
								};

								match &mut instance.action {

									BattleAction::Pokemon(battle_move) => match battle_move {

										BattleMove::Move(.., target) => {

											let active = match target {
												MoveTargetInstance::Opponent(index) => &mut other.active[*index],
											};

											if !gui.text.is_finished() {
												gui.text.update(delta);
											} else if active.renderer.is_flickering() || active.status.health_moving() {
												active.renderer.update_flicker(delta);
												active.status.update(delta);
											} else {

												println!("To - do: give pokemon exp.");
		
												// if other.is_faint() {
												// 	// get team and index of fainted pokemon
	
												// 	if team != BattleTeam::Opponent {
												// 		let experience = exp_gain(self.data.battle_type, other);
												// 		let level = if let Some((level, moves)) = user.pokemon_mut(active).unwrap().add_exp(experience) {
												// 			let pokemon = user.pokemon(active).unwrap();
												// 			if let Some(moves) = moves {
												// 				todo!()
												// 				// gui.level_up.spawn(instance.pokemon.active, pokemon, moves);
												// 			}
												// 			user.update_status(active, false);
												// 			Some(level)
												// 		} else {
												// 			None
												// 		};
												// 		let pokemon = user.pokemon(active).unwrap();
												// 		gui::text::player_gain_exp(&mut gui.text, pokemon.name(), pokemon.data.experience, level);
												// 	}
	
												// }

												queue.current = None;
	
											}											
										}
										BattleMove::UseItem(..) => todo!("handle items"),
										BattleMove::Switch(new) => {
											if gui.text.is_finished() {
												queue.current = None;
											} else {

												gui.text.update(delta);

												if gui.text.current() == 1 && user.pokemon[*new].is_some() {
													user.replace_pokemon(instance.pokemon.active, *new);
												}

											}
										}
									}
									BattleAction::Effective(..) => {
										if !gui.text.is_finished() {
											gui.text.update(delta);
										} else {
											queue.current = None;
										}
									}
									BattleAction::Faint => match instance.pokemon.team {
										BattleTeam::Player => {
											if !party_gui.is_alive() {
												if self.player.active[instance.pokemon.active].pokemon.is_some() {
													if !gui.text.is_finished() || !self.player.active[instance.pokemon.active].renderer.fainting.is_finished() {
														gui.text.update(delta);
														if !self.player.active[instance.pokemon.active].renderer.fainting.is_finished() {
															self.player.active[instance.pokemon.active].renderer.fainting.update(delta);
														};
													} else {
														self.player.remove_pokemon(instance.pokemon.active);
														if self.player.all_fainted() {
															gui.panel.despawn();
															self.data.winner = Some(BattleTeam::Opponent);
															closer.spawn_closer(self);
															return;
														} else if self.player.any_inactive() {										
															battle_party_gui(party_gui, &self.player, false);
														} else {
															queue.current = None;
															return;							
														}
													}
												}
											} else {

											}
										}
										BattleTeam::Opponent => {
											if !gui.text.is_finished() {
												gui.text.update(delta);
											} else if self.opponent.active[instance.pokemon.active].renderer.fainting.fainting {
												self.opponent.active[instance.pokemon.active].renderer.fainting.update(delta);
											} else {
												if self.opponent.all_fainted() {
													gui.panel.despawn();
													self.data.winner = Some(BattleTeam::Player);
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
				
													self.opponent.remove_pokemon(instance.pokemon.active);
	
													if !available.is_empty() {
														self.opponent.queue_replace(instance.pokemon.active, available[BATTLE_RANDOM.gen_range(0, available.len())]);
													}
	
													self.opponent.update_status(instance.pokemon.active, true);
	
													queue.current = None;
			
													// gui.update_gui(self, false, true);
												}
											}
										}
									}
									BattleAction::LevelUp => todo!(),
								    // BattleAction::Wait => todo!(),
								}
							}
						}
					},
					MoveState::SetupPost => {
						*move_state = MoveState::Post;
					},
					MoveState::Post => {
						*move_state = MoveState::End;
					}
					MoveState::End => {
						self.player.replace();
						self.opponent.replace();
						// if started { stuff } else start and do calculations and add text
						self.state = BattleState::default();
						// Once the text is finished, despawn it
						gui.text.despawn();
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
		    BattleState::Selecting(index) => {
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
				gui.panel.render();
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

	pub fn move_queue(player: &mut Box<[ActivePokemon]>, opponent: &mut Box<[ActivePokemon]>) -> VecDeque<BattleActionInstance> {

		use std::cmp::Reverse;

		#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
		enum Priority {
			First(ActivePokemonIndex),
			Second(Reverse<u8>, Reverse<u16>, ActivePokemonIndex), // priority, speed, pokemon <- fix last, player always goes first
		}

		fn insert(map: &mut std::collections::BTreeMap<Priority, BattleActionInstance>, team: BattleTeam, active: &mut Box<[ActivePokemon]>) {
			for (index, active) in active.iter_mut().enumerate() {
				if let (Some(pokemon), Some(battle_move)) = (active.pokemon.as_ref(), active.queued_move.take()) {
					let index = ActivePokemonIndex { team, active: index };
					map.insert(
						match battle_move {
							BattleMove::Move(..) => Priority::Second(Reverse(0), Reverse(pokemon.base.speed), index),
							_ => Priority::First(index),
						}, 
						BattleActionInstance { pokemon: index, action: BattleAction::Pokemon(battle_move) }
					);
				}
			}
		}

		let mut map = std::collections::BTreeMap::new();

		insert(&mut map, BattleTeam::Player, player);
		insert(&mut map, BattleTeam::Opponent, opponent);

		map.into_iter().map(|(_, i)| i).collect()

	}

	// warning: ignores PP
	#[deprecated(note = "use AIs instead")]
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

		let trainer = self.data.trainer.is_some();

		if let Some(winner) = self.data.winner {
			match winner {
			    BattleTeam::Player => {
					if let Some(trainer) = self.data.trainer {
						player.worth += trainer.worth as u32;
					}		
				}
			    BattleTeam::Opponent => {
				}
			}
		}

		player.party = self.player.collect_owned().into_iter().map(|instance| instance.to_saved()).collect();

		self.data.winner.map(|winner| (winner, trainer))
		
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