extern crate firecore_game as game;

use game::pokedex::moves::script::DamageKind;
use game::pokedex::moves::script::MoveAction;
use game::pokedex::pokemon::Health;
use game::{
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
		BattleWinner,
	},
	pokedex::{
		moves::{
			MoveCategory,
			PokemonMove,
			instance::MoveInstance,
		},
		pokemon::{
			party::PokemonParty,
			texture::PokemonTexture,
			instance::PokemonInstance,
		},
	},
	data::player::PlayerSave,
	macroquad::{
		rand::Random,
		prelude::{info, Vec2},
	}
};
use pokemon::BattleMoveStatus;
use pokemon::BattleMoveType;

use self::gui::BattleGui;
use self::gui::pokemon::PokemonGui;
use self::gui::pokemon::party::battle_party_gui;
use self::pokemon::BattleParty;
use self::transitions::managers::closer::BattleCloserManager;

pub mod manager;

pub mod pokemon;
pub mod gui;
pub mod transitions;

pub static BATTLE_RANDOM: Random = Random::new();

// #[deprecated(since = "0.4.0", note = "Move to seperate crate")]
pub struct Battle {

	battle_type: BattleType,
	trainer: Option<TrainerData>,
	
	pub player: BattleParty,
	pub opponent: BattleParty,

	pub winner: Option<BattleWinner>,
	post_run: bool,
	try_run: bool,

	
}

impl Battle {
	
	pub fn new(player: &PokemonParty, data: BattleData) -> Option<Self> {		
		if !(
			player.is_empty() || 
			data.party.is_empty() ||
			// Checks if player has any pokemon in party that aren' fainted (temporary)
			player.iter().filter(|pokemon| pokemon.current_hp.map(|hp| hp != 0).unwrap_or(true)).next().is_none()
		) {
			Some(
				Self {
					battle_type: data.get_type(),
					player: BattleParty::from_saved(player, PokemonTexture::Back, Vec2::new(40.0, 113.0)),
					opponent: BattleParty::new(data.party, PokemonTexture::Front, Vec2::new(144.0, 74.0)),
					trainer: data.trainer,
					winner: None,
					post_run: false,
					try_run: false,
				}
			)
		} else {
			None
		}
	}

	pub fn update(&mut self, delta: f32, battle_gui: &mut BattleGui, closer: &mut BattleCloserManager, party_gui: &mut PartyGui, bag_gui: &mut BagGui) {
		
		if self.try_run {
			if self.battle_type == BattleType::Wild {
				closer.spawn_closer(self);
			}
		}

		// 

		if bag_gui.is_alive() {
			if let Some(selected) = bag_gui.take_selected_despawn() {
				self.player.next_move = Some(BattleMoveStatus::new(BattleMoveType::UseItem(selected)));
				// self.player.active_mut().use_item(selected);
				battle_gui.battle_text.run(self);

				battle_gui.update_gui(self, false, false);
			}
		}

		// Test if there is a pokemon being selected in the party gui while it is alive

		else if party_gui.is_alive() {
			if let Some(selected) = party_gui.selected.take() {

				party_gui.despawn();

				if self.player.active().is_faint() {

					self.player.select_pokemon(selected as usize);

					// battle_gui.player_pokemon_gui.exp_bar.update_exp(self.player.active(), true); // level up is true to reset the xp display width
					battle_gui.update_gui(&self, true, false);
	
					battle_gui.panel.start();

				} else {

					self.player.next_move = Some(BattleMoveStatus::new(BattleMoveType::Switch(selected as usize)));
					battle_gui.battle_text.run(self);

				}
				
			}
		}

		// Update the level up move thing

		else if battle_gui.level_up.is_alive() {

			battle_gui.level_up.update(delta, self.player.active_mut());

		}

		// Update the battle text
		
		else if battle_gui.battle_text.text.is_alive() {
			if !battle_gui.battle_text.text.is_finished() {

				// Despawn the player button panel

				if battle_gui.panel.is_alive() {
					battle_gui.panel.despawn();
				}

				// Perform the player's move

				if battle_gui.battle_text.perform_player(self) {

					self.player_move(battle_gui);

					battle_gui.battle_text.on_move(self.opponent.active(), &mut battle_gui.opponent);

					// Handle opponent fainting to player's move

					if self.opponent.active().is_faint() {

						// add exp to player

						let gain = self.exp_gain();
						self.player.active_mut().data.experience += gain * 5;

						// get the maximum exp a player can have at their level

						let max_exp = {
							let player = self.player.active();
							player.pokemon.training.growth_rate.level_exp(player.data.level)
						};

						// level the player up if they reach a certain amount of exp (and then subtract the exp by the maximum for the previous level)

						let level = if self.player.active().data.experience > max_exp {
							self.player.pokemon[self.player.active].pokemon.data.level += 1;
							self.player.pokemon[self.player.active].pokemon.data.experience -= max_exp;
							let player = self.player.active_mut();

							let mut moves = player.moves_at_level();

							while player.moves.len() < 4 && !moves.is_empty() {
								info!("{} learned {}!", player.name(), moves[0].name);
								player.moves.push(MoveInstance::new(moves.remove(0)));
							}

							if !moves.is_empty() {
								battle_gui.level_up.setup(player, moves);
								// battle_gui.level_up.wants_to_spawn = true;
								battle_gui.level_up.spawn();
							}

							

							// info!("{} levelled up to Lv. {}", &player.pokemon.data.name, player.level);
							Some(player.data.level)
						} else {
							// info!("{} gained {} exp. {} is needed to level up!", self.player.active().pokemon.data.name, gain, max_exp - self.player.active().exp);
							None
						};

						// add the exp gain and level up text to the battle text

						let player = self.player.active();
						battle_gui.player.update_gui(player, false);
						battle_gui.battle_text.player_level_up(player.name(), player.data.experience, level);

					}

					// make sure the actions do not repeat

					// self.player.next_move = None; // queued = false;

				} else

				// Perform the opponent's move

				if battle_gui.battle_text.perform_opponent(self) {

					self.opponent_move();

					// Update the player's health bar and add faint text if the player has fainted

					battle_gui.battle_text.on_move(self.player.active(), &mut battle_gui.player);

					// make sure the actions do not repeat

					// self.opponent.next_move = None; // queued = false;

				} else

				if battle_gui.battle_text.perform_post(self) {

					self.post_move();

					battle_gui.update_gui(self, false, false);

				}

				// Update the text (so it scrolls)

				battle_gui.battle_text.text.update(delta);

				self.player.renderer.update_other(delta);
				self.opponent.renderer.update_other(delta);

				// if a pokemon has fainted, remove them from screen gradually using BattlePokemonTextureHandler (bad name)

				if let Some(faint_index) = battle_gui.battle_text.faint_index {
					if battle_gui.battle_text.text.can_continue && battle_gui.battle_text.text.current_message() == faint_index {
						if self.player.active().is_faint() {

							if !self.player.renderer.is_finished() {
								self.player.renderer.update_faint(delta);
							}

						} else if self.opponent.active().is_faint() {

							if !self.opponent.renderer.is_finished() {
								self.opponent.renderer.update_faint(delta);
							}

						}
					}
				}
				
			// } else if battle_gui.level_up.wants_to_spawn {
			// 	battle_gui.level_up.spawn();
			} else {

				// Handle player fainting

				if self.player.active().is_faint() {

					/*
					*	If the player's active pokemon has fainted, check if the player has whited out,
					*	and if so, end the battle, else spawn the party menu to let the player pick another
					*	pokemon to use in battle.
					*/  

					if self.player.all_fainted() {
						battle_gui.panel.despawn();
						self.winner = Some(BattleWinner::Opponent);
						closer.spawn_closer(self);
					} else {

						battle_party_gui(party_gui, &self.player);
						
						// Reset the pokemon renderer so it renders pokemon

						self.player.renderer.reset();

					}

				}
				
				// Handle opponent fainting
				
				else if self.opponent.active().is_faint() {

					// check if all of the opponent's pokemon have fainted, and if so, end the battle, else select a pokemon from the opponent's party
					
					if self.opponent.all_fainted() {
						battle_gui.panel.despawn();
						self.winner = Some(BattleWinner::Player);
						closer.spawn_closer(self);
					} else {
						let available: Vec<usize> = self.opponent.pokemon.iter().enumerate()
							.filter(|(_, pkmn)| pkmn.pokemon.current_hp != 0)
							.map(|(index, _)| index)
							.collect();
						self.opponent.select_pokemon(available[BATTLE_RANDOM.gen_range(0..available.len() as u32) as usize]);

						// Update the opponent's pokemon GUI

						battle_gui.update_gui(self, false, true);

						// Reset the pokemon renderer so it renders pokemon
	
						self.opponent.renderer.reset();

						battle_gui.panel.start();
						
					}

					// Once the text is finished, despawn it

					battle_gui.battle_text.text.despawn(); 
					

				}
				
				// Handle normal move case (no one faints, all moves were completed)

				else {
					// Once the text is finished, despawn it
					battle_gui.battle_text.text.despawn();
					// Spawn the player panel
					battle_gui.panel.start();
				}				
			}
		}

	}
	
	pub fn render_pokemon(&self, y_offset: f32) {
		self.player.renderer.render(self.player.active_texture(), y_offset);
		self.opponent.renderer.render(self.opponent.active_texture(), 0.0);
	}

	pub fn player_first(&self) -> bool {
		if let Some(player) = self.player.next_move.as_ref() {
			match player.action {
			    BattleMoveType::Move(_) => {
					self.player.active().base.speed >= self.opponent.active().base.speed
				}
			    BattleMoveType::UseItem(_) => {
					true
				}
			    BattleMoveType::Switch(_) => {
					// if let Some(opponent) = self.opponent.next_move.as_ref() {
					// 	if let BattleMoveType::Move(pokemon_move) = opponent.action {
					// 		pokemon_move.use_before_switch
					// 	} else {
					// 		true
					// 	}
					// } else {
						true
					// }
				}
			}
		} else {
			false
		}
	}

	pub fn player_move(&mut self, battle_gui: &mut BattleGui) {

		if let Some(remaining) = self.player.active_mut().data.status.map(|effect | effect.remaining).flatten().as_mut() {
			*remaining -= 1;
			if *remaining == 0 {
				self.player.active_mut().data.status = None;
			}
		}

		if let Some(move_type) = self.player.next_move.take() {
			match move_type.action {
			    BattleMoveType::Move(pokemon_move) => {
					if let Some(script) = pokemon_move.battle_script.as_ref() {
						self.player.renderer.move_actions = Some(script.actions.clone());
					}

					if let Some(script) = &pokemon_move.script {
						// script.conditions
						// let player = self.player.pokemon.get_mut(self.player.active).unwrap();
						for action in &script.actions {
							match action {
							    MoveAction::Damage(damage) => {
									let opponent = self.opponent.active_mut();
									let damage = match *damage {
									    DamageKind::PercentCurrent(percent) => {
											(opponent.current_hp as f32 * (1.0 - percent)) as Health
										}
									    DamageKind::PercentMax(percent) => {
											(opponent.base.hp as f32 * (1.0 - percent)) as Health
										}
									    DamageKind::Constant(damage) => damage,
									};
									opponent.current_hp = opponent.current_hp.saturating_sub(damage);
									if damage != 0 {
										self.opponent.renderer.flicker();
									}
								}
							    MoveAction::Status(chance, effect) => {
									if *chance >= BATTLE_RANDOM.gen_range(1..11) as u8 {
										self.opponent.active_mut().data.status = Some(*effect);
									}
								}
							    MoveAction::Persist(persistent, current_turn) => {
									let opponent = self.opponent.active_mut();
									opponent.persistent.push(*persistent.clone());
									if *current_turn {
										opponent.run_persistent_moves();
									}
								}
							}
						}
					}


					let damage = get_move_damage(pokemon_move, self.player.active(), self.opponent.active());
					let opponent = &mut self.opponent.active_mut();
					opponent.current_hp = opponent.current_hp.saturating_sub(damage);
					if damage != 0 {
						self.opponent.renderer.flicker();
					}
				}
			    BattleMoveType::UseItem(item) => {
					self.player.active_mut().execute_item(item);
					battle_gui.update_gui(&self, true, false);
				}
			    BattleMoveType::Switch(selected) => {
					self.player.select_pokemon(selected as usize);
					battle_gui.update_gui(&self, true, false);
				}
			}
		}
		
	}

	pub fn opponent_move(&mut self) {
		if let Some(move_type) = self.opponent.next_move.take() {
			if let BattleMoveType::Move(pokemon_move) = move_type.action {
				let damage = get_move_damage(pokemon_move, self.opponent.active(), self.player.active());
				let player = self.player.active_mut();
				player.current_hp = player.current_hp.saturating_sub(damage);
				if damage != 0 {
					self.player.renderer.flicker();
				}
			}
		}		
	}

	pub fn post_move(&mut self) {
		// flicker here
		self.player.active_mut().run_persistent_moves();
		self.opponent.active_mut().run_persistent_moves();
		self.post_run = false;
	}

	// warning: ignores PP
	pub fn generate_opponent_move(&mut self) {
		let index = crate::BATTLE_RANDOM.gen_range(0..self.opponent.active().moves.len() as u32) as usize;
		self.opponent.next_move = Some(BattleMoveStatus::new(BattleMoveType::Move(self.opponent.active_mut().moves[index].pokemon_move)));
		self.post_run = true;
	}

	pub fn update_data(self, player: &mut PlayerSave) -> Option<(BattleWinner, bool)> {

		let trainer = self.trainer.is_some();

		if let Some(winner) = self.winner {
			match winner {
			    BattleWinner::Player => {
					if let Some(trainer) = self.trainer {
						player.worth += trainer.worth as usize;
					}		
				}
			    BattleWinner::Opponent => {
				}
			}
		}
		
		player.party = self.player.pokemon.into_iter().map(|pokemon| {
			pokemon.pokemon.to_saved()
		}).collect();

		self.winner.map(|winner| (winner, trainer))
		
	}

	pub fn try_run(&mut self) {
		self.try_run = true;
	}

	fn exp_gain(&self) -> u32 {
		((self.opponent.active().pokemon.training.base_exp * self.opponent.active().data.level as u16) as f32 * match self.battle_type {
			BattleType::Wild => 1.0,
			_ => 1.5,
		} / 7.0) as u32
	}
	
}

pub fn get_move_damage(pokemon_move: &PokemonMove, pokemon: &PokemonInstance, recieving_pokemon: &PokemonInstance) -> u16 {
	if pokemon_move.accuracy.map(|accuracy| {
		let hit: u8 = BATTLE_RANDOM.gen_range(0..100) as u8;
		hit < accuracy
	}).unwrap_or(true) {
		if let Some(power) = pokemon_move.power {
			let effective = recieving_pokemon.move_effective(pokemon_move);
			if effective == game::pokedex::pokemon::types::effective::Effective::Ineffective {
				return 0;
			}
			let effective = effective.multiplier() as f64;
			let (atk, def) = match pokemon_move.category {
			    MoveCategory::Physical => (pokemon.base.atk as f64, recieving_pokemon.base.def as f64),
			    MoveCategory::Special => (pokemon.base.sp_atk as f64, recieving_pokemon.base.sp_def as f64),
			    MoveCategory::Status => (0.0, 0.0),
			};
			(
				(((((2.0 * pokemon.data.level as f64 / 5.0 + 2.0).floor() * atk * power as f64 / def).floor() / 50.0).floor() * effective) + 2.0)
			 	* (BATTLE_RANDOM.gen_range(85..101) as f64 / 100.0)
				* (if pokemon_move.pokemon_type == pokemon.pokemon.data.primary_type { 1.5 } else { 1.0 })
			) as Health
		} else {
			0
		}
	} else {
		info!("{} missed!", pokemon);
		0
	}	
}