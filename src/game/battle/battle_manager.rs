use log::info;
use opengl_graphics::GlGraphics;
use piston_window::Context;
use crate::entity::entity::Ticking;
use crate::util::traits::Loadable;
use crate::{engine::engine::Texture, game::world::pokemon::wild_pokemon_table::WildPokemonTable, io::data::player_data::PlayerData};
use crate::engine::text::TextRenderer;
use crate::gui::gui::GuiComponent;

use crate::engine::game_context::GameContext;

use crate::game::battle::battle::Battle;
use crate::game::pokedex::pokemon::pokemon_instance::PokemonInstance;
use crate::game::pokedex::pokemon::pokemon_owned::OwnedPokemon;
use crate::game::pokedex::pokedex::Pokedex;
use crate::gui::battle::pokemon_gui::{PokemonGui, PlayerPokemonGui, OpponentPokemonGui};
use crate::gui::battle::panels::player_panel::PlayerPanel;

use crate::util::file_util::asset_as_pathbuf;
use crate::util::render_util::draw_o;
use crate::util::texture_util::texture_from_path;

use crate::entity::entity::Entity;

use super::transitions::openers::player_intro::PlayerBattleIntro;
use super::transitions::managers::battle_opener_manager::BattleOpenerManager;
use super::transitions::traits::battle_transition_manager::BattleTransitionManager;

pub struct BattleManager {	
	
	pub current_battle: Battle,
	
	battle_opener_manager: BattleOpenerManager,

	background_texture: Option<Texture>,
	pad_texture: Option<Texture>,
	
//	grass_texture: Texture,

	pub player_pokemon_gui: PlayerPokemonGui,
	pub opponent_pokemon_gui: OpponentPokemonGui,
	pub player_panel: PlayerPanel,
	player_intro: PlayerBattleIntro,

	player_bounce_counter: u8,
	player_pokemon_y: u8,
	player_up: bool,
	player_gui_up: bool,

	pub finished: bool,
	
}

impl BattleManager {
	
	pub fn new() -> BattleManager {
		
		BattleManager {

			battle_opener_manager: BattleOpenerManager::new(),
		
			current_battle: Battle::empty(),
			
			background_texture: None,
			pad_texture: None,

			player_panel: PlayerPanel::new(0, 113),
			
			player_pokemon_gui: PlayerPokemonGui::new(127, 76),

			opponent_pokemon_gui: OpponentPokemonGui::new(14, 18),

			player_intro: PlayerBattleIntro::new(),

			player_bounce_counter: 0,
			player_pokemon_y: 0,
			player_up: false,
			player_gui_up: true,

			finished: false,

		}
		
	}
	
	pub fn load(&mut self) {

		self.background_texture = Some(texture_from_path(asset_as_pathbuf("gui/battle/background.png")));
		self.pad_texture = Some(texture_from_path(asset_as_pathbuf("gui/battle/grass_pad.png")));
		self.player_intro.load();

		self.battle_opener_manager.load_openers();
		self.player_panel.load();
		self.player_pokemon_gui.panel.load();
		self.opponent_pokemon_gui.panel.load();
//		self.battle_gui.update_gui(&self.current_battle);
	}
	
	pub fn on_start(&mut self, context: &mut GameContext) { // add battle type parameter
		self.finished = false;
		self.despawn_gui();
		self.player_panel.enable();
		self.battle_opener_manager.spawn();
		self.battle_opener_manager.on_start(context);
		self.player_intro.on_start(context);
	}
	
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
	
	pub fn load_battle(&mut self, battle: Battle) {
		info!("Loading Battle: {}", battle);
		self.current_battle = battle;
		self.update_gui();
		self.load_current_battle();
		let mut opponent_string = String::from("Wild ");
		opponent_string.push_str(self.current_battle.opponent_pokemon.instance.as_ref().unwrap().pokemon.name.to_uppercase().as_str());
		opponent_string.push_str(" appeared!");
		
		self.player_panel.intro_text.update_text(opponent_string);
		self.player_panel.update_text(&self.current_battle.player_pokemon.instance.as_ref().unwrap());
	}
	
	pub fn load_current_battle(&mut self) {
		self.current_battle.load();
	}

	pub fn despawn_gui(&mut self) {
		self.battle_opener_manager.despawn();
		self.player_panel.disable();
		self.player_pokemon_gui.panel.disable();
		self.opponent_pokemon_gui.panel.disable();
	}
	
	pub fn update(&mut self, context: &mut GameContext) {
		if self.battle_opener_manager.is_alive() {
			if self.battle_opener_manager.is_finished() {
				self.battle_opener_manager.despawn();
				self.player_panel.intro_text.enable();				
			} else {
				self.battle_opener_manager.update(context);
			}
		} else {
			if self.player_panel.intro_text.no_pause && self.player_panel.intro_text.can_continue && self.player_intro.should_update() {
				self.player_intro.update();
			} else {
				self.player_bounce_counter = (self.player_bounce_counter + 1) % 20;
				if self.player_bounce_counter == 0 {
					self.player_up = !self.player_up;
					if self.player_gui_up {
						self.player_pokemon_y = 0;
					} else {
						self.player_pokemon_y = 1;
					}
				}
				if self.player_bounce_counter == 10 {
					self.player_gui_up = !self.player_gui_up;
					if self.player_gui_up {
						self.player_pokemon_gui.offset_position(0, 0);
					} else {
						self.player_pokemon_gui.offset_position(0, -1);
					}
				}
			}
		}
		self.current_battle.update(context);
		self.player_panel.update(context);
		self.player_pokemon_gui.panel.update(context);
		self.opponent_pokemon_gui.panel.update(context);
			
	}
	
	pub fn update_gui(&mut self) {
		
		self.player_pokemon_gui.name.text = self.current_battle.player_pokemon.instance.as_ref().unwrap().pokemon.name.to_uppercase();
		self.opponent_pokemon_gui.name.text = self.current_battle.opponent_pokemon.instance.as_ref().unwrap().pokemon.name.to_uppercase();
		
		let pl = self.current_battle.player_pokemon.instance.as_ref().unwrap().level;
		let mut plstr = String::from("Lv");
		plstr.push_str(&pl.to_string());
		self.player_pokemon_gui.level.text = plstr;
		let ol = self.current_battle.opponent_pokemon.instance.as_ref().unwrap().level;
		let mut olstr = String::from("Lv");
		olstr.push_str(&ol.to_string());
		self.opponent_pokemon_gui.level.text = olstr;
		
		self.update_health(); // move to run whenever damage is taken
		
	}
	
	pub fn update_health(&mut self) {
		
		let mut ch = self.current_battle.player_pokemon.current_hp.to_string();
		ch.push('/');
		let hp = self.current_battle.player_pokemon.hp;
		ch.push_str(&hp.to_string());
		
		self.player_pokemon_gui.health_text.text = ch;
		
		self.opponent_pokemon_gui.update_gui(&self.current_battle);
		self.player_pokemon_gui.update_gui(&self.current_battle);
		
	}
	
	pub fn render(&mut self, ctx: &mut Context, g: &mut GlGraphics, tr: &mut TextRenderer) {
		draw_o(ctx, g, &self.background_texture, 0, 1);
		draw_o(ctx, g, &self.pad_texture, 113 - self.battle_opener_manager.offset() as isize, 50);
		draw_o(ctx, g, &self.pad_texture, 0 + self.battle_opener_manager.offset() as isize, 103);

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
			
	}
	
	pub fn input(&mut self, context: &mut GameContext){
		if !self.battle_opener_manager.is_alive() {
			self.player_panel.input(context, &mut self.current_battle, &mut self.player_pokemon_gui, &mut self.opponent_pokemon_gui);
			if self.current_battle.opponent_pokemon.faint || self.current_battle.player_pokemon.faint || self.current_battle.finished {
				self.finished = true;
				self.despawn_gui();
			}
		}
	}	
}

