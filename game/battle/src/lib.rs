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
			target::MoveTargetInstance,
		},
		pokemon::saved::SavedPokemonParty,
		texture::PokemonTexture,
	},
	storage::player::PlayerSave,
};

use crate::{
	state::{
		BattleState,
		MoveState,
		MoveQueue
	},
	pokemon::{
		BattleParty,
		ActivePokemonIndex,
		ActivePokemonArray,
		BattleAction,
		BattleActionInstance,
		BattleMove,
		ai::BattleAi,
	},
	ui::{
		BattleGui,
		BattleGuiPosition,
		battle_party_gui,
		transitions::{
			BattleCloser,
			managers::closer::BattleCloserManager,
		}
	},
};

pub mod state;
pub mod manager;

pub mod pokemon;
pub mod ui;

pub static BATTLE_RANDOM: Random = Random::new();

pub struct Battle {

	pub data: BattleData,
	
	player: BattleParty,
	opponent: BattleParty, // to - do: move input handling (ai, on screen, over network)
	ai: BattleAi,
	state: BattleState,
	
}

pub struct BattleData {
	battle_type: BattleType,
	trainer: Option<BattleTrainerEntry>,
	pub winner: Option<BattleTeam>,
}

impl Battle {
	
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
					ai: BattleAi::Random,
				}
			)
		} else {
			None
		}
	}

	pub fn start_moves(&mut self) {	
		self.ai.moves(&mut self.opponent.active, &self.player.active);
		self.state = BattleState::Moving(MoveState::Start);
	}

	// input happens here too!
	pub fn update(&mut self, delta: f32, gui: &mut BattleGui, closer: &mut BattleCloserManager, party_gui: &mut PartyGui, bag_gui: &mut BagGui) {
		gui.bounce.update(delta);
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
												ui::panels::BattlePanels::Main => {
													match gui.panel.battle.cursor {
														0 => gui.panel.active = ui::panels::BattlePanels::Fight,
														1 => bag_gui.spawn(false),
														2 => ui::battle_party_gui(party_gui, &self.player, true),
														3 => if self.data.battle_type == game::util::battle::BattleType::Wild {
															closer.spawn(self, &mut gui.text);
														},
														_ => unreachable!(),
													}
												}
												ui::panels::BattlePanels::Fight => {
													if let Some(pokemon_move) = pokemon.moves.get(gui.panel.fight.moves.cursor).map(|instance| instance.get()).flatten() {
														active.queued_move = Some(BattleMove::Move(gui.panel.fight.moves.cursor, 
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
													BattleMove::Move(move_index, target) => {
														let (user, target, index) = match target {
															// MoveTargetInstance::Player => (None, user.active_mut(pokemon.index)),
															// MoveTargetInstance::Team(index) => (user.active(pokemon.index), user.active_mut(index)),
															MoveTargetInstance::Opponent(index) => (pokemon, &mut other.active[*index], ActivePokemonIndex { team: instance.pokemon.team.other(), active: *index }),
														};
														if let Some(pokemon) = target.pokemon.as_mut() {
															if let Some((pokemon_move, damaged)) = user.use_move(*move_index, pokemon) {
																if damaged {
																	target.renderer.flicker();
																}
																ui::text::on_move(&mut gui.text, pokemon_move, user);
	
																let effective = pokemon.effective(pokemon_move.pokemon_type);
															
																if effective != game::pokedex::pokemon::types::Effective::Effective && pokemon_move.category != MoveCategory::Status {
																	queue.actions.push_front(BattleActionInstance { pokemon: index, action: BattleAction::Effective(effective) })
																}
																
																target.status.update_gui(Some(pokemon), false);
																if pokemon.fainted() {
																	queue.actions.push_front(BattleActionInstance { pokemon: index, action: BattleAction::Faint });
																}
															}
														}
													}
													BattleMove::UseItem(item) => {
														pokemon.execute_item(item);
														ui::text::on_item(&mut gui.text, pokemon, item);
														user.active[instance.pokemon.active].update_status(false);
													}
													BattleMove::Switch(new) => {
														ui::text::on_switch(&mut gui.text, pokemon, user.pokemon[*new].as_ref().unwrap());
													}
												}
												BattleAction::Effective(effective) => {
													ui::text::on_effective(&mut gui.text, effective);
												}
											    BattleAction::Faint => {
													ui::text::on_faint(&mut gui.text, self.data.battle_type, pokemon);
													user.active[instance.pokemon.active].renderer.faint();
												}
											    // BattleAction::LevelUp => todo!(),
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
											} else if active.renderer.flicker.flickering() || active.status.health_moving() {
												active.renderer.flicker.update(delta);
												active.status.update_hp(delta);
											} else {

												game::macroquad::prelude::info!("To - do: give pokemon exp.");
		
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
										BattleMove::UseItem(..) => {
											if !gui.text.is_finished() {
												gui.text.update(delta)
											} else if user.active[instance.pokemon.active].status.health_moving() {
												user.active[instance.pokemon.active].status.update_hp(delta);
											} else {
												queue.current = None;
											}
										},
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
													if !gui.text.is_finished() || !self.player.active[instance.pokemon.active].renderer.faint.fainting() {
														gui.text.update(delta);
														if self.player.active[instance.pokemon.active].renderer.faint.fainting() {
															self.player.active[instance.pokemon.active].renderer.faint.update(delta);
														};
													} else {
														self.player.remove_pokemon(instance.pokemon.active);
														if self.player.all_fainted() {
															gui.panel.despawn();
															self.data.winner = Some(BattleTeam::Opponent);
															closer.spawn(self, &mut gui.text);
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
												party_gui.update(delta);
												party_gui.input();
												if let Some(selected) = party_gui.selected.take() {
													self.player.queue_replace(instance.pokemon.active, selected);
													queue.current = None;
												}
											}
										}
										BattleTeam::Opponent => {
											if self.opponent.active[instance.pokemon.active].renderer.faint.fainting() {
												self.opponent.active[instance.pokemon.active].renderer.faint.update(delta);
											} else if !gui.text.is_finished() {
												gui.text.update(delta);
											} else {
												if self.opponent.all_fainted() {
													gui.panel.despawn();
													self.data.winner = Some(BattleTeam::Player);
													closer.spawn(self, &mut gui.text);
													// self.move_queue = MoveQueueOption:;
												} else {
													let available: Vec<usize> = self.opponent.pokemon.iter()
														// .map(|(index, pokemon)| pokemon.as_ref().map(|pokemon| (index, pokemon)))
														.enumerate()
														.filter(|(_, pokemon)| pokemon.as_ref().map(|instance| !instance.fainted()).unwrap_or(false))
														.map(|(index, _)| index)
														.collect();
				
													// self.opponent.remove_pokemon(instance.pokemon.active);
	
													if !available.is_empty() {
														self.opponent.queue_replace(instance.pokemon.active, available[BATTLE_RANDOM.gen_range(0, available.len())]);
													}
	
													queue.current = None;
			
													// gui.update_gui(self, false, true);
												}
											}
										}
									}
									// BattleAction::LevelUp => todo!(),
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
			active.status.render(0.0, 0.0);
		}
		match &self.state {
		    BattleState::Selecting(index) => {
				for (current, active) in self.player.active.iter().enumerate() {
					if current.eq(index) {
						active.renderer.render(game::macroquad::prelude::Vec2::new(0.0, gui.bounce.offset));
						active.status.render(0.0, -gui.bounce.offset);
					} else {
						active.renderer.render(game::macroquad::prelude::Vec2::default());
						active.status.render(0.0, 0.0);
					}
				}
				gui.render_panel();
				gui.panel.render();
			},
			BattleState::Moving( .. ) => {
				for active in self.player.active.iter() {
					active.renderer.render(game::macroquad::prelude::Vec2::default());
					active.status.render(0.0, 0.0);
				}
				gui.render_panel();
				gui.text.render();
			}
		}
	}

	pub fn move_queue(player: &mut ActivePokemonArray, opponent: &mut ActivePokemonArray) -> VecDeque<BattleActionInstance> {

		use std::cmp::Reverse;

		#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
		enum Priority {
			First(ActivePokemonIndex),
			Second(Reverse<u8>, Reverse<u16>, ActivePokemonIndex), // priority, speed, pokemon <- fix last, player always goes first
		}

		fn insert(map: &mut std::collections::BTreeMap<Priority, BattleActionInstance>, team: BattleTeam, active: &mut ActivePokemonArray) {
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

		map.into_iter().map(|(_, i)| i).collect() // into_values

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

		player.party = self.player.collect_owned().into_iter().map(|instance| instance.into_saved()).collect();

		self.data.winner.map(|winner| (winner, trainer))
		
	}
	
}