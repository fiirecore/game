use opengl_graphics::GlGraphics;
use piston_window::Context;

use crate::engine::game_context::GameContext;
use crate::engine::text::TextRenderer;
use crate::entity::entity::Entity;
use crate::battle::battle::Battle;
use crate::gui::gui::GuiComponent;
use crate::util::traits::Loadable;

use super::battle_background::BattleBackground;
use super::battle_text::BattleText;
use super::panels::player_panel::PlayerPanel;
use super::player_bounce::PlayerBounce;
use super::pokemon_gui::OpponentPokemonGui;
use super::pokemon_gui::PlayerPokemonGui;
use super::pokemon_gui::PokemonGui;

pub struct BattleGui {

	alive: bool,

	pub battle_background: BattleBackground,

	pub player_pokemon_gui: PlayerPokemonGui,
	pub opponent_pokemon_gui: OpponentPokemonGui,
	pub player_panel: PlayerPanel,

	pub battle_text: BattleText,

	pub player_bounce: PlayerBounce,

}

impl BattleGui {

	pub fn new() -> Self {

		Self {

			alive: false,

			battle_background: BattleBackground::new(),

			player_panel: PlayerPanel::new(0, 113),
			player_pokemon_gui: PlayerPokemonGui::new(127, 76),
			opponent_pokemon_gui: OpponentPokemonGui::new(14, 18),

			battle_text: BattleText::new(0, 113),

			player_bounce: PlayerBounce::new(),

		}

	}

	pub fn on_battle_start(&mut self, battle: &Battle) {
	
		self.update_gui(battle);

	}

	pub fn update_gui(&mut self, battle: &Battle) {

		self.player_panel.update_text(battle.player());

		// update health
		
		self.opponent_pokemon_gui.update_gui(battle);
		self.player_pokemon_gui.update_gui(battle);
	}

	pub fn update(&mut self, context: &mut GameContext) {
		self.player_bounce.update(&mut self.player_pokemon_gui);
		self.player_panel.update(context);
		self.player_pokemon_gui.update();
		self.opponent_pokemon_gui.update();
		self.battle_text.update();
	}

	pub fn input(&mut self, context: &mut GameContext, battle: &mut Battle) {
		self.player_panel.input(context, battle);
	}

	pub fn render_background(&self, ctx: &mut Context, g: &mut GlGraphics, offset: u16) {
		self.battle_background.render(ctx, g, offset);
	}

	pub fn render_panel(&self, ctx: &mut Context, g: &mut GlGraphics, tr: &mut TextRenderer) {
		self.player_panel.render(ctx, g, tr);
		self.battle_text.render(ctx, g, tr);
	}

	pub fn render(&self, ctx: &mut Context, g: &mut GlGraphics, tr: &mut TextRenderer) {
		self.opponent_pokemon_gui.render(ctx, g, tr);
		self.player_pokemon_gui.render(ctx, g, tr);
	}

}

impl Loadable for BattleGui {

	fn load(&mut self) {
		self.battle_background.load();
		self.player_panel.load();
		self.player_pokemon_gui.panel.load();
		self.opponent_pokemon_gui.panel.load();
	}
	
}

impl Entity for BattleGui {

    fn spawn(&mut self) {
		self.alive = true;
        self.player_panel.enable();
    }

    fn despawn(&mut self) {
		self.alive = false;
		self.player_panel.disable();
		self.player_pokemon_gui.despawn();
		self.opponent_pokemon_gui.despawn();
		self.battle_text.disable();
    }

    fn is_alive(&self) -> bool {
        self.alive
    }
}

/*

pub trait BattleGuiComponent {
	
	//fn update_gui(&mut self, battle_manager: &mut BattleManager);

	// fn on health change

	// fn on ...
	
}

*/

pub trait BattleGuiButton {

	fn on_use(&mut self, context: &mut GameContext, battle: &mut Battle);

}

pub trait BattleActivatable {

	fn focus(&mut self);

	fn unfocus(&mut self);

	fn in_focus(&mut self) -> bool;

	fn input(&mut self, context: &mut GameContext, battle: &mut Battle, pp_gui: &mut PlayerPokemonGui, op_gui: &mut OpponentPokemonGui);

	fn next(&self) -> u8;

}

