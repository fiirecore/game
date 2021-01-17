use log::info;
use opengl_graphics::GlGraphics;
use piston_window::Context;
use crate::entity::Ticking;
use crate::game::pokedex::pokedex::Pokedex;
use crate::gui::battle::battle_gui::BattleGui;
use crate::util::context::battle_context::BattleData;
use crate::util::traits::Completable;
use crate::util::traits::Loadable;
use crate::io::data::player_data::PlayerData;
use crate::util::text_renderer::TextRenderer;

use crate::util::context::GameContext;

use super::battle::Battle;
// use super::battle_context::BattleData;

use crate::entity::Entity;

use super::transitions::managers::battle_closer_manager::BattleCloserManager;
use super::transitions::managers::battle_screen_transition_manager::BattleScreenTransitionManager;
use super::transitions::managers::battle_opener_manager::BattleOpenerManager;

pub struct BattleManager {	
	
	pub current_battle: Battle,
	pub battle_data: BattleData,
	
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
			battle_data: BattleData::default(),

			battle_gui: BattleGui::new(),

			finished: false,

		}
		
	}

	pub fn world_active(&self) -> bool {
		return self.battle_screen_transition_manager.is_alive() || self.battle_closer_manager.world_active();
	}

}

impl Loadable for BattleManager {

	fn load(&mut self) {

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
		self.battle_data = context.battle_data.take().unwrap();
		self.create_battle(player_data, pokedex);
		self.battle_gui.despawn();
		self.battle_gui.spawn();		
		self.battle_screen_transition_manager.spawn();
		self.battle_screen_transition_manager.on_start(context, self.battle_data.battle_type);
	}

	pub fn create_battle(&mut self, player_data: &PlayerData, pokedex: &Pokedex) {
		
		let battle = Battle::new(pokedex, &player_data.party, &self.battle_data.party);
		info!("Loading Battle: {}", battle);
		self.current_battle = battle;
		self.current_battle.load();
		self.battle_gui.on_battle_start(&self.current_battle);
	}

	pub fn update(&mut self, context: &mut GameContext, player_data: &mut PlayerData) {
		
		if self.battle_screen_transition_manager.is_alive() {
			if self.battle_screen_transition_manager.is_finished() {
				self.battle_screen_transition_manager.despawn();
				self.battle_opener_manager.spawn_type(self.battle_data.battle_type);
				self.battle_opener_manager.on_start(context);
				self.battle_opener_manager.battle_introduction_manager.setup_text(&self.current_battle, self.battle_data.trainer_data.as_ref());
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
				// self.battle_closer_manager.update_player(player_data);
				self.battle_closer_manager.despawn();
				self.finished = true;
			} else {
				self.battle_closer_manager.update(context);
			}
		} else /*if !self.current_battle.is_finished()*/ {
			self.current_battle.update(context, &mut self.battle_gui, &mut self.battle_closer_manager);
			self.battle_gui.update(context);
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
			if !self.world_active() {
				self.battle_gui.render_background(ctx, g, 0);
				self.current_battle.render(ctx, g, 0, self.battle_gui.player_bounce.pokemon_offset());
				self.battle_gui.render(ctx, g, tr);
				self.battle_gui.render_panel(ctx, g, tr);
			}
			self.battle_closer_manager.render(ctx, g, tr);
		} else {
			self.battle_gui.render_background(ctx, g, 0);
			self.current_battle.render(ctx, g, 0, self.battle_gui.player_bounce.pokemon_offset());
			self.battle_gui.render(ctx, g, tr);
			self.battle_gui.render_panel(ctx, g, tr);
		}
	}
	
	pub fn input(&mut self, context: &mut GameContext) {

		if context.fkeys[0] == 1 {
			self.battle_closer_manager.spawn() // exit shortcut
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

