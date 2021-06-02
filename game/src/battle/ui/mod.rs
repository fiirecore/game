use background::BattleBackground;
use pokemon::bounce::PlayerBounce;
use crate::{
	gui::{
		DynamicText,
		party::PartyGui,
		pokemon::PokemonDisplay,
	},
	graphics::position,
	tetra::{
		Context,
		graphics::DrawParams
	},
};

use super::pokemon::{BattleParty, BattlePartyPlayerView};
// use self::panels::level_up::LevelUpMovePanel;

pub mod background;
pub mod text;
pub mod pokemon;
pub mod panels;
pub mod exp_bar;

pub mod transitions;

pub(crate) const PANEL_ORIGIN: DrawParams = position(0.0f32, 113.0);

#[derive(Debug, Clone, Copy)]
pub enum BattleGuiPosition {
	Top, // index and size
	Bottom,
}

#[derive(Debug, Clone, Copy)]
pub struct BattleGuiPositionIndex {
	pub position: BattleGuiPosition,
	pub index: u8,
	pub size: u8,
}

impl BattleGuiPositionIndex {

	pub const fn new(position: BattleGuiPosition, index: u8, size: u8) -> Self {
		Self {
		    position,
		    index,
		    size,
		}
	}

}

pub struct BattleGui {

	pub background: BattleBackground,

	pub text: DynamicText,

	pub bounce: PlayerBounce,

	// pub level_up: LevelUpMovePanel,

}

impl BattleGui {

	pub fn new(ctx: &mut Context) -> Self {

		Self {

			background: BattleBackground::new(ctx),

			// panel: BattlePanel::new(ctx),

			text: text::new(),

			bounce: PlayerBounce::new(),

			// level_up: LevelUpMovePanel::new(Vec2::new(0.0, 113.0)),

		}

	}

	#[inline]
	pub fn draw_panel(&self, ctx: &mut Context) {
        self.background.panel.draw(ctx, PANEL_ORIGIN)
	}

	pub fn reset(&mut self) {
		self.bounce.reset();
	}

}

pub fn battle_party_player_view_gui(gui: &PartyGui, party: &BattlePartyPlayerView, exitable: bool) {
	gui.spawn(party.collect_cloned().into_iter().map(|instance| PokemonDisplay::new(std::borrow::Cow::Owned(instance))).collect(), Some(false), exitable);
}

pub fn battle_party_gui(gui: &PartyGui, party: &BattleParty, exitable: bool) {
    gui.spawn(party.collect_cloned().into_iter().map(|instance| PokemonDisplay::new(std::borrow::Cow::Owned(instance))).collect(), Some(false), exitable);
}