use crate::{
    graphics::position,
    gui::{party::PartyGui, pokemon::PokemonDisplay, TextDisplay},
    pokedex::battle::party::knowable::BattlePartyKnown,
    tetra::{graphics::DrawParams, Context},
};

use self::{background::BattleBackground, panels::BattlePanel, pokemon::bounce::PlayerBounce};

use crate::battle::player::BattlePlayer;

use super::transition::{
    introduction::BattleIntroductionManager, opener::BattleOpenerManager,
    trainer::BattleTrainerPartyIntro,
};
// use self::panels::level_up::LevelUpMovePanel;

pub mod background;
pub mod exp_bar;
pub mod panels;
pub mod pokemon;
pub mod text;

pub mod view;

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

    pub panel: BattlePanel,

    pub text: TextDisplay,

    pub bounce: PlayerBounce,

    pub opener: BattleOpenerManager,
    pub introduction: BattleIntroductionManager,
    pub trainer: BattleTrainerPartyIntro,
    // pub level_up: LevelUpMovePanel,
}

impl BattleGui {
    pub fn new(ctx: &mut Context) -> Self {
        Self {
            background: BattleBackground::new(ctx),

            panel: BattlePanel::new(ctx),

            text: self::text::new(),

            bounce: PlayerBounce::new(),

            opener: BattleOpenerManager::new(ctx),
            introduction: BattleIntroductionManager::new(ctx),
			trainer: BattleTrainerPartyIntro::new(ctx),
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

pub fn battle_party_known_gui<
    ID: Sized + Copy + core::fmt::Debug + core::fmt::Display + Eq + Ord,
>(
    gui: &PartyGui,
    party: &BattlePartyKnown<ID>,
    exitable: bool,
) {
    gui.spawn(
        party
            .pokemon
            .iter()
            .cloned()
            .map(|instance| PokemonDisplay::new(std::borrow::Cow::Owned(instance)))
            .collect(),
        Some(false),
        exitable,
    );
}

pub fn battle_party_gui<ID: Sized + Copy + core::fmt::Debug + core::fmt::Display + Eq + Ord>(
    gui: &PartyGui,
    player: &BattlePlayer<ID>,
    exitable: bool,
) {
    gui.spawn(
        player
            .party
            .cloned()
            .into_iter()
            .map(|instance| PokemonDisplay::new(std::borrow::Cow::Owned(instance)))
            .collect(),
        Some(false),
        exitable,
    );
}
