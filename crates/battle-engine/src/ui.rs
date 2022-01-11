use core::ops::Deref;
use std::rc::Rc;

use pokedex::{
    gui::{bag::BagGui, party::PartyGui},
    moves::Move,
};

use pokedex::engine::{gui::MessageBox, Context};

use crate::context::BattleGuiData;

use self::{
    background::BattleBackground,
    panels::{level::LevelUpMovePanel, BattlePanel},
    pokemon::bounce::PlayerBounce,
};

use super::transition::{
    introduction::BattleIntroductionManager, trainer::PokemonCount,
};
// use self::panels::level_up::LevelUpMovePanel;

pub mod background;
pub mod exp_bar;
pub mod panels;
pub mod pokemon;
pub mod text;

pub mod view;

pub(crate) const PANEL_Y: f32 = 113.0;

#[derive(Debug, Clone, Copy)]
pub enum BattleGuiPosition {
    Top, // index and size
    Bottom,
}

impl Default for BattleGuiPosition {
    fn default() -> Self {
        Self::Top
    }
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

pub struct BattleGui<M: Deref<Target = Move> + Clone> {
    pub background: BattleBackground,

    pub panel: BattlePanel<M>,

    pub text: MessageBox,

    pub bounce: PlayerBounce,

    // pub opener: BattleOpenerManager,
    pub introduction: BattleIntroductionManager,
    pub trainer: PokemonCount,
    pub level_up: LevelUpMovePanel<M>,
}

impl<M: Deref<Target = Move> + Clone> BattleGui<M> {
    pub fn new(
        ctx: &mut Context,
        btl: &BattleGuiData,
    ) -> Self {
        Self {
            background: BattleBackground::new(ctx, btl),

            panel: BattlePanel::new(),

            text: self::text::new(),

            bounce: PlayerBounce::new(),

            // opener: BattleOpenerManager::new(ctx, btl),
            introduction: BattleIntroductionManager::new(btl),
            trainer: PokemonCount::new(btl),
            level_up: LevelUpMovePanel::new(),
        }
    }

    pub fn reset(&mut self) {
        self.bounce.reset();
    }
}
