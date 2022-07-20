use core::ops::Deref;

use pokengine::{
    engine::{
        egui,
        graphics::{Draw, DrawImages},
    },
    pokedex::{
        item::Item,
        moves::Move,
        pokemon::{Pokemon, PokemonTexture},
    },
    PokedexClientData,
};

use crate::context::BattleGuiData;

use self::{
    background::BattleBackground,
    panels::{level::LevelUpMovePanel, BattleAction, BattlePanel},
    pokemon::{bounce::PlayerBounce, PokemonRenderer},
    text::BattleText,
    trainer::PokemonCount,
};

// use self::panels::level_up::LevelUpMovePanel;

mod background;
mod exp_bar;
pub mod panels;
mod pokemon;
pub mod text;
mod trainer;

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

impl From<PokemonTexture> for BattleGuiPosition {
    fn from(texture: PokemonTexture) -> Self {
        match texture {
            PokemonTexture::Front => BattleGuiPosition::Top,
            PokemonTexture::Back => BattleGuiPosition::Bottom,
            PokemonTexture::Icon => panic!("Cannot convert icon into position"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct BattleGuiPositionIndex {
    pub position: BattleGuiPosition,
    pub index: usize,
    pub size: usize,
}

pub struct BattleGui<
    ID,
    D: Deref<Target = PokedexClientData> + Clone,
    M: Deref<Target = Move> + Clone,
> {
    pub background: BattleBackground,

    pub panel: BattlePanel<D>,
    pub actions: Vec<BattleAction<ID>>,

    pub text: BattleText,

    pub bounce: PlayerBounce,
    pub pokemon: PokemonRenderer<D>,

    pub trainer: PokemonCount,
    pub level_up: LevelUpMovePanel<M>,
}

impl<ID, D: Deref<Target = PokedexClientData> + Clone, M: Deref<Target = Move> + Clone>
    BattleGui<ID, D, M>
{
    pub fn new(data: D, btl: &BattleGuiData) -> Self {
        Self {
            background: BattleBackground::new(btl),

            panel: BattlePanel::new(data.clone()),
            actions: Vec::new(),

            text: BattleText::default(),

            bounce: PlayerBounce::new(),
            pokemon: PokemonRenderer::new(data, btl),

            trainer: PokemonCount::new(btl),
            level_up: LevelUpMovePanel::new(),
        }
    }

    pub fn draw_panel(&self, draw: &mut Draw) {
        draw.image(&self.background.panel).position(0.0, PANEL_Y);
        // self.background
        //     .panel
        //     .draw(ctx, 0.0, PANEL_Y, Default::default());
    }

    pub fn reset(&mut self) {
        self.bounce.reset();
        self.panel.reset();
        self.actions.clear();
        self.pokemon.reset();
        self.trainer.reset();
    }
}

pub struct PokemonStatus;

impl PokemonStatus {
    pub fn status<
        ID: std::fmt::Display + std::hash::Hash,
        P: Deref<Target = Pokemon> + Clone,
        M: Deref<Target = Move> + Clone,
        I: Deref<Target = Item> + Clone,
        POK: crate::view::GuiPokemonView<P, M, I>,
    >(
        egui: &egui::Context,
        hashnum: usize,
        pokemon: &POK,
    ) {
        egui::Window::new(format!("Status {}", hashnum))
            .title_bar(false)
            .show(egui, |ui| {
                pokemon::PokemonStatusGui::ui(ui, hashnum, pokemon)
            });
    }
}
