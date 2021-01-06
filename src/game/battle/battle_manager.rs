use log::info;
use opengl_graphics::GlGraphics;
use piston_window::Context;
use crate::entity::entity::Ticking;
use crate::game::pokedex::pokedex::Pokedex;
use crate::gui::battle::battle_gui::BattleActivatable;
use crate::gui::battle::battle_gui::BattleGui;
use crate::gui::gui::GuiComponent;
use crate::util::traits::Completable;
use crate::util::traits::Loadable;
use crate::io::data::player_data::PlayerData;
use crate::engine::text::TextRenderer;

use crate::engine::game_context::GameContext;

use crate::game::battle::battle::Battle;

use crate::entity::entity::Entity;

use super::battle_context::BattleContext;
use super::transitions::managers::battle_closer_manager::BattleCloserManager;
use super::transitions::managers::battle_screen_transition_manager::BattleScreenTransitionManager;
use super::transitions::managers::battle_opener_manager::BattleOpenerManager;

pub struct BattleManager {	
	
	pub current_battle: Battle,
	
	battle_screen_transition_manager: BattleScreenTransitionManager,
	battle_opener_manager: BattleOpenerManager,
	battle_closer_manager: BattleCloserManager,

	pub battle_gui: BattleGui,

	pub finished: bool,
	
}

impl BattleManager {
	
	pub fn new() -> BattleManager {
		
		BattleManager {

			battle_screen_transition_manager: BattleScreenTransitionManager::new(),
			battle_opener_manager: BattleOpenerManager::new(),
			battle_closer_manager: BattleCloserManager::new(),
		
			current_battle: Battle::default(),

			battle_gui: BattleGui::new(),

			finished: false,

		}
		
	}

	pub fn world_active(&self) -> bool {
		return self.battle_screen_transition_manager.is_alive();
	}

}

impl Loadable for BattleManager {

	fn load(&mut self) {

		//self.player_intro.load();

		self.battle_screen_transition_manager.load();
		self.battle_screen_transition_manager.load_transitions();
		self.battle_opener_manager.load_openers();
		self.battle_closer_manager.load_closers();
		

		self.battle_gui.load();

	}

}

impl Completable for BattleManager {

	fn is_finished(&self) -> bool {
		self.finished
	}
	
}

impl BattleManager {

	pub fn on_start(&mut self, context: &mut GameContext, pokedex: &Pokedex, player_data: &PlayerData) { // add battle type parameter
		self.finished = false;
		self.create_battle(player_data, pokedex, &context.battle_context);
		self.battle_gui.despawn();
		self.battle_gui.spawn();		
		self.battle_screen_transition_manager.spawn();
		self.battle_screen_transition_manager.on_start(context);
	}

	pub fn create_battle(&mut self, player_data: &PlayerData, pokedex: &Pokedex, battle_context: &BattleContext) {
		let battle = Battle::new(pokedex, &player_data.party, &battle_context.battle_data.as_ref().unwrap().party);

		info!("Loading Battle: {}", battle);
		self.current_battle = battle;
		self.current_battle.load();
		self.battle_gui.on_battle_start(&self.current_battle);
	}

	pub fn update(&mut self, context: &mut GameContext, player_data: &mut PlayerData) {
		
		if self.battle_screen_transition_manager.is_alive() {
			if self.battle_screen_transition_manager.is_finished() {
				self.battle_screen_transition_manager.despawn();
				self.battle_opener_manager.spawn();
				self.battle_opener_manager.on_start(context);
				self.battle_opener_manager.battle_introduction_manager.setup_text(&self.current_battle);
			} else {
				self.battle_screen_transition_manager.update(context);
			}
		} else if self.battle_opener_manager.is_alive() {
			if self.battle_opener_manager.is_finished() {
				self.battle_opener_manager.despawn();
				self.battle_gui.player_panel.start();
			} else {
				self.battle_opener_manager.update(context);
				self.battle_opener_manager.battle_introduction_manager.update_gui(&mut self.battle_gui);
				//self.battle_gui.opener_update(context);
			}
		} else if self.battle_closer_manager.is_alive() {
			if self.battle_closer_manager.is_finished() {
				self.battle_closer_manager.despawn();
				self.finished = true;
			} else {
				self.battle_closer_manager.update(context);
			}
		} else /*if !self.current_battle.is_finished()*/ {
			//self.current_battle.update(context);
			self.battle_gui.update(context);
			if self.current_battle.pmove_queued || self.current_battle.omove_queued || self.current_battle.faint_queued {
				if self.battle_gui.opponent_pokemon_gui.health_bar.get_width() == 0 {
					self.battle_gui.update_gui(&self.current_battle);
				}
				if self.current_battle.player().base.speed > self.current_battle.opponent().base.speed {
					self.pmove();
				} else {
					self.omove();
				}
			} else if !(self.battle_gui.player_panel.battle_panel.is_active() || self.battle_gui.player_panel.fight_panel.is_active()) {
				self.battle_gui.player_panel.start();
			}

		}

	}

	fn pmove(&mut self) {
		if self.current_battle.pmove_queued {
			if self.battle_gui.battle_text.is_active() {
				if self.battle_gui.battle_text.can_continue {
					if !self.battle_gui.opponent_pokemon_gui.health_bar.is_moving() && self.battle_gui.battle_text.timer.is_finished() {
							
							self.current_battle.pmove_queued = false;
							self.battle_gui.battle_text.disable();

							if self.current_battle.opponent().current_hp == 0 {
								self.current_battle.faint_queued = true;
								self.current_battle.omove_queued = false;
							}
			
					} else if !self.battle_gui.battle_text.timer.is_alive() {
						self.battle_gui.battle_text.timer.spawn();
						self.battle_gui.opponent_pokemon_gui.health_bar.update_bar(self.current_battle.opponent().current_hp, self.current_battle.opponent().base.hp);
					}	
					self.battle_gui.battle_text.timer.update();
				}
			} else {
				self.current_battle.player_move();
				self.battle_gui.battle_text.enable();
				self.battle_gui.battle_text.update_text(&self.current_battle.player().pokemon.name, &self.current_battle.player_move.name);
			}
		} else if self.current_battle.faint_queued {
			self.faint_queued();
	 	} else if self.current_battle.omove_queued {
			self.omove();
		}
	}

	fn omove(&mut self) {
		if self.current_battle.omove_queued {
			if self.battle_gui.battle_text.is_active() {
				if self.battle_gui.battle_text.can_continue {
					if !self.battle_gui.player_pokemon_gui.health_bar.is_moving() && self.battle_gui.battle_text.timer.is_finished() {

							self.current_battle.omove_queued = false;
							self.battle_gui.battle_text.disable();
	
							if self.current_battle.player().current_hp == 0 {
								self.current_battle.faint_queued = true;
								self.current_battle.pmove_queued = false;
							}

					} else if !self.battle_gui.battle_text.timer.is_alive() {
						self.battle_gui.battle_text.timer.spawn();
						self.battle_gui.player_pokemon_gui.update_hp(self.current_battle.player().current_hp, self.current_battle.player().base.hp);
					}
					self.battle_gui.battle_text.timer.update();
				}
			} else {
				self.current_battle.opponent_move();
				self.battle_gui.battle_text.enable();
				self.battle_gui.battle_text.update_text(&self.current_battle.opponent().pokemon.name, &self.current_battle.opponent_move.name);
			}
		} else if self.current_battle.faint_queued {
			self.faint_queued();
		} else if self.current_battle.pmove_queued {
			self.pmove();
		}		
	}

	fn faint_queued(&mut self) {
		if self.current_battle.player().current_hp == 0 {
			if self.battle_gui.battle_text.is_active() {
				if self.battle_gui.battle_text.can_continue {
					if self.battle_gui.battle_text.timer.is_finished() {

						self.battle_gui.battle_text.disable();
						self.current_battle.faint_queued = false;
						
						if self.current_battle.player_active + 1 < self.current_battle.player_pokemon.len() {
							for i in self.current_battle.player_active..self.current_battle.player_pokemon.len() {
								if self.current_battle.player_pokemon[self.current_battle.player_active].instance.current_hp != 0 {
									self.current_battle.player_active = i;
									self.battle_gui.update_gui(&self.current_battle);
									self.battle_gui.player_pokemon_gui.update_hp(self.current_battle.player().current_hp, self.current_battle.player().base.hp);
									break;
								}
								if i == self.current_battle.player_pokemon.len() - 1 {
									info!("Player is out of pokemon!");
									self.battle_closer_manager.spawn();
								}
							}
						} else {
							self.battle_closer_manager.spawn();							
						}		

					} else if !self.battle_gui.battle_text.timer.is_alive() {
						self.battle_gui.battle_text.timer.spawn();
					}
					self.battle_gui.battle_text.timer.update();
				}
			} else {

				self.battle_gui.battle_text.enable();
				self.battle_gui.battle_text.update_faint(&self.current_battle.player().pokemon.name);

			}						
		} else {

			if self.battle_gui.battle_text.is_active() {
				if self.battle_gui.battle_text.can_continue {
					if self.battle_gui.battle_text.timer.is_finished() {

						self.current_battle.faint_queued = false;
						self.battle_gui.battle_text.disable();

						if self.current_battle.opponent_active + 1 < self.current_battle.opponent_pokemon.len() {
							self.current_battle.opponent_active += 1;
							self.battle_gui.update_gui(&self.current_battle);
						} else {
							self.battle_closer_manager.spawn();							
						}						

					} else if !self.battle_gui.battle_text.timer.is_alive() {
						self.battle_gui.battle_text.timer.spawn();
					}
					self.battle_gui.battle_text.timer.update();
				}
			} else {
				self.battle_gui.battle_text.enable();
				self.battle_gui.battle_text.update_faint(&self.current_battle.opponent().pokemon.name);

			}
		}
	}

    pub fn render(&self, ctx: &mut Context, g: &mut GlGraphics, tr: &mut TextRenderer) {

		if self.battle_screen_transition_manager.is_alive() {
			self.battle_screen_transition_manager.render(ctx, g, tr);
		} else if self.battle_opener_manager.is_alive() {
			self.battle_gui.render_background(ctx, g, self.battle_opener_manager.offset());
			self.battle_opener_manager.render_below_panel(ctx, g, tr, &self.current_battle);
			self.battle_gui.render(ctx, g, tr);
			self.battle_gui.render_panel(ctx, g, tr);
			self.battle_opener_manager.render(ctx, g, tr);
		} else if self.battle_closer_manager.is_alive() {
			self.battle_gui.render_background(ctx, g, 0);
			self.battle_gui.render_panel(ctx, g, tr);
			self.battle_closer_manager.render(ctx, g, tr);
		} else {
			self.battle_gui.render_background(ctx, g, 0);
			self.current_battle.render(ctx, g, 0, self.battle_gui.player_bounce.pokemon_offset());
			self.battle_gui.render(ctx, g, tr);
			self.battle_gui.render_panel(ctx, g, tr);
		}		
		
		//self.battle_introduction_manager.render(ctx, g, tr);
		
		/*
		if !self.player_intro.should_update() && !self.battle_opener_manager.is_alive() {
			self.current_battle.render(ctx, g, tr, self.battle_opener_manager.offset(), self.player_pokemon_y);
			self.player_pokemon_gui.render(ctx, g, tr);
		} else {
			//draw_o(ctx, g, &self.current_battle.opponent_pokemon_texture, 144 - self.battle_opener.offset as isize, 74 - self.current_battle.opponent_y_offset as isize); // fix with offset
			self.current_battle.opponent_pokemon.render(ctx, g, 144 - self.battle_opener_manager.offset() as isize, 74);
			self.player_intro.draw(ctx, g, self.battle_opener_manager.offset());
		}
		
		if !self.battle_opener_manager.is_alive() {
			
			self.opponent_pokemon_gui.render(ctx, g, tr);
		}
		self.battle_opener_manager.render_below_panel(ctx, g, tr);
		self.player_panel.render(ctx, g, tr);
		if self.battle_opener_manager.is_alive() {
			self.battle_opener_manager.render(ctx, g, tr);
		}
	*/
	}
	
	

	/*
	
	pub fn new_battle(&mut self, player_instance: OwnedPokemon, opponent_instance: PokemonInstance) {
		self.load_battle(Battle::new(player_instance, opponent_instance));
	}

	

	pub fn generate_test_battle(&mut self, pokedex: &Pokedex, context: &mut GameContext, player_data: &PlayerData) {
		let id = context.random.rand_range(0..pokedex.pokemon_list.len() as u32) as usize;
		let opponent = PokemonInstance::generate(pokedex, context, pokedex.pokemon_list.values().nth(id).expect("Could not get pokemon from position in hashmap values"), 1, 100);
		self.new_battle(player_data.party.pokemon[0].to_instance(pokedex), opponent);
	}

	pub fn generate_wild_battle(&mut self, pp: OwnedPokemon, wild_pokemon_table: &mut Box<dyn WildPokemonTable>, pokedex: &Pokedex, context: &mut GameContext) {
		self.new_battle(pp, wild_pokemon_table.generate(pokedex, context));
	}

	
	
	pub fn update(&mut self, context: &mut GameContext, player_data: &mut PlayerData) {
			if self.player_panel.intro_text.no_pause && self.player_panel.intro_text.can_continue && self.player_intro.should_update() {
				self.player_intro.update();
			} else {
				
			}
		self.current_battle.update(context);
		self.battle_gui.update(context);
			
	}

	*/	
	
	pub fn input(&mut self, context: &mut GameContext) {

		if context.fkeys[0] == 1 {
			self.finished = true; // exit shortcut
		}

		if !self.battle_screen_transition_manager.is_alive() {	
			if self.battle_opener_manager.is_alive() {
				self.battle_opener_manager.battle_introduction_manager.input(context);
			} else if self.battle_closer_manager.is_alive() {
				//self.battle_closer_manager.input(context);
			} else {
				self.battle_gui.input(context, &mut self.current_battle);
			}
		}
	}
}

